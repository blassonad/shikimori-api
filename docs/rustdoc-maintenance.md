# Сопровождение Rustdoc

## Назначение

`shikimori-api` рассматривает Rustdoc как **каноническую документацию публичного Cargo API**. `README.md` остаётся точкой входа, но contract типов, enum, методов и response-моделей поддерживается рядом с исходным кодом и отображается через `cargo doc`.

> Любой новый публичный item обязан быть понятен пользователю, который видит только страницу docs.rs и не открывает исходники проекта.

## Обязательный Rustdoc checklist

| Публичный item | Минимальное содержание документации |
| --- | --- |
| Crate/module | Назначение, scope, major constraints, links к связанным разделам. |
| Struct/enum/type alias | Роль в HTTP-contract и контекст использования. |
| Public field | API-семантика, nullable/`Option` behavior, wire-format или unit при необходимости. |
| Input enum variant | Точное документированное wire value и endpoint context. |
| Endpoint method | HTTP verb/URI, purpose, input type, success result, auth/scope caveat, deprecated replacement либо special behavior. |
| Builder/config method | Default, validation, side effect и error condition. |
| Error variant | Источник, доступные diagnostic details и ожидаемое recovery decision. |

## Стиль

Документация пишется по-русски. Rust identifiers, HTTP verbs, URI, JSON key, scope и wire values остаются без перевода и выделяются обратными кавычками. Предпочитайте intra-doc links, например [`ShikimoriClient`](../shikimori_api/struct.ShikimoriClient.html), когда соседний публичный тип помогает навигации.

Публичные response fields, которые deliberately остаются `String` для forward compatibility, должны прямо объяснять этот выбор. Для `Option<T>` обязательно поясняйте, означает ли `None` «не отправлять field», server-side `null` или «значение отсутствует в response».

## Примеры и doctests

Используйте `rust,no_run` для примеров с HTTP: Cargo проверит imports, типы и вызовы, но не обратится к живому API. Изменяющие operations не должны иметь runnable doctest. Для JSON/wire-format contract добавляйте deterministic fixture/unit test в `tests/`.

```rust,no_run
use shikimori_api::{ClientConfig, ShikimoriClient};

# async fn example() -> Result<(), shikimori_api::Error> {
let client = ShikimoriClient::new(
    ClientConfig::builder("my-client/0.1").build()?,
);
let anime = client.anime(1).await?;
println!("{}", anime.name);
# Ok(())
# }
```

## Required commands

Запускайте все команды перед commit документации:

```bash
cargo fmt --check
cargo doc --no-deps --all-features
cargo test --doc --all-features
cargo test --all-targets --all-features --locked
cargo clippy --all-targets --all-features -- -D warnings
```

Публичный API также поддерживает Rust 1.75. Изменения, затрагивающие публичные объявления либо examples, должны дополнительно пройти:

```bash
RUSTC="$(rustup which --toolchain 1.75.0 rustc)" \
  "$(rustup which --toolchain 1.75.0 cargo)" \
  check --all-targets --locked
```

## Где обновлять документы

| Изменение | Обязательные места |
| --- | --- |
| Новый REST route | Method Rustdoc, input/output models, [`endpoint matrix`](endpoint-matrix.md), fixture/doctest. |
| Новый query/body field | Field Rustdoc с exact wire behavior и endpoint method, который его использует. |
| Новый response field | Field Rustdoc и fixture, если возникает nullable/shape compatibility risk. |
| Changed OAuth/scope rule | Crate overview, relevant endpoint Rustdoc, endpoint matrix и README при влиянии на onboarding. |
| Deprecated route | `#[deprecated]` note, replacement method Rustdoc и endpoint matrix. |

## Проверка docs.rs

`Cargo.toml` содержит `[package.metadata.docs.rs]` с `all-features = true`. Не добавляйте nightly-only rustdoc features без отдельной необходимости: published docs должны собираться на stable. Перед release сопоставьте локальный `cargo doc --no-deps --all-features` с открываемой docs.rs страницей пакета.
