# shikimori-api

`shikimori-api` — подробный асинхронный Rust-клиент официального **Shikimori REST API v1/v2**. Библиотека охватывает все REST-маршруты, опубликованные в API v1 и v2; **GraphQL и OAuth authorization/refresh flow намеренно не реализуются**. Для защищённых маршрутов пользователь передаёт ранее полученный access token.

> Официальная документация помечает v1/v2 как устаревшие и рекомендует GraphQL при наличии подходящего запроса. Этот crate создан именно для проектов, которым нужно полное и типобезопасное взаимодействие с опубликованным REST API. [1] [2]

## Возможности

| Область | Реализация |
| --- | --- |
| REST coverage | Все официальные v1/v2 endpoints, кроме GraphQL и OAuth flow; подробности в [матрице покрытия](docs/endpoint-matrix.md). |
| Transport | Async Hyper HTTPS через Rustls/WebPKI roots поверх Tokio. |
| JSON | Только `simd_json` и `serde` для body и responses; `serde_json` не используется API-кодом. |
| Rate limiting | Два общих `governor` limiter по умолчанию: **5 rps + 90 rpm**, как требует Shikimori. [1] |
| Inputs | Отдельные typed query/body structs и enum для filter/status/policy values; query и multipart кодируются без URL/form crates. |
| Safety | Токены не добавляются в error text; изменяющие методы не имеют автоматических повторов. |
| Compatibility | API output enum-like fields часто сохраняются строками, чтобы новые значения сервера не ломали десериализацию. |

## Установка

```toml
[dependencies]
shikimori-api = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Минимальная заявленная версия Rust — **1.75**. При сборке самого проекта lockfile использует совместимые версии прямых зависимостей `hyper`, `hyper-rustls`, `tokio`, `simd-json`, `serde` и `governor`.

## Быстрый старт: публичный каталог

```rust,no_run
use shikimori_api::{ClientConfig, ListAnimesQuery, ShikimoriClient};

#[tokio::main]
async fn main() -> Result<(), shikimori_api::Error> {
    let client = ShikimoriClient::new(
        ClientConfig::builder("my-shikimori-app/0.1")
            .build()?,
    );

    let results = client.list_animes(ListAnimesQuery {
        search: Some("Cowboy Bebop".into()),
        limit: Some(10),
        ..Default::default()
    }).await?;

    for anime in results {
        println!("{} — {}", anime.id, anime.name);
    }
    Ok(())
}
```

`User-Agent` не является опциональным: Shikimori требует передавать имя приложения и предупреждает не маскировать API client под браузер. [1]

## Защищённая v2-операция

OAuth flow находится за пределами crate. Когда token уже получен вне библиотеки, передайте его в configuration. Скоуп операции всё равно проверяет сервер.

```rust,no_run
use shikimori_api::{ClientConfig, NewUserRate, RateTargetType, ShikimoriClient, UserRateStatus, CreateUserRateRequest};

# async fn example() -> Result<(), shikimori_api::Error> {
let client = ShikimoriClient::new(
    ClientConfig::builder("my-shikimori-app/0.1")
        .access_token("access-token-from-your-oauth-client")
        .build()?,
);

let rate = client.create_user_rate_v2(CreateUserRateRequest {
    user_rate: NewUserRate {
        user_id: 123,
        target_id: 1,
        target_type: RateTargetType::Anime,
        status: Some(UserRateStatus::Watching),
        score: None,
        chapters: None,
        episodes: Some(1),
        volumes: None,
        rewatches: None,
        text: Some("Начал смотреть".into()),
    },
}).await?;

assert_eq!(rate.target_id, Some(1));
# Ok(())
# }
```

## Пагинация и списочные фильтры

`ListAnimesQuery`, `ListMangasQuery` и `ListRanobeQuery` поддерживают CSV filtering, включая документированный subtraction mode через `ListValue::Exclude`.

```rust
use shikimori_api::{AnimeKind, ListAnimesQuery, ListValue};

let query = ListAnimesQuery {
    kind: Some(vec![
        ListValue::Include(AnimeKind::Tv),
        ListValue::Exclude(AnimeKind::Movie),
    ]),
    season: Some(vec!["2024".into()]),
    ..Default::default()
};
```

Shikimori указывает, что пагинированный ответ часто содержит `N+1` результатов при наличии следующей страницы. Клиент возвращает точный серверный массив и не пытается обрезать его. [1]

## Ошибки

```rust,no_run
use shikimori_api::{Error, ShikimoriClient};

# async fn example(client: &ShikimoriClient) {
match client.anime(1).await {
    Ok(anime) => println!("{}", anime.name),
    Err(Error::Api(error)) if error.status == 429 => {
        eprintln!("rate limited; retry-after: {:?}", error.retry_after_seconds);
    }
    Err(error) => eprintln!("request failed: {error}"),
}
# }
```

`ApiError` сохраняет HTTP code, нормализованный массив `errors` (если он есть), ответ body и числовой `Retry-After`. Клиент намеренно не делает implicit retry, особенно для POST/PATCH/PUT/DELETE.

## Документация и покрытие

| Документ | Содержимое |
| --- | --- |
| [API reference](docs/api-reference.md) | Конфигурация, ошибки, модели, input DTO, enum, use cases и особенности спецификации. |
| [Endpoint matrix](docs/endpoint-matrix.md) | Каждый URI, HTTP-метод, публичный Rust method, input, auth scope, output type и статус реализации. |
| [`src/params.rs`](src/params.rs) | Полные typed request/query структуры и JSON wire names. |
| [`src/types.rs`](src/types.rs) | Response structures и вложенные API models. |

## Ограничения

GraphQL и получение, refresh либо revoke OAuth token не реализуются. Deprecated v1 endpoints остаются в crate для полного REST coverage; там, где есть официальный replacement, метод помечен `#[deprecated]`. V2 user-rate methods следует предпочитать legacy v1 CRUD.

## Разработка

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

Opt-in live smoke tests, когда добавлены, должны запускаться только против public GET routes и не выполняют защищённых/изменяющих действий.

## References

[1]: https://shikimori.io/api/doc/1.0 "Shikimori API v1"
[2]: https://shikimori.io/api/doc/2.0 "Shikimori API v2"
