# Матрица покрытия Shikimori REST API v1/v2

**Статус исследования:** завершён. Матрица охватывает каждый маршрут, перечисленный на официальных страницах API v1/v2 на 13 августа 2026 года. GraphQL и OAuth token endpoints намеренно исключены. Все URI приведены относительно `https://shikimori.io`.

> Документация Shikimori называет v1 и v2 устаревшими и рекомендует GraphQL, однако данная библиотека сознательно предоставляет полное типобезопасное покрытие их опубликованной REST-поверхности. Официальное ограничение: **5 rps** и **90 rpm**; клиент применяет оба ограничения до отправки запроса. [1]

## Общие соглашения

| Обозначение | Значение |
| --- | --- |
| `id` | Числовой идентификатор в path, если не обозначено иначе. Все public методы принимают `u64` и корректно percent-encode строковые path-параметры. |
| `Query<T>` | Отдельная `#[derive(Serialize)]` структура, где `None` означает отсутствие параметра. CSV-поля сериализуются вручную; у catalog filter поддержано документированное префиксное исключение `!`. |
| `Body<T>` | JSON `application/json`, сериализованный через `simd_json`; для вложенных legacy inputs клиент сохраняет документированные обёртки (`comment`, `topic`, `user_rate` и т. п.). |
| `Form<T>` | `application/x-www-form-urlencoded`, когда endpoint принимает плоские legacy-параметры. |
| `Multipart` | Ручной `multipart/form-data` для `POST /api/user_images`; дополнительные crates не используются. |
| Auth | `—` означает публичный маршрут; `scope` означает `Authorization: Bearer …` с соответствующим OAuth scope. OAuth-flow не входит в crate. |
| Result | Имя публичной Rust-модели/return type. `()` означает 2xx с пустым body, `Notice` — `{ notice: String }`, `SuccessNotice` — `{ success: bool, notice: String }`. |

## API v1

### Каталог, метаданные и календарь

| HTTP и путь | Публичный метод | Вход | Auth | Результат | Состояние |
| --- | --- | --- | --- | --- | --- |
| `GET /api/achievements` | `achievements(user_id)` | `user_id` обязателен | — | `Vec<Achievement>` | implemented |
| `GET /api/animes` | `list_animes(query)` | `ListAnimesQuery` | — | `Vec<Anime>` | implemented |
| `GET /api/animes/:id` | `anime(id)` | path id | — | `AnimeDetails` | implemented |
| `GET /api/animes/:id/roles` | `anime_roles(id)` | path id | — | `Vec<Role>` | implemented |
| `GET /api/animes/:id/similar` | `similar_animes(id)` | path id | — | `Vec<Anime>` | implemented |
| `GET /api/animes/:id/related` | `related_animes(id)` | path id | — | `Vec<Related>` | implemented |
| `GET /api/animes/:id/screenshots` | `anime_screenshots(id)` | path id | — | `Vec<Screenshot>` | implemented |
| `GET /api/animes/:id/videos` | `anime_videos(id)` | path id | — | `Vec<Video>` | implemented |
| `GET /api/animes/:id/franchise` | `anime_franchise(id)` | path id | — | `Franchise` | implemented |
| `GET /api/animes/:id/external_links` | `anime_external_links(id)` | path id | — | `Vec<ExternalLink>` | implemented |
| `GET /api/animes/search` | `search_animes(query)` | `ListAnimesQuery` | — | `Vec<Anime>` | implemented; deprecated upstream |
| `GET /api/animes/:id/topics` | `anime_topics(id, query)` | `PageLimit30Query` | — | `Vec<Topic>` | implemented |
| `GET /api/calendar` | `calendar(censored)` | `censored: Option<bool>` | — | `Vec<CalendarEntry>` | implemented |
| `GET /api/characters/:id` | `character(id)` | path id | — | `CharacterDetails` | implemented |
| `GET /api/characters/search` | `search_characters(search)` | `search: Option<&str>` | — | `Vec<Character>` | implemented |
| `GET /api/genres` | `genres()` | — | — | `Vec<Genre>` | implemented |
| `GET /api/people/:id` | `person(id)` | path id | — | `PersonDetails` | implemented |
| `GET /api/people/search` | `search_people(query)` | search, `kind: Option<PersonKind>` | — | `Vec<Person>` | implemented |
| `GET /api/publishers` | `publishers()` | — | — | `Vec<Publisher>` | implemented |
| `GET /api/studios` | `studios()` | — | — | `Vec<Studio>` | implemented |
| `GET /api/styles/:id` | `style(id)` | path id | token | `Style` | implemented |
| `POST /api/styles/preview` | `preview_style(css)` | `StylePreviewRequest` | token | `Style` (201) | implemented |
| `POST /api/styles` | `create_style(body)` | `CreateStyleRequest` | token | `Style` (201) | implemented |
| `PATCH /api/styles/:id` | `patch_style(id, body)` | `UpdateStyleRequest` | token | `Style` | implemented |
| `PUT /api/styles/:id` | `put_style(id, body)` | `UpdateStyleRequest` | token | `Style` | implemented |
| `GET /api/constants/anime` | `anime_constants()` | — | — | `AnimeConstants` | implemented |
| `GET /api/constants/manga` | `manga_constants()` | — | — | `MangaConstants` | implemented |
| `GET /api/constants/user_rate` | `user_rate_constants()` | — | — | `UserRateConstants` | implemented |
| `GET /api/constants/club` | `club_constants()` | — | — | `ClubConstants` | implemented |
| `GET /api/constants/smileys` | `smileys()` | — | — | `Vec<Smiley>` | implemented |
| `GET /api/stats/active_users` | `active_users()` | — | — | `Vec<UserId>` | implemented |

`ListAnimesQuery` включает `page`, `limit ≤ 50`, `order`, legacy `type`, `kind`, `status`, `season`, минимальный `score`, `duration`, `rating`, `genre`, `genre_v2`, `studio`, `franchise`, `censored`, `mylist`, `ids`, `exclude_ids`, `search`. `AnimeOrder` покрывает `id`, `id_desc`, `ranked`, `kind`, `popularity`, `name`, `aired_on`, `episodes`, `status`, `random`, `ranked_random`, `ranked_shiki`, `created_at`, `created_at_desc`, `updated_at`, `updated_at_desc`. `CreateStyleRequest` requires css/name/owner_id/owner_type (`User|Club`); update accepts css/name; preview requires css.

| HTTP и путь | Публичный метод | Вход | Auth | Результат | Состояние |
| --- | --- | --- | --- | --- | --- |
| `GET /api/mangas` | `list_mangas(query)` | `ListMangasQuery` | — | `Vec<Manga>` | implemented |
| `GET /api/mangas/:id` | `manga(id)` | path id | — | `MangaDetails` | implemented |
| `GET /api/mangas/:id/roles` | `manga_roles(id)` | path id | — | `Vec<Role>` | implemented |
| `GET /api/mangas/:id/similar` | `similar_mangas(id)` | path id | — | `Vec<Manga>` | implemented |
| `GET /api/mangas/:id/related` | `related_mangas(id)` | path id | — | `Vec<Related>` | implemented |
| `GET /api/mangas/:id/franchise` | `manga_franchise(id)` | path id | — | `Franchise` | implemented |
| `GET /api/mangas/:id/external_links` | `manga_external_links(id)` | path id | — | `Vec<ExternalLink>` | implemented |
| `GET /api/mangas/search` | `search_mangas(query)` | `ListMangasQuery` | — | `Vec<Manga>` | implemented; deprecated upstream |
| `GET /api/mangas/:id/topics` | `manga_topics(id, query)` | `PageLimit30Query` | — | `Vec<Topic>` | implemented |
| `GET /api/ranobe` | `list_ranobe(query)` | `ListRanobeQuery` | — | `Vec<Manga>` | implemented |
| `GET /api/ranobe/:id` | `ranobe(id)` | path id | — | `MangaDetails` | implemented |
| `GET /api/ranobe/:id/roles` | `ranobe_roles(id)` | path id | — | `Vec<Role>` | implemented |
| `GET /api/ranobe/:id/similar` | `similar_ranobe(id)` | path id | — | `Vec<Manga>` | implemented |
| `GET /api/ranobe/:id/related` | `related_ranobe(id)` | path id | — | `Vec<Related>` | implemented |
| `GET /api/ranobe/:id/franchise` | `ranobe_franchise(id)` | path id | — | `Franchise` | implemented |
| `GET /api/ranobe/:id/external_links` | `ranobe_external_links(id)` | path id | — | `Vec<ExternalLink>` | implemented |
| `GET /api/ranobe/:id/topics` | `ranobe_topics(id, query)` | `PageLimit30Query` | — | `Vec<Topic>` | implemented |

`ListMangasQuery` и `ListRanobeQuery` имеют `page`, `limit ≤ 50`, manga-специфичный order (включая `volumes`, `chapters`), status, season, score, genre, publisher, franchise, censored, mylist, ids, exclude_ids и search. У Manga дополнительно kind (`manga`, `manhwa`, `manhua`, `light_novel`, `novel`, `one_shot`, `doujin`) и `genre_v2`; Ranobe фиксирует вид light novel на сервере.

### Клубы, форумы, комментарии, темы и обзоры

| HTTP и путь | Публичный метод | Вход | Auth | Результат | Состояние |
| --- | --- | --- | --- | --- | --- |
| `GET /api/clubs` | `list_clubs(query)` | page, limit ≤30, search | — | `Vec<Club>` | implemented |
| `GET /api/clubs/:id` | `club(id)` | path id | — | `ClubDetails` | implemented |
| `PATCH /api/clubs/:id` | `patch_club(id, body)` | `UpdateClubRequest` | `clubs` | `ClubDetails` | implemented |
| `PUT /api/clubs/:id` | `put_club(id, body)` | `UpdateClubRequest` | `clubs` | `ClubDetails` | implemented |
| `GET /api/clubs/:id/animes` | `club_animes(id, query)` | page, limit ≤20 | — | `Vec<Anime>` | implemented |
| `GET /api/clubs/:id/mangas` | `club_mangas(id, query)` | page, limit ≤20 | — | `Vec<Manga>` | implemented |
| `GET /api/clubs/:id/ranobe` | `club_ranobe(id, query)` | page, limit ≤20 | — | `Vec<Manga>` | implemented |
| `GET /api/clubs/:id/characters` | `club_characters(id, query)` | page, limit ≤20 | — | `Vec<Character>` | implemented |
| `GET /api/clubs/:id/collections` | `club_collections(id, query)` | page, limit ≤4 | — | `Vec<Topic>` | implemented |
| `GET /api/clubs/:id/clubs` | `club_related_clubs(id, query)` | page, limit ≤30 | — | `Vec<Club>` | implemented |
| `GET /api/clubs/:id/members` | `club_members(id, query)` | page, limit ≤100 | — | `Vec<User>` | implemented |
| `GET /api/clubs/:id/images` | `club_images(id, query)` | page, limit ≤100 | — | `Vec<ClubImage>` | implemented |
| `POST /api/clubs/:id/join` | `join_club(id)` | empty body | `clubs` | `()` | implemented |
| `POST /api/clubs/:id/leave` | `leave_club(id)` | empty body | `clubs` | `()` | implemented |
| `GET /api/forums` | `forums()` | — | — | `Vec<Forum>` | implemented |
| `GET /api/comments` | `list_comments(query)` | required commentable id/type, page, limit ≤30, desc | — | `Vec<Comment>` | implemented |
| `GET /api/comments/:id` | `comment(id)` | path id | — | `Comment` | implemented |
| `POST /api/comments` | `create_comment(body)` | `CreateCommentRequest` | `comments` | `Comment` (201) | implemented |
| `PATCH /api/comments/:id` | `patch_comment(id, body)` | `UpdateCommentRequest` | `comments` | `Comment` | implemented |
| `PUT /api/comments/:id` | `put_comment(id, body)` | `UpdateCommentRequest` | `comments` | `Comment` | implemented |
| `DELETE /api/comments/:id` | `delete_comment(id)` | path id | `comments` | `Notice` | implemented |
| `GET /api/topics` | `list_topics(query)` | `ListTopicsQuery` | — | `Vec<Topic>` | implemented |
| `GET /api/topics/updates` | `topic_updates(query)` | page, limit ≤30 | — | `Vec<TopicUpdate>` | implemented |
| `GET /api/topics/hot` | `hot_topics(limit)` | limit ≤10 | — | `Vec<Topic>` | implemented |
| `GET /api/topics/:id` | `topic(id)` | path id | — | `Topic` | implemented |
| `POST /api/topics` | `create_topic(body)` | `CreateTopicRequest` | `topics` | `Topic` (201) | implemented |
| `PATCH /api/topics/:id` | `patch_topic(id, body)` | `UpdateTopicRequest` | `topics` | `Topic` | implemented |
| `PUT /api/topics/:id` | `put_topic(id, body)` | `UpdateTopicRequest` | `topics` | `Topic` | implemented |
| `DELETE /api/topics/:id` | `delete_topic(id)` | path id | `topics` | `Notice` | implemented |
| `POST /api/reviews` | `create_review(body)` | `CreateReviewRequest` | token | `Review` (201) | implemented |
| `PATCH /api/reviews/:id` | `patch_review(id, body)` | `UpdateReviewRequest` | token | `Review` | implemented |
| `PUT /api/reviews/:id` | `put_review(id, body)` | `UpdateReviewRequest` | token | `Review` | implemented |
| `DELETE /api/reviews/:id` | `delete_review(id)` | path id | token | `Notice` | implemented |

`UpdateClubRequest` contains name, description, display_images, comment_policy, topic_policy, image_upload_policy. `CreateCommentRequest` wraps required body/commentable id/commentable type plus optional is_offtopic, frontend and broadcast flags. `CreateTopicRequest` wraps required body/forum id/title/type (`Topic`)/user id and optional linked id/type; update permits body/title/linked fields. `CreateReviewRequest` requires anime_id/body/opinion (`positive|neutral|negative`); update accepts body/opinion.

### Социальные отношения, сообщения, избранное и служебные действия

| HTTP и путь | Публичный метод | Вход | Auth | Результат | Состояние |
| --- | --- | --- | --- | --- | --- |
| `POST /api/appears` | `mark_appeared(ids)` | CSV `ids?` | token | `()` | implemented |
| `GET /api/bans` | `bans()` | — | token | `Vec<Ban>` | implemented |
| `GET /api/dialogs` | `dialogs()` | — | `messages` | `Vec<Dialog>` | implemented |
| `GET /api/dialogs/:id` | `dialog(id)` | string user/nickname id | `messages` | `Vec<MessageWithParticipants>` | implemented |
| `DELETE /api/dialogs/:id` | `delete_dialog(id)` | string user/nickname id | `messages` | `Notice` | implemented |
| `POST /api/favorites/:linked_type/:linked_id/:kind` | `create_favorite(target)` | target + optional kind | token | `SuccessNotice` | implemented |
| `DELETE /api/favorites/:linked_type/:linked_id` | `delete_favorite(target)` | target | token | `SuccessNotice` | implemented |
| `POST /api/favorites/:id/reorder` | `reorder_favorite(id, index)` | `new_index?` | token | `()` | implemented |
| `POST /api/friends/:id` | `add_friend(id)` | user id | `friends` | `Notice` | implemented |
| `DELETE /api/friends/:id` | `delete_friend(id)` | user id | `friends` | `Notice` | implemented |
| `POST /api/ignores/:id` | `ignore_user_v1(id)` | user id | `ignores` | `Notice` | implemented; deprecated upstream |
| `DELETE /api/ignores/:id` | `unignore_user_v1(id)` | user id | `ignores` | `Notice` | implemented; deprecated upstream |
| `GET /api/messages/:id` | `message(id)` | path id | `messages` | `MessageWithParticipants` | implemented |
| `POST /api/messages` | `create_message(body)` | `CreateMessageRequest` | `messages` | `MessageWithParticipants` (201) | implemented |
| `PATCH /api/messages/:id` | `patch_message(id, body)` | `UpdateMessageRequest` | `messages` | `MessageWithParticipants` | implemented |
| `PUT /api/messages/:id` | `put_message(id, body)` | `UpdateMessageRequest` | `messages` | `MessageWithParticipants` | implemented |
| `DELETE /api/messages/:id` | `delete_message(id)` | path id | `messages` | `()` (204) | implemented |
| `POST /api/messages/mark_read` | `mark_messages_read(body)` | `ids?`, `is_read: 0|1?` | `messages` | `()` | implemented |
| `POST /api/messages/read_all` | `read_all_messages(kind)` | required `news|notifications` | `messages` | `()` | implemented |
| `POST /api/messages/delete_all` | `delete_all_messages(kind)` | required `news|notifications` | `messages` | `()` | implemented |
| `POST /api/topic_ignores` | `create_topic_ignore_v1(body)` | topic_id?, user_id? | `topics` | `ToggleUrl` | implemented; deprecated upstream |
| `DELETE /api/topic_ignores/:id` | `delete_topic_ignore_v1(id)` | path id | `topics` | `ToggleUrl` | implemented; deprecated upstream |
| `POST /api/user_images` | `upload_user_image(file, type)` | multipart image required, linked_type? | `comments` | `UserImage` | implemented |

### Пользователи, списки и legacy user rates

| HTTP и путь | Публичный метод | Вход | Auth | Результат | Состояние |
| --- | --- | --- | --- | --- | --- |
| `GET /api/users` | `list_users(query)` | page, limit ≤100, search | — | `Vec<User>` | implemented |
| `GET /api/users/:id` | `user(id, is_nickname)` | string id/nickname, `is_nickname=1?` | — | `UserDetails` | implemented |
| `GET /api/users/:id/info` | `user_info(id)` | string id/nickname | — | `UserBrief` | implemented |
| `GET /api/users/whoami` | `whoami()` | — | token | `UserBrief` | implemented |
| `POST /api/users/sign_out` | `sign_out()` | empty body | token | `PlainText` | implemented |
| `GET /api/users/:id/friends` | `user_friends(id, query)` | page, limit ≤100 | — | `Vec<User>` | implemented |
| `GET /api/users/:id/clubs` | `user_clubs(id)` | path id | — | `Vec<Club>` | implemented |
| `GET /api/users/:id/anime_rates` | `user_anime_rates(id, query)` | page, limit ≤5000, status?, censored? | — | `Vec<UserRateWithTarget>` | implemented |
| `GET /api/users/:id/manga_rates` | `user_manga_rates(id, query)` | page, limit ≤5000, censored? | — | `Vec<UserRateWithTarget>` | implemented |
| `GET /api/users/:id/favourites` | `user_favourites(id)` | path id | — | `Favourites` | implemented |
| `GET /api/users/:id/messages` | `user_messages(id, query)` | required inbox/private/sent/news/notifications; page/limit≤100 | `messages` | `Vec<MessageWithParticipants>` | implemented |
| `GET /api/users/:id/unread_messages` | `unread_messages(id)` | path id | `messages` | `UnreadMessages` | implemented |
| `GET /api/users/:id/history` | `user_history(id, query)` | page/limit≤100, target id/type? | — | `Vec<HistoryEntry>` | implemented |
| `GET /api/users/:id/bans` | `user_bans(id)` | path id | — | `Vec<Ban>` | implemented |
| `GET /api/user_rates/:id` | `user_rate_v1(id)` | path id | — | `UserRate` | implemented; deprecated upstream |
| `POST /api/user_rates` | `create_user_rate_v1(body)` | `CreateUserRateRequest` | `user_rates` | `UserRateWithTarget` (201) | implemented; deprecated upstream |
| `PATCH /api/user_rates/:id` | `patch_user_rate_v1(id, body)` | `UpdateUserRateRequest` | `user_rates` | `UserRateWithTarget` | implemented; deprecated upstream |
| `PUT /api/user_rates/:id` | `put_user_rate_v1(id, body)` | `UpdateUserRateRequest` | `user_rates` | `UserRateWithTarget` | implemented; deprecated upstream |
| `POST /api/user_rates/:id/increment` | `increment_user_rate_v1(id)` | path id | `user_rates` | `UserRateWithTarget` (201) | implemented; deprecated upstream |
| `DELETE /api/user_rates/:id` | `delete_user_rate_v1(id)` | path id | `user_rates` | `()` (204) | implemented; deprecated upstream |
| `DELETE /api/user_rates/:type/cleanup` | `cleanup_user_rates(kind)` | `anime|manga` | `user_rates` | `Notice` | implemented |
| `DELETE /api/user_rates/:type/reset` | `reset_user_rate_scores(kind)` | `anime|manga` | `user_rates` | `Notice` | implemented |

`CreateUserRateRequest` has required user_id/target_id/target_type (`Anime|Manga`)/status and optional score, chapters, episodes, volumes, rewatches, text. Update permits only mutable fields. v2 should be preferred for normal user-rate CRUD.

### Видео

| HTTP и путь | Публичный метод | Вход | Auth | Результат | Состояние |
| --- | --- | --- | --- | --- | --- |
| `GET /api/animes/:id/videos` | `anime_videos(id)` | path id | — | `Vec<Video>` | implemented |
| `POST /api/animes/:id/videos` | `create_anime_video(id, body)` | `VideoInput` | `content` | `Video` (201) | implemented |
| `DELETE /api/animes/:anime_id/videos/:id` | `delete_anime_video(anime_id, id)` | path ids | `content` | `()` | implemented |

`VideoInput` requires `kind`, `name`, `url`; `VideoKind`: `pv`, `character_trailer`, `cm`, `op`, `ed`, `op_ed_clip`, `clip`, `other`, `episode_preview`.

## API v2

| HTTP и путь | Публичный метод | Вход | Auth | Результат | Состояние |
| --- | --- | --- | --- | --- | --- |
| `POST /api/v2/topics/:topic_id/ignore` | `ignore_topic(topic_id)` | path id | `topics` | `IgnoreState` | implemented |
| `DELETE /api/v2/topics/:topic_id/ignore` | `unignore_topic(topic_id)` | path id | `topics` | `IgnoreState` | implemented |
| `POST /api/v2/users/:user_id/ignore` | `ignore_user(user_id)` | path id | `ignores` | `IgnoreState` | implemented |
| `DELETE /api/v2/users/:user_id/ignore` | `unignore_user(user_id)` | path id | `ignores` | `IgnoreState` | implemented |
| `POST /api/v2/abuse_requests/offtopic` | `request_offtopic(body)` | required comment_id | token | `AbuseRequest` (201) | implemented |
| `POST /api/v2/abuse_requests/convert_review` | `request_review_conversion(body)` | comment_id?, topic_id? | token | `()` | implemented |
| `POST /api/v2/abuse_requests/abuse` | `request_abuse(body)` | comment_id?, topic_id?, reason? | token | `()` | implemented |
| `POST /api/v2/abuse_requests/spoiler` | `request_spoiler_review(body)` | comment_id?, topic_id?, reason? | token | `()` | implemented |
| `POST /api/v2/episode_notifications` | `create_episode_notification(body)` | `EpisodeNotificationRequest` | private endpoint token in body | `EpisodeNotification` | implemented |
| `GET /api/v2/user_rates` | `list_user_rates_v2(query)` | `ListUserRatesV2Query` | — | `Vec<UserRate>` | implemented |
| `GET /api/v2/user_rates/:id` | `user_rate_v2(id)` | path id | — | `UserRate` | implemented |
| `POST /api/v2/user_rates` | `create_user_rate_v2(body)` | `CreateUserRateRequest` | `user_rates` | `UserRate` (201) | implemented |
| `PATCH /api/v2/user_rates/:id` | `patch_user_rate_v2(id, body)` | `UpdateUserRateRequest` | `user_rates` | `UserRate` | implemented |
| `PUT /api/v2/user_rates/:id` | `put_user_rate_v2(id, body)` | `UpdateUserRateRequest` | `user_rates` | `UserRate` | implemented |
| `POST /api/v2/user_rates/:id/increment` | `increment_user_rate_v2(id)` | path id | `user_rates` | `UserRate` (201) | implemented |
| `DELETE /api/v2/user_rates/:id` | `delete_user_rate_v2(id)` | path id | `user_rates` | `()` (204) | implemented |

`ListUserRatesV2Query`: optional user_id, target_id, target_type (`Anime|Manga`), status, page and limit (max 1000); API ignores page/limit when user_id is specified. `EpisodeNotificationRequest` nests required anime_id/episode/aired_at (ISO 8601) plus optional availability booleans, and has a top-level required private token.

## Зафиксированные расхождения документации

| Место | Расхождение | Поведение библиотеки |
| --- | --- | --- |
| v2 index vs detail: review conversion | Index выводит `/api/v2/abuse_requests/review`, detailed route page показывает `/api/v2/abuse_requests/convert_review`. | Public method использует `convert_review`, поскольку это точный путь официальной детальной страницы; rustdoc фиксирует оба значения. |
| Styles index vs detail | Index заявляет `GET /api/styles/:id`; подробная страница по заголовку «Show» демонстрирует `PATCH`. | Публично реализуется документированный GET show и PATCH/PUT update; fixture/tests разделяют маршруты. |
| Anime videos | Ресурс animes помечает GET videos deprecated, dedicated Videos page включает тот же GET и CRUD. | Один не дублирующий `anime_videos` плюс create/delete. |
| Person date fields | Examples содержат `{}` для birth/deceased/birthday вместо устойчивой string/null схемы. | Типы сохраняют wire-compatible гибкую JSON форму, без ложной строгой date-десериализации. |

## Источники

[1]: https://shikimori.io/api/doc/1.0 "Shikimori API v1"
[2]: https://shikimori.io/api/doc/2.0 "Shikimori API v2"
[3]: https://shikimori.io/api/doc/1.0/animes.html "Shikimori API v1 — Animes"
[4]: https://shikimori.io/api/doc/1.0/users.html "Shikimori API v1 — Users"
[5]: https://shikimori.io/api/doc/1.0/topics.html "Shikimori API v1 — Topics"
[6]: https://shikimori.io/api/doc/1.0/user_rates.html "Shikimori API v1 — User rates"
[7]: https://shikimori.io/api/doc/2.0/user_rates.html "Shikimori API v2 — User rates"
[8]: https://shikimori.io/api/doc/2.0/abuse_requests.html "Shikimori API v2 — Abuse requests"
