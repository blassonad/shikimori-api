# Contributing

Спасибо за интерес к `shikimori-api`. Проект принимает воспроизводимые bug fixes, improvements типов/документации, тесты и новые REST routes, подтверждённые официальной документацией Shikimori.

## Перед началом

Сначала откройте issue либо найдите существующее обсуждение для нетривиального изменения public API. Для нового route приложите URI, HTTP method, scope, request/response shape и ссылку на официальную документацию. Не добавляйте GraphQL или OAuth issuance/refresh flow: это явно вне scope crate.

Все участники соблюдают [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md). Security defects сообщаются только по [SECURITY.md](SECURITY.md).

## Локальная настройка

```bash
git clone https://github.com/blassonad/shikimori-api.git
cd shikimori-api
rustup toolchain install 1.75.0
cargo test --all-targets --all-features --locked
```

Crate поддерживает Rust 1.75. Не добавляйте новую прямую dependency, пока невозможно доказать, что она необходима и совместима с заявленной MSRV. HTTP transport, TLS, runtime, JSON и rate limiting намеренно ограничены `hyper`/`hyper-rustls`, `tokio`, `simd_json`, `serde` и `governor`.

## Рабочий процесс

Создавайте одну ветку на изменение и один focused pull request. Используйте понятный conventional-like commit subject, например `fix: encode nickname path segment` или `docs: clarify user-rate wire format`. Pull request должен объяснить проблему, решение, breaking-change impact и ссылаться на issue, если он был.

| Тип изменения | Обязательное дополнение |
| --- | --- |
| Новый endpoint | Typed query/body/result models, method Rustdoc, endpoint matrix, fixture/doctest и ссылка на Shikimori docs. |
| Изменение request serialization | Тест точного JSON/query/multipart wire-format. |
| Изменение response model | Fixture для documented response shape; пояснение nullable/forward compatibility. |
| Public API breaking change | SemVer rationale, migration note и version bump proposal. |
| Documentation change | Проверка `cargo doc` и doctests. |
| Workflow change | Least-privilege permissions, отсутствие secret в logs и описание trigger/guard. |

## Required checks

Запустите все проверки до push. CI повторяет их на pull request и main.

```bash
cargo fmt --check
cargo doc --no-deps --all-features
cargo test --doc --all-features
cargo test --all-targets --all-features --locked
cargo clippy --all-targets --all-features -- -D warnings
cargo publish --dry-run --locked
```

Для MSRV-sensitive изменения дополнительно выполните:

```bash
RUSTC="$(rustup which --toolchain 1.75.0 rustc)" \
  "$(rustup which --toolchain 1.75.0 cargo)" \
  check --all-targets --locked
```

## Стиль Rust и API

Следуйте `rustfmt`; не добавляйте `allow` для подавления корректного Clippy или Rustdoc warning без пояснения в pull request. Публичные структуры, поля, enum variants и methods должны иметь русскоязычный Rustdoc; precise identifiers, URI, JSON keys и wire values оставляйте в обратных кавычках. Подробные правила находятся в [docs/rustdoc-maintenance.md](docs/rustdoc-maintenance.md).

Поддерживайте строго типизированные input enum и tolerant output model policy. Не превращайте API fields с потенциальными будущими server values в закрытые response enum без fallback. Сохраняйте `Option<T>` semantics: `None` в input обычно означает «не сериализировать поле», тогда как output `None` может отражать `null`/отсутствие field у server.

## Pull request checklist

- [ ] Изменение ограничено одной понятной задачей и не содержит случайных format/refactor шумов.
- [ ] Нет secrets, личных access token, downloaded binaries или `target/` artifacts.
- [ ] Все required checks проходят локально.
- [ ] Public API имеет Rustdoc, а новые HTTP contracts имеют test/fixture.
- [ ] `README.md`, Rustdoc, endpoint matrix и API reference синхронизированы при изменении поведения.
- [ ] Изменения `Cargo.toml` объяснены и не нарушают Rust 1.75.
