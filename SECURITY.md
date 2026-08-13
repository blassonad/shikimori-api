# Security Policy

## Поддерживаемые версии

| Версия | Security fixes |
| --- | --- |
| Последняя версия на `main` | Да |
| Последний опубликованный non-yanked release | Да, когда исправление применимо без нарушения SemVer |
| Более старые версии | Нет; обновитесь до поддерживаемой версии |

## Сообщить об уязвимости

Не публикуйте security-sensitive issue в открытом GitHub issue tracker и не добавляйте proof-of-concept с доступными token, endpoint credentials или персональными данными. Используйте [private GitHub Security Advisory](https://github.com/blassonad/shikimori-api/security/advisories/new) репозитория.

В отчёте укажите затронутую версию, минимальный reproducible example, ожидаемое и фактическое поведение, потенциальное влияние, возможный mitigation и контакт для уточняющих вопросов. Никогда не передавайте реальные Shikimori OAuth/access token, crates.io token, GitHub PAT или приватные user data; замените их заведомо недействительными значениями.

## Что происходит после disclosure

Maintainer подтверждает получение отчёта, воспроизводит проблему в закрытом контексте и согласует remediation timeline. Исправление готовится приватно до согласованного disclosure; затем выходит patched crate version и public advisory с credit репортёру, если он этого хочет.

> Публикация crate в crates.io необратима. При попадании секрета в опубликованный package немедленно отзовите secret; `cargo yank` не удаляет исходный архив и не является заменой rotation. [1]

## Границы security responsibility

Crate не получает OAuth token, не хранит учетные данные и не выполняет OAuth flow. Он принимает уже выданный bearer token через configuration и не включает его в own error/debug text. Пользователь отвечает за scopes, secret storage и проверку identity endpoint-а. Проблемы, вызванные внешним Shikimori service или credentials пользователя, следует сообщать соответствующему владельцу сервиса, если они не демонстрируют defect в этом crate.

## References

[1]: https://doc.rust-lang.org/cargo/reference/publishing.html "The Cargo Book — Publishing on crates.io"
