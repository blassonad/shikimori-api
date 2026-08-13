# `shikimori-api`

`shikimori-api` — типобезопасный асинхронный Rust-клиент официального **Shikimori REST API v1/v2**. Crate предоставляет отдельно типизированные query DTO, JSON request DTO и response-модели для каждого опубликованного REST-маршрута двух версий API.

> **Границы поддержки.** Crate намеренно не реализует GraphQL и OAuth authorization, refresh или revoke flow. Для защищённых REST-вызовов передайте уже выданный access token в [`ClientConfig::builder`]. Официальная документация рекомендует GraphQL для новых интеграций, однако REST v1/v2 остаётся доступным и полностью покрывается данным crate. [1] [2]

## Быстрый старт

Ниже приведён безопасный публичный GET-пример. Он компилируется как doctest, но не выполняется автоматически.

```rust,no_run
use shikimori_api::{ClientConfig, ListAnimesQuery, ShikimoriClient};

# async fn example() -> Result<(), shikimori_api::Error> {
let client = ShikimoriClient::new(
    ClientConfig::builder("my-shikimori-app/0.1")
        .build()?,
);

let animes = client
    .list_animes(ListAnimesQuery {
        search: Some("Cowboy Bebop".into()),
        limit: Some(10),
        ..Default::default()
    })
    .await?;

for anime in animes {
    println!("#{}: {}", anime.id, anime.name);
}
# Ok(())
# }
```

## Конфигурация и авторизация

[`ClientConfig`] строится через [`ClientConfig::builder`]. Builder требует осмысленный непустой `User-Agent`; Shikimori требует передавать имя OAuth2 application и прямо запрещает имитировать браузер. [1]

```rust,no_run
use shikimori_api::{ClientConfig, ShikimoriClient};

# fn example() -> Result<(), shikimori_api::Error> {
let client = ShikimoriClient::new(
    ClientConfig::builder("my-shikimori-app/0.1")
        // Получение token — ответственность вашего OAuth2-клиента.
        .access_token("access-token-obtained-elsewhere")
        .build()?,
);
assert!(client.has_access_token());
# Ok(())
# }
```

Bearer token отправляется в `Authorization` только при его явной настройке. Token не включается в [`Error`] и [`ApiError`]. Требуемый scope зависит от маршрута; Rust-клиент не подменяет серверную проверку прав.

## Ограничение частоты и параллелизм

По умолчанию [`ShikimoriClient`] одновременно применяет два общего для всех clones limiter: **5 запросов в секунду** и **90 запросов в минуту**, как определено Shikimori. [1] Перед каждым HTTP-вызовом клиент ждёт разрешения обоих limiter. Это означает, что `client.clone()` безопасен для конкурентных задач Tokio: clones не обходят общий лимит.

Клиент не выполняет скрытых повторов. Такой выбор предотвращает неявное дублирование POST/PATCH/PUT/DELETE. При 429 [`ApiError::retry_after_seconds`] хранит числовой `Retry-After`, если его вернул сервер; вызывающий код сам выбирает безопасную retry policy.

## Модули и маршруты

| Раздел | Что предоставляет |
| --- | --- |
| [`ShikimoriClient`] | Единственный async клиент; все endpoint methods реализованы как inherent methods этого типа. |
| [`v1`] | Справочник REST API v1: каталог, клубы, комментарии, темы, users, legacy user rates, styles, videos и другие ресурсы. |
| [`v2`] | Справочник REST API v2: ignore actions, moderation requests, episode notifications и актуальный CRUD user rates. |
| Input structures | Публичные query, JSON body и enum-input структуры re-exported с crate root. |
| [`types`] | Публичные модели успешных response bodies и вложенных ресурсов. |
| [`Error`] / [`ApiError`] | Типизированные transport, codec и HTTP API ошибки. |

Полная таблица `HTTP method → URI → Rust method → input → response → scope` находится в [endpoint matrix](https://github.com/blassonad/shikimori-api/blob/feat/complete-rest-client/docs/endpoint-matrix.md). Расширенные пояснения полей и wire contract находятся в [API reference](https://github.com/blassonad/shikimori-api/blob/feat/complete-rest-client/docs/api-reference.md).

## Input и wire-format

Каждый query/body имеет отдельный Rust type вместо универсального JSON map. Например, [`ListAnimesQuery`] поддерживает все документированные catalog filters, а [`ListValue`] выражает включение и исключение CSV-фильтра (`tv,!movie`) без ручной строковой конкатенации.

```rust
use shikimori_api::{AnimeKind, ListAnimesQuery, ListValue};

let query = ListAnimesQuery {
    kind: Some(vec![
        ListValue::Include(AnimeKind::Tv),
        ListValue::Exclude(AnimeKind::Movie),
    ]),
    ..Default::default()
};
```

JSON request bodies сериализуются `simd_json` + Serde. Legacy nested wrappers сохраняются: `CreateCommentRequest` формирует `{ "comment": { ... } }`, `CreateUserRateRequest` — `{ "user_rate": { ... } }`. Query и `multipart/form-data` для user-image upload кодируются crate без дополнительной URL/form dependency.

## Модели и совместимость

В публичных моделях `Anime`, `Manga`, `Character`, `Person`, `Topic`, `Comment`, `UserRate` и других стабильные поля типизированы. Там, где REST server может добавлять новые строковые значения, output fields сознательно сохраняются как `String`/`Option<String>`: это не превращает добавленный сервером enum value в ошибку десериализации.

Даты остаются строками wire-format, а [`types::FlexibleDate`] допускает legacy неоднородные person-date значения. Такой подход соответствует примерам официальной документации, где person date может быть объектом `{}` вместо ISO-строки. [3]

## Ошибки

Все async методы возвращают [`Result<T>`].

```rust,no_run
use shikimori_api::{Error, ShikimoriClient};

# async fn example(client: &ShikimoriClient) {
match client.anime(1).await {
    Ok(anime) => println!("{}", anime.anime.name),
    Err(Error::Api(api)) if api.status == 429 => {
        eprintln!("Подождите {:?} секунд", api.retry_after_seconds);
    }
    Err(error) => eprintln!("Запрос не выполнен: {error}"),
}
# }
```

| Вариант | Значение |
| --- | --- |
| [`Error::Configuration`] | Некорректный base URL, пустой token либо User-Agent. |
| [`Error::Transport`] / [`Error::Body`] | Ошибка TLS/соединения или чтения HTTP body. |
| [`Error::Json`] | Response/body не соответствует ожидаемой Serde model. |
| [`Error::Api`] | Не-2xx response Shikimori; содержит HTTP status, parsed `errors`, body и `Retry-After`. |

## Пагинация

Shikimori предупреждает, что запрос `N` элементов пагинируемого API в большинстве случаев может вернуть `N+1` при существовании следующей страницы. Crate возвращает серверный массив как есть и не обрезает последний элемент. [1] Используйте endpoint-specific query input (`PageLimitQuery`, `PageLimit30Query`, `ListUsersQuery`, `ListUserRatesV2Query` и т. п.) и интерпретируйте результат согласно этой семантике.

## Deprecated v1 API

Полнота покрытия включает deprecated v1 routes. Там, где у Shikimori есть replacement, методы несут `#[deprecated]` и rustdoc note: legacy ignores заменяются v2 ignore actions, а legacy v1 user-rate CRUD — соответствующими методами v2. Аналогично, `search_animes` и `search_mangas` сохранены для совместимости, но для новых вызовов используйте list routes с полем `search`.

## Проверка документации

```text
cargo doc --no-deps --all-features
cargo test --doc --all-features
```

Все сетевые примеры помечены `no_run`: Cargo проверяет их актуальность и компилируемость, не создавая/не меняя данные Shikimori.

## References

[1]: https://shikimori.io/api/doc/1.0 "Shikimori API v1"
[2]: https://shikimori.io/api/doc/2.0 "Shikimori API v2"
[3]: https://shikimori.io/api/doc/1.0/people.html "Shikimori API v1 — People"
