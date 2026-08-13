# Справочник Rust API

Этот документ описывает публичную поверхность crate `shikimori-api`. Полная построчная карта HTTP-методов, путей, input и output типов находится в [матрице покрытия](endpoint-matrix.md). Она является контрактом реализации: каждый документированный REST route v1/v2, кроме GraphQL и OAuth flow, имеет публичный async метод.

## Создание клиента

```rust,no_run
use shikimori_api::{ClientConfig, ShikimoriClient};

let config = ClientConfig::builder("example-shikimori-client/0.1")
    // OAuth flow не реализуется: передаётся уже выданный пользователю token.
    .access_token("redacted-access-token")
    .build()?;
let client = ShikimoriClient::new(config);
# Ok::<(), shikimori_api::Error>(())
```

`ClientConfig::builder` требует непустой `User-Agent`. Официальная документация требует указывать имя OAuth2 application и запрещает маскировать клиент под браузер; API также задаёт лимиты 5 rps и 90 rpm. Клиент создаёт **два** совместно используемых governor limiter, поэтому любой clone `ShikimoriClient` соблюдает оба лимита до выполнения HTTP-запроса. [1]

| Поле `ClientConfig` | Назначение | По умолчанию |
| --- | --- | --- |
| `base_url` | Origin API; предназначен также для локального contract testing. | `https://shikimori.io` |
| `user_agent` | Идентификатор приложения в каждом запросе. | Обязателен в builder. |
| `access_token` | Bearer token для защищённых REST routes. Не логируется клиентом. | `None` |
| `requests_per_second` | Пиковая квота governor. | 5 |
| `requests_per_minute` | Минутная квота governor. | 90 |

## Transport, JSON и ошибки

Клиент использует `hyper` 0.14 с `hyper-rustls` и WebPKI roots, Tokio runtime, `simd_json` с Serde и `governor`. Это единственные прямые функциональные зависимости crate. Query-string и multipart upload построены в самом crate, поэтому дополнительные URL, form или multipart dependencies отсутствуют.

| Ошибка | Когда возвращается | Полезные поля |
| --- | --- | --- |
| `Error::Configuration` | Непустой User-Agent, origin или token не прошли валидацию. | Текст причины, без token. |
| `Error::InvalidUri`/`InvalidRequest` | Некорректный внутренний путь/заголовок. | Причина построения HTTP-запроса. |
| `Error::Transport`/`Body` | TLS, соединение либо чтение body завершились ошибкой. | Исходная `hyper::Error`. |
| `Error::Json` | JSON не соответствует публичной response-модели. | Исходная `simd_json::Error`. |
| `Error::Api(ApiError)` | Сервер ответил не-2xx. | `status`, `messages`, lossy `body`, `retry_after_seconds`. |

Неудачные запросы не повторяются автоматически. В частности, это исключает непреднамеренное дублирование создающих и изменяющих действий. Ответ 429 сохраняет числовой `Retry-After` в `ApiError`, если сервер его передал; решение о повторе принимает вызывающий код.

## Модели ответов

| Семейство | Ключевые типы | Принцип моделирования |
| --- | --- | --- |
| Каталог | `Anime`, `AnimeDetails`, `Manga`, `MangaDetails`, `CharacterDetails`, `PersonDetails`, `Genre`, `Studio`, `Publisher` | Compact и full shapes разделены; title/image/status/score и вложенные связи типизированы. |
| Связи | `Role`, `Related`, `Franchise`, `FranchiseLink`, `FranchiseNode`, `ExternalLink`, `Video`, `Screenshot` | Подтипы вынесены отдельно, поэтому не требуется разбирать `Value` вручную. |
| Сообщество | `Club`, `ClubDetails`, `Forum`, `Topic`, `Comment`, `Review`, `Ban`, `Style` | Ресурсы имеют полные поля, а неизвестные будущие поля Serde безопасно игнорирует. |
| Пользователи | `User`, `UserBrief`, `UserDetails`, `UserRate`, `UserRateWithTarget`, `Favourites`, `HistoryEntry`, `UnreadMessages` | Timestamp остаётся string wire-format; это корректно для ISO-8601 и legacy nullable/object date examples. |
| v2 | `IgnoreState`, `AbuseRequest`, `EpisodeNotification` | Каждый v2 action имеет отдельный result type. |

> Output enum-поля, которые сервер может расширить, сохранены строками там, где строгий Rust enum мог бы превратить новый серверный ответ в runtime error. Строгие enum применяются к **input** values, документированным как фиксированный набор.

## Query inputs и фиксированные значения

`ListAnimesQuery`, `ListMangasQuery` и `ListRanobeQuery` дают явные поля для каждого документированного фильтра. Списочные фильтры (`kind`, `status`, `ids`, genres и т. п.) сериализуются в CSV. Для season/kind/status поддерживается документированный режим исключения:

```rust
use shikimori_api::{AnimeKind, ListAnimesQuery, ListValue};

let query = ListAnimesQuery {
    kind: Some(vec![ListValue::Include(AnimeKind::Tv), ListValue::Exclude(AnimeKind::Movie)]),
    search: Some("Cowboy Bebop".into()),
    ..Default::default()
};
```

Этот input будет закодирован как `kind=tv,!movie&search=Cowboy%20Bebop`. Поля пагинации не скрыты в generic map: API потребляет `PageLimitQuery`, `PageLimit30Query`, `ListClubsQuery`, `ListTopicsQuery`, `ListUsersQuery`, `UserAnimeRatesQuery`, `UserMangaRatesQuery`, `UserMessagesQuery`, `UserHistoryQuery` и `ListUserRatesV2Query` согласно конкретной операции.

| Input enum | Допустимые wire values |
| --- | --- |
| `AnimeKind` | `tv`, `movie`, `ova`, `ona`, `special`, `tv_special`, `music`, `pv`, `cm`, `tv_13`, `tv_24`, `tv_48` |
| `MangaKind` | `manga`, `manhwa`, `manhua`, `light_novel`, `novel`, `one_shot`, `doujin` |
| `AnimeStatus` | `anons`, `ongoing`, `released` |
| `MangaStatus` | `anons`, `ongoing`, `released`, `paused`, `discontinued` |
| `UserRateStatus` | `planned`, `watching`, `rewatching`, `completed`, `on_hold`, `dropped` |
| `RateTargetType` | `Anime`, `Manga` |
| `ReviewOpinion` | `positive`, `neutral`, `negative` |
| `VideoKind` | `pv`, `character_trailer`, `cm`, `op`, `ed`, `op_ed_clip`, `clip`, `other`, `episode_preview` |

## Изменяющие input-структуры

Все JSON body используют их документированную обёртку. Например, `CreateCommentRequest { comment: NewComment { … } }` кодируется как `{ "comment": { … } }`, а не как плоский JSON. У option-полей стоит `skip_serializing_if`, поэтому `None` не превращается в неявный `null`.

| Группа | Create / update inputs | Метод |
| --- | --- | --- |
| Club | `UpdateClubRequest`, `ClubInput` | `patch_club`, `put_club` |
| Comment | `CreateCommentRequest`, `NewComment`, `UpdateCommentRequest` | `create_comment`, `patch_comment`, `put_comment` |
| Topic | `CreateTopicRequest`, `NewTopic`, `UpdateTopicRequest`, `TopicPatch` | `create_topic`, `patch_topic`, `put_topic` |
| Review | `CreateReviewRequest`, `NewReview`, `UpdateReviewRequest`, `ReviewPatch` | `create_review`, `patch_review`, `put_review` |
| Message | `CreateMessageRequest`, `NewMessage`, `UpdateMessageRequest`, `MessagePatch` | `create_message`, `patch_message`, `put_message` |
| User rate | `CreateUserRateRequest`, `NewUserRate`, `UpdateUserRateRequest`, `UserRatePatch` | v1 legacy и v2 CRUD |
| Style | `StylePreviewRequest`, `CreateStyleRequest`, `UpdateStyleRequest` | `preview_style`, `create_style`, `patch_style`, `put_style` |
| Video | `CreateVideoRequest`, `NewVideo` | `create_anime_video` |
| v2 abuse | `AbuseRequestInput` | `request_offtopic`, `request_review_conversion`, `request_abuse`, `request_spoiler_review` |
| v2 notification | `EpisodeNotificationRequest`, `NewEpisodeNotification` | `create_episode_notification` |

## Группы методов

Следующая таблица — удобный навигатор. Полный метод/URI/input/result размещён в endpoint matrix, чтобы не дублировать 145+ строк API reference.

| Группа | Основные методы клиента |
| --- | --- |
| Anime | `list_animes`, `anime`, roles/similar/related/screenshots/videos/franchise/external links/topics, legacy `search_animes` |
| Manga и ranobe | `list_mangas`/`list_ranobe`, full show, roles, similar, related, franchise, external links, topics |
| Справочники | `achievements`, `calendar`, `genres`, `publishers`, `studios`, `forums`, constants, `smileys`, `active_users` |
| Clubs | list/show/PATCH/PUT, все 8 subcollections, join/leave |
| Discussions | comments CRUD, topics list/updates/hot/show/CRUD, reviews CRUD |
| Social | appears, bans, dialogs, favourites, friends, legacy ignores/topic ignores |
| Messages | message CRUD, selected/all read/delete, per-user folder and counters |
| Users | list/show/info/whoami/sign-out, friends/clubs/rates/favourites/messages/history/bans |
| User rates | полный v2 CRUD + increment; полный legacy v1 CRUD/cleanup/reset с `#[deprecated]` на old CRUD |
| Media | styles GET/preview/CRUD, anime video GET/create/delete, multipart `upload_user_image` |
| v2 actions | topic/user ignores, moderator report actions, episode notifications |

## Безопасные и изменяющие методы

Публичные list/show GET routes не требуют token, если официальная документация не указывает scope. Для всех защищённых действий передайте `access_token`; библиотека не может проверить OAuth scopes до запроса, и Shikimori вернёт обычный `Error::Api` при недостаточных правах.

`upload_user_image` принимает `UserImageUpload` с filename, raw bytes, optional linked_type и optional MIME type. Filename и header-like values очищаются от CR/LF/кавычек перед ручной multipart-сериализацией. Эта операция требует `comments` scope. [2]

API содержит deprecated v1 endpoints (`/api/ignores`, `/api/topic_ignores`, большинство user-rate CRUD и separate anime/manga search). Они не вырезаны из библиотеки, потому что задача требует полную официальную REST-поверхность. Методы явно снабжены `#[deprecated]`, где upstream фиксирует replacement.

## Важные особенности спецификации

| Особенность | Закреплённое решение |
| --- | --- |
| V2 review conversion | Index v2 и detail-page расходятся. Публичный `request_review_conversion` использует detail route `/api/v2/abuse_requests/convert_review`. [3] |
| Style show | Index перечисляет GET show, detail page в примере содержит PATCH под тем же заголовком. Реализованы GET, PATCH и PUT. [4] |
| Pagination | API часто отдаёт `N+1` объектов при наличии следующей страницы; библиотека не обрезает ответ. [1] |
| Даты Person | Legacy examples возвращают `{}` там, где ожидается дата. `FlexibleDate` допускает text/null/unknown shape. [5] |
| V2 notification token | Это endpoint-level private `token` в `EpisodeNotificationRequest`, а не реализация OAuth. [6] |

## Ссылки

[1]: https://shikimori.io/api/doc/1.0 "Shikimori API v1"
[2]: https://shikimori.io/api/doc/1.0/user_images.html "Shikimori API v1 — User images"
[3]: https://shikimori.io/api/doc/2.0/abuse_requests.html "Shikimori API v2 — Abuse requests"
[4]: https://shikimori.io/api/doc/1.0/styles.html "Shikimori API v1 — Styles"
[5]: https://shikimori.io/api/doc/1.0/people.html "Shikimori API v1 — People"
[6]: https://shikimori.io/api/doc/2.0/episode_notifications.html "Shikimori API v2 — Episode notifications"
