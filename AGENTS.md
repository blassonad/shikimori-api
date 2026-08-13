# AGENTS.md

## Назначение и приоритеты

Этот файл задаёт правила для AI agents и contributors, изменяющих `shikimori-api`. Цели по приоритету: сохранить correctness public REST contract; не раскрыть secrets и не выполнять необратимые external actions без явного запроса; сохранить Rust 1.75 compatibility; поддерживать Rustdoc, tests и endpoint matrix синхронно с кодом.

## Быстрая карта репозитория

| Путь | Назначение |
| --- | --- |
| `src/lib.rs` | Public crate root, re-exports и crate-level Rustdoc. |
| `src/client.rs` | Hyper/Rustls transport, configuration, headers, rate limiting и error mapping. |
| `src/params.rs` | Public query/body DTO, input enum и JSON/query serialization rules. |
| `src/types.rs` | Public response models и tolerant deserialization. |
| `src/v1.rs`, `src/v2.rs` | Inherent endpoint methods [`ShikimoriClient`](src/client.rs). |
| `tests/fixtures.rs` | Deterministic response/serialization tests. |
| `docs/endpoint-matrix.md` | Полная route-to-method coverage table. |
| `docs/api-reference.md` | Расширенный human-readable API contract. |
| `docs/rustdoc-maintenance.md` | Правила public Rustdoc и doctests. |
| `.github/workflows/` | CI и guarded crates.io release automation. |

## Non-negotiable технические ограничения

1. Не добавляйте GraphQL и OAuth authorization/refresh/revoke flow; это вне scope crate.
2. Используйте только утверждённые прямые libraries для transport/runtime/JSON/rate limiting: `hyper` с `hyper-rustls`, `tokio`, `simd_json`, `serde`, `governor`. Перед добавлением любой иной прямой dependency запросите explicit approval.
3. Не заменяйте `simd_json` на `serde_json` в library code. `serde_json` может появляться транзитивно, но не должен вызываться crate API-кодом.
4. Не используйте `unsafe`; crate forbids его на root уровне.
5. Сохраняйте MSRV Rust 1.75. Изменение `rust-version`, edition или dependency MSRV требует отдельного обоснования и проверки.
6. Не логируйте и не помещайте в tests/docs реальные Shikimori access token, crates.io token, GitHub PAT, OAuth client secret или персональные данные.

## Изменение REST API

Перед добавлением либо изменением endpoint-а сверяйтесь с официальной REST documentation Shikimori. Добавьте или обновите method в соответствующем `v1.rs`/`v2.rs`, typed input в `params.rs`, response type в `types.rs`, fixture/wire test, Rustdoc и `docs/endpoint-matrix.md`.

Каждый public endpoint method должен иметь rustdoc с HTTP verb/URI, purpose, входным DTO, success type, auth/scope caveat, special behavior и migration note для deprecated route. Не открывайте response string field в closed enum без future-compatible fallback. Для input enum задавайте точные documented wire values.

## Документация

Публичные items должны иметь русскоязычный Rustdoc. Не заменяйте полезный rustdoc формальной фразой: field docs обязаны объяснять API meaning, `Option<T>` semantics, units, JSON key rename или nullable behavior, где это важно. Используйте `rust,no_run` для HTTP examples, чтобы doctests компилировались без сетевых вызовов.

После изменения public behavior синхронизируйте `README.md`, crate overview, `docs/api-reference.md`, endpoint matrix и maintenance guide, если они затронуты.

## Тестирование и validation

Выполните полный набор до commit:

```bash
cargo fmt --check
cargo doc --no-deps --all-features
cargo test --doc --all-features
cargo test --all-targets --all-features --locked
cargo clippy --all-targets --all-features -- -D warnings
cargo publish --dry-run --locked
RUSTC="$(rustup which --toolchain 1.75.0 rustc)" \
  "$(rustup which --toolchain 1.75.0 cargo)" \
  check --all-targets --locked
```

Tests должны быть deterministic. Не добавляйте live integration tests, выполняющие create/update/delete, в default test suite. Все live tests должны быть opt-in, ограничены public GET и защищены явным feature/ENV guard.

## GitHub Actions и release safety

Workflows должны иметь least-privilege `permissions`. CI никогда не получает `CARGO_REGISTRY_TOKEN`. Publish workflow работает только на `main`, только в GitHub Environment `crates-io`, требует одновременно `CARGO_REGISTRY_TOKEN` и `CRATES_IO_PUBLISH_ENABLED=true`, выполняет `cargo publish --dry-run` перед upload и пропускает существующий version. Не обходите эти guards и не добавляйте `cargo publish` в pull-request workflow.

Публикация crates.io необратима. Workflow может публиковать только version, сознательно изменённый maintainer-ом в `Cargo.toml`; не увеличивайте version автоматически и не force-push release history.

## Изменения репозитория

Не коммитьте `target/`, локальные env files, package archives или downloaded binaries. Не изменяйте `Cargo.lock` вне изменений dependency/version resolution. Создавайте focused commits и не перезаписывайте user-owned remote changes: перед push используйте `git fetch`, затем безопасный rebase/merge и только потом push.
