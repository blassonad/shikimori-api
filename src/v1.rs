//! REST-методы Shikimori API v1.
//!
//! Этот модуль группирует complete legacy REST surface v1. Все методы остаются
//! inherent methods [`ShikimoriClient`]; модуль служит навигационным разделом в
//! `cargo doc` и не требует отдельного client handle.
//!
//! ## Группы ресурсов
//!
//! | Группа | Методы |
//! | --- | --- |
//! | Каталог | Anime, manga, ranobe, characters, people, calendar, constants, genres, studios, publishers. |
//! | Community | Clubs, forums, comments, topics, reviews, dialogs и messages. |
//! | Users | Profiles, friends, favourites, history, list rates, bans и private folders. |
//! | Media | Styles, anime videos и multipart user image upload. |
//!
//! ## Авторизация и статусы
//!
//! Public GET routes не требуют token. Изменяющие methods требуют bearer token
//! и scope, указанный официальной спецификацией. Каждый method возвращает
//! [`Result<T>`]; HTTP non-success доступен как [`Error::Api`](crate::Error::Api).
//! Server-side scope validation не заменяется клиентом.
//!
//! ## Устаревшие маршруты
//!
//! V1 routes всё ещё документированы и реализованы для compatibility. Методы
//! `ignore_user_v1`, `unignore_user_v1`, topic-ignore и user-rate CRUD имеют
//! `#[deprecated]`: для новых интеграций используйте v2 equivalents. Legacy
//! separate search routes также сохранены, но предпочтительнее list methods с
//! полем `search` в [`ListAnimesQuery`] или [`ListMangasQuery`].
//!
//! ## Пагинация
//!
//! Используйте endpoint-specific query structs. Shikimori может вернуть `N+1`
//! results для signal о следующей странице; client намеренно не обрезает array.
//!
//! Полная HTTP-матрица: <https://github.com/blassonad/shikimori-api/blob/feat/complete-rest-client/docs/endpoint-matrix.md>.

use crate::{client::ShikimoriClient, encoding::path_segment, params::*, types::*, Result};

impl ShikimoriClient {
    // Каталоги и метаданные.
    /// Выполняет `GET /api/achievements`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn achievements(&self, user_id: u64) -> Result<Vec<Achievement>> {
        self.get("/api/achievements", &AchievementsQuery { user_id })
            .await
    }
    /// Выполняет `GET /api/animes`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn list_animes(&self, q: ListAnimesQuery) -> Result<Vec<Anime>> {
        self.get("/api/animes", &q).await
    }
    /// Выполняет `GET /api/animes/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn anime(&self, id: u64) -> Result<AnimeDetails> {
        self.get(&format!("/api/animes/{id}"), &()).await
    }
    /// Выполняет `GET /api/animes/{id}/roles`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn anime_roles(&self, id: u64) -> Result<Vec<Role>> {
        self.get(&format!("/api/animes/{id}/roles"), &()).await
    }
    /// Выполняет `GET /api/animes/{id}/similar`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn similar_animes(&self, id: u64) -> Result<Vec<Anime>> {
        self.get(&format!("/api/animes/{id}/similar"), &()).await
    }
    /// Выполняет `GET /api/animes/{id}/related`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn related_animes(&self, id: u64) -> Result<Vec<Related>> {
        self.get(&format!("/api/animes/{id}/related"), &()).await
    }
    /// Выполняет `GET /api/animes/{id}/screenshots`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn anime_screenshots(&self, id: u64) -> Result<Vec<Screenshot>> {
        self.get(&format!("/api/animes/{id}/screenshots"), &())
            .await
    }
    /// Выполняет `GET /api/animes/{id}/videos`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn anime_videos(&self, id: u64) -> Result<Vec<Video>> {
        self.get(&format!("/api/animes/{id}/videos"), &()).await
    }
    /// Выполняет `GET /api/animes/{id}/franchise`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn anime_franchise(&self, id: u64) -> Result<Franchise> {
        self.get(&format!("/api/animes/{id}/franchise"), &()).await
    }
    /// Выполняет `GET /api/animes/{id}/external_links`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn anime_external_links(&self, id: u64) -> Result<Vec<ExternalLink>> {
        self.get(&format!("/api/animes/{id}/external_links"), &())
            .await
    }
    #[deprecated(note = "use list_animes with ListAnimesQuery.search")]
    /// Выполняет `GET /api/animes/search`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn search_animes(&self, q: ListAnimesQuery) -> Result<Vec<Anime>> {
        self.get("/api/animes/search", &q).await
    }
    /// Выполняет `GET /api/animes/{id}/topics`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn anime_topics(&self, id: u64, q: PageLimit30Query) -> Result<Vec<Topic>> {
        self.get(&format!("/api/animes/{id}/topics"), &q).await
    }
    /// Выполняет `GET /api/calendar`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn calendar(&self, censored: Option<bool>) -> Result<Vec<CalendarEntry>> {
        self.get("/api/calendar", &CensoredQuery { censored }).await
    }
    /// Выполняет `GET /api/characters/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn character(&self, id: u64) -> Result<CharacterDetails> {
        self.get(&format!("/api/characters/{id}"), &()).await
    }
    /// Выполняет `GET /api/characters/search`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn search_characters(&self, search: Option<String>) -> Result<Vec<Character>> {
        self.get(
            "/api/characters/search",
            &SearchQuery { search, kind: None },
        )
        .await
    }
    /// Выполняет `GET /api/genres`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn genres(&self) -> Result<Vec<Genre>> {
        self.get("/api/genres", &()).await
    }
    /// Выполняет `GET /api/people/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn person(&self, id: u64) -> Result<PersonDetails> {
        self.get(&format!("/api/people/{id}"), &()).await
    }
    /// Выполняет `GET /api/people/search`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn search_people(
        &self,
        search: Option<String>,
        kind: Option<PersonKind>,
    ) -> Result<Vec<Person>> {
        self.get("/api/people/search", &SearchQuery { search, kind })
            .await
    }
    /// Выполняет `GET /api/publishers`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn publishers(&self) -> Result<Vec<Publisher>> {
        self.get("/api/publishers", &()).await
    }
    /// Выполняет `GET /api/studios`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn studios(&self) -> Result<Vec<Studio>> {
        self.get("/api/studios", &()).await
    }
    /// Выполняет `GET /api/constants/anime`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn anime_constants(&self) -> Result<AnimeConstants> {
        self.get("/api/constants/anime", &()).await
    }
    /// Выполняет `GET /api/constants/manga`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn manga_constants(&self) -> Result<MangaConstants> {
        self.get("/api/constants/manga", &()).await
    }
    /// Выполняет `GET /api/constants/user_rate`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn user_rate_constants(&self) -> Result<UserRateConstants> {
        self.get("/api/constants/user_rate", &()).await
    }
    /// Выполняет `GET /api/constants/club`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn club_constants(&self) -> Result<ClubConstants> {
        self.get("/api/constants/club", &()).await
    }
    /// Выполняет `GET /api/constants/smileys`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn smileys(&self) -> Result<Vec<Smiley>> {
        self.get("/api/constants/smileys", &()).await
    }
    /// Выполняет `GET /api/stats/active_users`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn active_users(&self) -> Result<Vec<UserId>> {
        self.get("/api/stats/active_users", &()).await
    }

    /// Выполняет `GET /api/mangas`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn list_mangas(&self, q: ListMangasQuery) -> Result<Vec<Manga>> {
        self.get("/api/mangas", &q).await
    }
    /// Выполняет `GET /api/mangas/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn manga(&self, id: u64) -> Result<MangaDetails> {
        self.get(&format!("/api/mangas/{id}"), &()).await
    }
    /// Выполняет `GET /api/mangas/{id}/roles`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn manga_roles(&self, id: u64) -> Result<Vec<Role>> {
        self.get(&format!("/api/mangas/{id}/roles"), &()).await
    }
    /// Выполняет `GET /api/mangas/{id}/similar`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn similar_mangas(&self, id: u64) -> Result<Vec<Manga>> {
        self.get(&format!("/api/mangas/{id}/similar"), &()).await
    }
    /// Выполняет `GET /api/mangas/{id}/related`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn related_mangas(&self, id: u64) -> Result<Vec<Related>> {
        self.get(&format!("/api/mangas/{id}/related"), &()).await
    }
    /// Выполняет `GET /api/mangas/{id}/franchise`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn manga_franchise(&self, id: u64) -> Result<Franchise> {
        self.get(&format!("/api/mangas/{id}/franchise"), &()).await
    }
    /// Выполняет `GET /api/mangas/{id}/external_links`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn manga_external_links(&self, id: u64) -> Result<Vec<ExternalLink>> {
        self.get(&format!("/api/mangas/{id}/external_links"), &())
            .await
    }
    #[deprecated(note = "use list_mangas with ListMangasQuery.search")]
    /// Выполняет `GET /api/mangas/search`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn search_mangas(&self, q: ListMangasQuery) -> Result<Vec<Manga>> {
        self.get("/api/mangas/search", &q).await
    }
    /// Выполняет `GET /api/mangas/{id}/topics`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn manga_topics(&self, id: u64, q: PageLimit30Query) -> Result<Vec<Topic>> {
        self.get(&format!("/api/mangas/{id}/topics"), &q).await
    }
    /// Выполняет `GET /api/ranobe`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn list_ranobe(&self, q: ListRanobeQuery) -> Result<Vec<Manga>> {
        self.get("/api/ranobe", &q).await
    }
    /// Выполняет `GET /api/ranobe/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn ranobe(&self, id: u64) -> Result<MangaDetails> {
        self.get(&format!("/api/ranobe/{id}"), &()).await
    }
    /// Выполняет `GET /api/ranobe/{id}/roles`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn ranobe_roles(&self, id: u64) -> Result<Vec<Role>> {
        self.get(&format!("/api/ranobe/{id}/roles"), &()).await
    }
    /// Выполняет `GET /api/ranobe/{id}/similar`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn similar_ranobe(&self, id: u64) -> Result<Vec<Manga>> {
        self.get(&format!("/api/ranobe/{id}/similar"), &()).await
    }
    /// Выполняет `GET /api/ranobe/{id}/related`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn related_ranobe(&self, id: u64) -> Result<Vec<Related>> {
        self.get(&format!("/api/ranobe/{id}/related"), &()).await
    }
    /// Выполняет `GET /api/ranobe/{id}/franchise`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn ranobe_franchise(&self, id: u64) -> Result<Franchise> {
        self.get(&format!("/api/ranobe/{id}/franchise"), &()).await
    }
    /// Выполняет `GET /api/ranobe/{id}/external_links`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn ranobe_external_links(&self, id: u64) -> Result<Vec<ExternalLink>> {
        self.get(&format!("/api/ranobe/{id}/external_links"), &())
            .await
    }
    /// Выполняет `GET /api/ranobe/{id}/topics`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn ranobe_topics(&self, id: u64, q: PageLimit30Query) -> Result<Vec<Topic>> {
        self.get(&format!("/api/ranobe/{id}/topics"), &q).await
    }

    // Клубы, комментарии, темы, рецензии.
    /// Выполняет `GET /api/clubs`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn list_clubs(&self, q: ListClubsQuery) -> Result<Vec<Club>> {
        self.get("/api/clubs", &q).await
    }
    /// Выполняет `GET /api/clubs/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn club(&self, id: u64) -> Result<ClubDetails> {
        self.get(&format!("/api/clubs/{id}"), &()).await
    }
    /// Выполняет `PATCH /api/clubs/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn patch_club(&self, id: u64, b: UpdateClubRequest) -> Result<ClubDetails> {
        self.patch_json(&format!("/api/clubs/{id}"), &b).await
    }
    /// Выполняет `PUT /api/clubs/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn put_club(&self, id: u64, b: UpdateClubRequest) -> Result<ClubDetails> {
        self.put_json(&format!("/api/clubs/{id}"), &b).await
    }
    /// Выполняет `GET /api/clubs/{id}/animes`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn club_animes(&self, id: u64, q: PageLimitQuery) -> Result<Vec<Anime>> {
        self.get(&format!("/api/clubs/{id}/animes"), &q).await
    }
    /// Выполняет `GET /api/clubs/{id}/mangas`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn club_mangas(&self, id: u64, q: PageLimitQuery) -> Result<Vec<Manga>> {
        self.get(&format!("/api/clubs/{id}/mangas"), &q).await
    }
    /// Выполняет `GET /api/clubs/{id}/ranobe`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn club_ranobe(&self, id: u64, q: PageLimitQuery) -> Result<Vec<Manga>> {
        self.get(&format!("/api/clubs/{id}/ranobe"), &q).await
    }
    /// Выполняет `GET /api/clubs/{id}/characters`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn club_characters(&self, id: u64, q: PageLimitQuery) -> Result<Vec<Character>> {
        self.get(&format!("/api/clubs/{id}/characters"), &q).await
    }
    /// Выполняет `GET /api/clubs/{id}/collections`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn club_collections(&self, id: u64, q: PageLimitQuery) -> Result<Vec<Topic>> {
        self.get(&format!("/api/clubs/{id}/collections"), &q).await
    }
    /// Выполняет `GET /api/clubs/{id}/clubs`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn club_related_clubs(&self, id: u64, q: PageLimitQuery) -> Result<Vec<Club>> {
        self.get(&format!("/api/clubs/{id}/clubs"), &q).await
    }
    /// Выполняет `GET /api/clubs/{id}/members`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn club_members(&self, id: u64, q: PageLimitQuery) -> Result<Vec<User>> {
        self.get(&format!("/api/clubs/{id}/members"), &q).await
    }
    /// Выполняет `GET /api/clubs/{id}/images`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn club_images(&self, id: u64, q: PageLimitQuery) -> Result<Vec<ClubImage>> {
        self.get(&format!("/api/clubs/{id}/images"), &q).await
    }
    /// Выполняет `POST /api/clubs/{id}/join`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn join_club(&self, id: u64) -> Result<()> {
        self.post_empty(&format!("/api/clubs/{id}/join")).await
    }
    /// Выполняет `POST /api/clubs/{id}/leave`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn leave_club(&self, id: u64) -> Result<()> {
        self.post_empty(&format!("/api/clubs/{id}/leave")).await
    }
    /// Выполняет `GET /api/forums`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn forums(&self) -> Result<Vec<Forum>> {
        self.get("/api/forums", &()).await
    }
    /// Выполняет `GET /api/comments`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn list_comments(&self, q: ListCommentsQuery) -> Result<Vec<Comment>> {
        self.get("/api/comments", &q).await
    }
    /// Выполняет `GET /api/comments/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn comment(&self, id: u64) -> Result<Comment> {
        self.get(&format!("/api/comments/{id}"), &()).await
    }
    /// Выполняет `POST /api/comments`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn create_comment(&self, b: CreateCommentRequest) -> Result<Comment> {
        self.post_json("/api/comments", &b).await
    }
    /// Выполняет `PATCH /api/comments/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn patch_comment(&self, id: u64, b: UpdateCommentRequest) -> Result<Comment> {
        self.patch_json(&format!("/api/comments/{id}"), &b).await
    }
    /// Выполняет `PUT /api/comments/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn put_comment(&self, id: u64, b: UpdateCommentRequest) -> Result<Comment> {
        self.put_json(&format!("/api/comments/{id}"), &b).await
    }
    /// Выполняет `DELETE /api/comments/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn delete_comment(&self, id: u64) -> Result<Notice> {
        self.delete_json(&format!("/api/comments/{id}")).await
    }
    /// Выполняет `GET /api/topics`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn list_topics(&self, q: ListTopicsQuery) -> Result<Vec<Topic>> {
        self.get("/api/topics", &q).await
    }
    /// Выполняет `GET /api/topics/updates`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn topic_updates(&self, q: PageLimit30Query) -> Result<Vec<TopicUpdate>> {
        self.get("/api/topics/updates", &q).await
    }
    /// Выполняет `GET /api/topics/hot`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn hot_topics(&self, limit: Option<u64>) -> Result<Vec<Topic>> {
        self.get("/api/topics/hot", &LimitQuery { limit }).await
    }
    /// Выполняет `GET /api/topics/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn topic(&self, id: u64) -> Result<Topic> {
        self.get(&format!("/api/topics/{id}"), &()).await
    }
    /// Выполняет `POST /api/topics`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn create_topic(&self, b: CreateTopicRequest) -> Result<Topic> {
        self.post_json("/api/topics", &b).await
    }
    /// Выполняет `PATCH /api/topics/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn patch_topic(&self, id: u64, b: UpdateTopicRequest) -> Result<Topic> {
        self.patch_json(&format!("/api/topics/{id}"), &b).await
    }
    /// Выполняет `PUT /api/topics/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn put_topic(&self, id: u64, b: UpdateTopicRequest) -> Result<Topic> {
        self.put_json(&format!("/api/topics/{id}"), &b).await
    }
    /// Выполняет `DELETE /api/topics/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn delete_topic(&self, id: u64) -> Result<Notice> {
        self.delete_json(&format!("/api/topics/{id}")).await
    }
    /// Выполняет `POST /api/reviews`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn create_review(&self, b: CreateReviewRequest) -> Result<Review> {
        self.post_json("/api/reviews", &b).await
    }
    /// Выполняет `PATCH /api/reviews/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn patch_review(&self, id: u64, b: UpdateReviewRequest) -> Result<Review> {
        self.patch_json(&format!("/api/reviews/{id}"), &b).await
    }
    /// Выполняет `PUT /api/reviews/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn put_review(&self, id: u64, b: UpdateReviewRequest) -> Result<Review> {
        self.put_json(&format!("/api/reviews/{id}"), &b).await
    }
    /// Выполняет `DELETE /api/reviews/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn delete_review(&self, id: u64) -> Result<Notice> {
        self.delete_json(&format!("/api/reviews/{id}")).await
    }

    // Социальные действия, диалоги и сообщения.
    /// Выполняет `POST /api/appears`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn mark_appeared(&self, b: AppearRequest) -> Result<()> {
        self.post_unit("/api/appears", Some(&b)).await
    }
    /// Выполняет `GET /api/bans`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn bans(&self) -> Result<Vec<Ban>> {
        self.get("/api/bans", &()).await
    }
    /// Выполняет `GET /api/dialogs`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn dialogs(&self) -> Result<Vec<Dialog>> {
        self.get("/api/dialogs", &()).await
    }
    /// Выполняет `GET /api/dialogs/{}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn dialog(&self, user: &str) -> Result<Vec<MessageWithParticipants>> {
        self.get(&format!("/api/dialogs/{}", path_segment(user)), &())
            .await
    }
    /// Выполняет `DELETE /api/dialogs/{}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn delete_dialog(&self, user: &str) -> Result<Notice> {
        self.delete_json(&format!("/api/dialogs/{}", path_segment(user)))
            .await
    }
    /// Выполняет `DELETE /api/favorites/{}/{}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn create_favorite(&self, t: FavouriteTarget) -> Result<SuccessNotice> {
        let mut p = format!("/api/favorites/{}/{}", t.linked_type, t.linked_id);
        if let Some(k) = t.kind {
            p.push('/');
            p.push_str(&k.to_string())
        };
        self.post_empty_json(&p).await
    }
    /// Выполняет `DELETE /api/favorites/{}/{}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn delete_favorite(&self, t: FavouriteTarget) -> Result<SuccessNotice> {
        self.delete_json(&format!("/api/favorites/{}/{}", t.linked_type, t.linked_id))
            .await
    }
    /// Выполняет `POST /api/favorites/{id}/reorder`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn reorder_favorite(&self, id: u64, new_index: Option<u64>) -> Result<()> {
        self.post_unit(
            &format!("/api/favorites/{id}/reorder"),
            Some(&ReorderFavoriteRequest { new_index }),
        )
        .await
    }
    /// Выполняет `POST /api/friends/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn add_friend(&self, id: u64) -> Result<Notice> {
        self.post_empty_json(&format!("/api/friends/{id}")).await
    }
    /// Выполняет `DELETE /api/friends/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn delete_friend(&self, id: u64) -> Result<Notice> {
        self.delete_json(&format!("/api/friends/{id}")).await
    }
    #[deprecated(note = "use v2 ignore_user")]
    /// Выполняет `POST /api/ignores/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn ignore_user_v1(&self, id: u64) -> Result<Notice> {
        self.post_empty_json(&format!("/api/ignores/{id}")).await
    }
    #[deprecated(note = "use v2 unignore_user")]
    /// Выполняет `DELETE /api/ignores/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn unignore_user_v1(&self, id: u64) -> Result<Notice> {
        self.delete_json(&format!("/api/ignores/{id}")).await
    }
    /// Выполняет `GET /api/messages/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn message(&self, id: u64) -> Result<MessageWithParticipants> {
        self.get(&format!("/api/messages/{id}"), &()).await
    }
    /// Выполняет `POST /api/messages`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn create_message(&self, b: CreateMessageRequest) -> Result<MessageWithParticipants> {
        self.post_json("/api/messages", &b).await
    }
    /// Выполняет `PATCH /api/messages/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn patch_message(
        &self,
        id: u64,
        b: UpdateMessageRequest,
    ) -> Result<MessageWithParticipants> {
        self.patch_json(&format!("/api/messages/{id}"), &b).await
    }
    /// Выполняет `PUT /api/messages/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn put_message(
        &self,
        id: u64,
        b: UpdateMessageRequest,
    ) -> Result<MessageWithParticipants> {
        self.put_json(&format!("/api/messages/{id}"), &b).await
    }
    /// Выполняет `DELETE /api/messages/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn delete_message(&self, id: u64) -> Result<()> {
        self.delete_unit(&format!("/api/messages/{id}")).await
    }
    /// Выполняет `POST /api/messages/mark_read`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn mark_messages_read(&self, b: MarkMessagesReadRequest) -> Result<()> {
        self.post_unit("/api/messages/mark_read", Some(&b)).await
    }
    /// Выполняет `POST /api/messages/read_all`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn read_all_messages(&self, b: MessageBulkRequest) -> Result<()> {
        self.post_unit("/api/messages/read_all", Some(&b)).await
    }
    /// Выполняет `POST /api/messages/delete_all`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn delete_all_messages(&self, b: MessageBulkRequest) -> Result<()> {
        self.post_unit("/api/messages/delete_all", Some(&b)).await
    }
    #[deprecated(note = "use v2 ignore_topic")]
    /// Выполняет `POST /api/topic_ignores`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn create_topic_ignore_v1(&self, b: CreateTopicIgnoreV1Request) -> Result<ToggleUrl> {
        self.post_json("/api/topic_ignores", &b).await
    }
    #[deprecated(note = "use v2 unignore_topic")]
    /// Выполняет `DELETE /api/topic_ignores/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn delete_topic_ignore_v1(&self, id: u64) -> Result<ToggleUrl> {
        self.delete_json(&format!("/api/topic_ignores/{id}")).await
    }

    // Пользователи и оценки.
    /// Выполняет `GET /api/users`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn list_users(&self, q: ListUsersQuery) -> Result<Vec<User>> {
        self.get("/api/users", &q).await
    }
    /// Выполняет `GET /api/users/{}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn user(&self, id: &str, is_nickname: bool) -> Result<UserDetails> {
        self.get(
            &format!("/api/users/{}", path_segment(id)),
            &IsNicknameQuery { is_nickname },
        )
        .await
    }
    /// Выполняет `GET /api/users/{}/info`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn user_info(&self, id: &str) -> Result<UserBrief> {
        self.get(&format!("/api/users/{}/info", path_segment(id)), &())
            .await
    }
    /// Выполняет `GET /api/users/whoami`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn whoami(&self) -> Result<UserBrief> {
        self.get("/api/users/whoami", &()).await
    }
    /// Выполняет `POST /api/users/sign_out`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn sign_out(&self) -> Result<String> {
        self.post_plain_text("/api/users/sign_out").await
    }
    /// Выполняет `GET /api/users/{id}/friends`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn user_friends(&self, id: u64, q: PageLimitQuery) -> Result<Vec<User>> {
        self.get(&format!("/api/users/{id}/friends"), &q).await
    }
    /// Выполняет `GET /api/users/{id}/clubs`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn user_clubs(&self, id: u64) -> Result<Vec<Club>> {
        self.get(&format!("/api/users/{id}/clubs"), &()).await
    }
    /// Выполняет `GET /api/users/{id}/anime_rates`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn user_anime_rates(
        &self,
        id: u64,
        q: UserAnimeRatesQuery,
    ) -> Result<Vec<UserRateWithTarget>> {
        self.get(&format!("/api/users/{id}/anime_rates"), &q).await
    }
    /// Выполняет `GET /api/users/{id}/manga_rates`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn user_manga_rates(
        &self,
        id: u64,
        q: UserMangaRatesQuery,
    ) -> Result<Vec<UserRateWithTarget>> {
        self.get(&format!("/api/users/{id}/manga_rates"), &q).await
    }
    /// Выполняет `GET /api/users/{id}/favourites`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn user_favourites(&self, id: u64) -> Result<Favourites> {
        self.get(&format!("/api/users/{id}/favourites"), &()).await
    }
    /// Выполняет `GET /api/users/{id}/messages`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn user_messages(
        &self,
        id: u64,
        q: UserMessagesQuery,
    ) -> Result<Vec<MessageWithParticipants>> {
        self.get(&format!("/api/users/{id}/messages"), &q).await
    }
    /// Выполняет `GET /api/users/{id}/unread_messages`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn unread_messages(&self, id: u64) -> Result<UnreadMessages> {
        self.get(&format!("/api/users/{id}/unread_messages"), &())
            .await
    }
    /// Выполняет `GET /api/users/{id}/history`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn user_history(&self, id: u64, q: UserHistoryQuery) -> Result<Vec<HistoryEntry>> {
        self.get(&format!("/api/users/{id}/history"), &q).await
    }
    /// Выполняет `GET /api/users/{id}/bans`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn user_bans(&self, id: u64) -> Result<Vec<Ban>> {
        self.get(&format!("/api/users/{id}/bans"), &()).await
    }
    #[deprecated(note = "prefer v2 user_rate_v2")]
    /// Выполняет `GET /api/user_rates/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn user_rate_v1(&self, id: u64) -> Result<UserRate> {
        self.get(&format!("/api/user_rates/{id}"), &()).await
    }
    #[deprecated(note = "prefer v2 create_user_rate_v2")]
    /// Выполняет `POST /api/user_rates`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn create_user_rate_v1(
        &self,
        b: CreateUserRateRequest,
    ) -> Result<UserRateWithTarget> {
        self.post_json("/api/user_rates", &b).await
    }
    #[deprecated(note = "prefer v2 patch_user_rate_v2")]
    /// Выполняет `PATCH /api/user_rates/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn patch_user_rate_v1(
        &self,
        id: u64,
        b: UpdateUserRateRequest,
    ) -> Result<UserRateWithTarget> {
        self.patch_json(&format!("/api/user_rates/{id}"), &b).await
    }
    #[deprecated(note = "prefer v2 put_user_rate_v2")]
    /// Выполняет `PUT /api/user_rates/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn put_user_rate_v1(
        &self,
        id: u64,
        b: UpdateUserRateRequest,
    ) -> Result<UserRateWithTarget> {
        self.put_json(&format!("/api/user_rates/{id}"), &b).await
    }
    #[deprecated(note = "prefer v2 increment_user_rate_v2")]
    /// Выполняет `POST /api/user_rates/{id}/increment`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn increment_user_rate_v1(&self, id: u64) -> Result<UserRateWithTarget> {
        self.post_empty_json(&format!("/api/user_rates/{id}/increment"))
            .await
    }
    #[deprecated(note = "prefer v2 delete_user_rate_v2")]
    /// Выполняет `DELETE /api/user_rates/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn delete_user_rate_v1(&self, id: u64) -> Result<()> {
        self.delete_unit(&format!("/api/user_rates/{id}")).await
    }
    /// Выполняет `DELETE /api/user_rates/{t}/cleanup`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn cleanup_user_rates(&self, t: UserRateListType) -> Result<Notice> {
        self.delete_json(&format!("/api/user_rates/{t}/cleanup"))
            .await
    }
    /// Выполняет `DELETE /api/user_rates/{t}/reset`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn reset_user_rate_scores(&self, t: UserRateListType) -> Result<Notice> {
        self.delete_json(&format!("/api/user_rates/{t}/reset"))
            .await
    }

    // Styles, video CRUD и upload.
    /// Выполняет `GET /api/styles/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn style(&self, id: u64) -> Result<Style> {
        self.get(&format!("/api/styles/{id}"), &()).await
    }
    /// Выполняет `POST /api/styles/preview`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn preview_style(&self, b: StylePreviewRequest) -> Result<Style> {
        self.post_json("/api/styles/preview", &b).await
    }
    /// Выполняет `POST /api/styles`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn create_style(&self, b: CreateStyleRequest) -> Result<Style> {
        self.post_json("/api/styles", &b).await
    }
    /// Выполняет `PATCH /api/styles/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn patch_style(&self, id: u64, b: UpdateStyleRequest) -> Result<Style> {
        self.patch_json(&format!("/api/styles/{id}"), &b).await
    }
    /// Выполняет `PUT /api/styles/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn put_style(&self, id: u64, b: UpdateStyleRequest) -> Result<Style> {
        self.put_json(&format!("/api/styles/{id}"), &b).await
    }
    /// Выполняет `POST /api/animes/{anime_id}/videos`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn create_anime_video(&self, anime_id: u64, b: CreateVideoRequest) -> Result<Video> {
        self.post_json(&format!("/api/animes/{anime_id}/videos"), &b)
            .await
    }
    /// Выполняет `DELETE /api/animes/{anime_id}/videos/{id}`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn delete_anime_video(&self, anime_id: u64, id: u64) -> Result<()> {
        self.delete_unit(&format!("/api/animes/{anime_id}/videos/{id}"))
            .await
    }
    /// Выполняет `POST /api/user_images`.
    ///
    /// Типы входных параметров и успешного ответа указаны в сигнатуре.
    /// См. также module-level документацию и endpoint matrix проекта для scope.
    pub async fn upload_user_image(&self, upload: UserImageUpload) -> Result<UploadedUserImage> {
        let boundary = "----shikimori-api-boundary";
        let filename = upload.filename.replace(['\r', '\n', '"'], "_");
        let mime = upload
            .content_type
            .unwrap_or_else(|| "application/octet-stream".into())
            .replace(['\r', '\n'], "");
        let mut body = Vec::new();
        if let Some(v) = upload.linked_type {
            body.extend_from_slice(format!("--{boundary}\r\nContent-Disposition: form-data; name=\"linked_type\"\r\n\r\n{}\r\n",v.replace(['\r','\n'],"")).as_bytes());
        }
        body.extend_from_slice(format!("--{boundary}\r\nContent-Disposition: form-data; name=\"image\"; filename=\"{filename}\"\r\nContent-Type: {mime}\r\n\r\n").as_bytes());
        body.extend_from_slice(&upload.bytes);
        body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());
        self.post_multipart(
            "/api/user_images",
            body,
            format!("multipart/form-data; boundary={boundary}"),
        )
        .await
    }
}
