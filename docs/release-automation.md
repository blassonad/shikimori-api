# Release automation и crates.io

## Назначение

GitHub Actions в `.github/workflows/` разделены на два независимых контура: **CI** проверяет каждый pull request и обновление ветки `main`; **publish** выполняет release-процедуру только после успешного обновления `main` либо при ручном запуске workflow.

> Публикация crates.io необратима: опубликованный version нельзя перезаписать или удалить. Поэтому publish workflow никогда не создаёт новую версию сам и не публикует уже существующий version. [1]

## CI

`ci.yml` запускает format, build, unit/fixture tests, doctests, Clippy, documentation build и `cargo publish --dry-run`. Матрица покрывает `stable` и заявленную MSRV Rust 1.75; package dry-run выполняется на stable. Workflow имеет только `contents: read` permission.

| Trigger | Назначение | Публикация |
| --- | --- | --- |
| `pull_request` в `main` | Проверить contribution до merge. | Нет. |
| `push` в `main` | Проверить интегрированный commit. | Нет: publish находится в отдельном workflow. |
| `workflow_dispatch` | Ручная диагностика CI. | Нет. |

## Publish workflow

`publish.yml` запускается на `push` в `main` и через `workflow_dispatch`. У него есть GitHub Environment `crates-io`; maintainer должен создать это Environment до первого release и сохранить в нём secret `CARGO_REGISTRY_TOKEN`. Environment рекомендуется защитить required reviewer-ом, чтобы deployment ожидал явного одобрения.

| Guard | Поведение |
| --- | --- |
| Repository guard | Publish не выполняется для fork repository. |
| Branch guard | Автоматический trigger реагирует только на `main`. |
| Environment | Job ожидает правила GitHub Environment `crates-io`; secret не доступен ни обычному CI, ни pull request workflow. |
| Package check | Сначала запускается `cargo publish --dry-run --locked`. |
| Version check | Workflow спрашивает crates.io API: если `Cargo.toml` version уже существует, upload пропускается успешно. |
| Explicit token | Реальная публикация использует только `secrets.CARGO_REGISTRY_TOKEN`; token не печатается в logs. |
| Minimal permissions | Workflow имеет `contents: read`; GitHub token не используется для публикации. |

## Однократная настройка maintainer-а

1. Создайте аккаунт crates.io и подтвердите email.
2. Убедитесь, что package name `shikimori-api` свободен или уже принадлежит maintainer-у.
3. Создайте restricted crates.io API token с правом publish/update и храните его только как Environment secret `CARGO_REGISTRY_TOKEN` в GitHub Environment `crates-io`.
4. Создайте GitHub Environment `crates-io`; включите required reviewers и ограничьте deployment branch `main`.
5. Выполните controlled первый release, изменив `version` в `Cargo.toml` и отправив commit в `main`.

После первого release можно заменить token-based публикацию на crates.io Trusted Publishing, если crate ownership и repository association настроены в crates.io. Trusted Publishing использует short-lived GitHub OIDC identity вместо долгоживущего secret. [2]

## Release checklist

Перед merge version bump maintainer должен обновить `Cargo.toml`, `Cargo.lock`, README/API docs при изменении public contract и добавить changelog entry, когда changelog появится. CI проверит package contents и build. После publish проверьте страницу `https://crates.io/crates/shikimori-api` и tag/release policy проекта.

## References

[1]: https://doc.rust-lang.org/cargo/reference/publishing.html "The Cargo Book — Publishing on crates.io"
[2]: https://crates.io/docs/trusted-publishing "crates.io Trusted Publishing"
