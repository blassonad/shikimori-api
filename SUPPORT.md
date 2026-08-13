# Support

## Куда обращаться

| Ситуация | Канал | Что приложить |
| --- | --- | --- |
| Вопрос по использованию crate | [GitHub Discussions](https://github.com/blassonad/shikimori-api/discussions), если включены, либо GitHub issue с меткой `question` | Версия crate/Rust, минимальный фрагмент кода, ожидаемый результат. |
| Reproducible bug в crate | [GitHub issue](https://github.com/blassonad/shikimori-api/issues/new/choose) | Минимальный reproduction, actual/expected behavior, OS, Rust version и package version. |
| Request новой REST-функции | GitHub issue с меткой `enhancement` | Ссылка на официальный маршрут Shikimori, proposed Rust API и compatibility considerations. |
| Уязвимость или leakage | [Private Security Advisory](https://github.com/blassonad/shikimori-api/security/advisories/new) | Следуйте [SECURITY.md](SECURITY.md); не создавайте public issue. |
| Неполадка Shikimori API | Официальная поддержка Shikimori | HTTP status, request ID при наличии; не публикуйте access token. |

## Перед созданием обращения

Проверьте текущий `README.md`, сгенерированную `cargo doc --no-deps --all-features --open` документацию, [API reference](docs/api-reference.md), [endpoint matrix](docs/endpoint-matrix.md) и открытые/закрытые issues. Вопрос должен описывать одну проблему, а example обязан быть минимальным и не включать реальные token или приватные данные.

## Поддерживаемые границы

Maintainer помогает с публичным API этого crate: transport, serialization, request/response models, documented REST route coverage и release automation. OAuth flow, GraphQL, Shikimori account management, получение/выбор токенов и third-party application logic находятся за пределами поддержки.

Для нового вопроса используйте дружелюбный, воспроизводимый формат. Contributions и changes проходят через правила из [CONTRIBUTING.md](CONTRIBUTING.md) и [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
