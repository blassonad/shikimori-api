//! Методы Shikimori REST API v1.

use crate::{client::ShikimoriClient, encoding::path_segment, params::*, types::*, Result};

impl ShikimoriClient {
    // Каталоги и метаданные.
    pub async fn achievements(&self, user_id: u64) -> Result<Vec<Achievement>> {
        self.get("/api/achievements", &AchievementsQuery { user_id })
            .await
    }
    pub async fn list_animes(&self, q: ListAnimesQuery) -> Result<Vec<Anime>> {
        self.get("/api/animes", &q).await
    }
    pub async fn anime(&self, id: u64) -> Result<AnimeDetails> {
        self.get(&format!("/api/animes/{id}"), &()).await
    }
    pub async fn anime_roles(&self, id: u64) -> Result<Vec<Role>> {
        self.get(&format!("/api/animes/{id}/roles"), &()).await
    }
    pub async fn similar_animes(&self, id: u64) -> Result<Vec<Anime>> {
        self.get(&format!("/api/animes/{id}/similar"), &()).await
    }
    pub async fn related_animes(&self, id: u64) -> Result<Vec<Related>> {
        self.get(&format!("/api/animes/{id}/related"), &()).await
    }
    pub async fn anime_screenshots(&self, id: u64) -> Result<Vec<Screenshot>> {
        self.get(&format!("/api/animes/{id}/screenshots"), &())
            .await
    }
    pub async fn anime_videos(&self, id: u64) -> Result<Vec<Video>> {
        self.get(&format!("/api/animes/{id}/videos"), &()).await
    }
    pub async fn anime_franchise(&self, id: u64) -> Result<Franchise> {
        self.get(&format!("/api/animes/{id}/franchise"), &()).await
    }
    pub async fn anime_external_links(&self, id: u64) -> Result<Vec<ExternalLink>> {
        self.get(&format!("/api/animes/{id}/external_links"), &())
            .await
    }
    #[deprecated(note = "use list_animes with ListAnimesQuery.search")]
    pub async fn search_animes(&self, q: ListAnimesQuery) -> Result<Vec<Anime>> {
        self.get("/api/animes/search", &q).await
    }
    pub async fn anime_topics(&self, id: u64, q: PageLimit30Query) -> Result<Vec<Topic>> {
        self.get(&format!("/api/animes/{id}/topics"), &q).await
    }
    pub async fn calendar(&self, censored: Option<bool>) -> Result<Vec<CalendarEntry>> {
        self.get("/api/calendar", &CensoredQuery { censored }).await
    }
    pub async fn character(&self, id: u64) -> Result<CharacterDetails> {
        self.get(&format!("/api/characters/{id}"), &()).await
    }
    pub async fn search_characters(&self, search: Option<String>) -> Result<Vec<Character>> {
        self.get(
            "/api/characters/search",
            &SearchQuery { search, kind: None },
        )
        .await
    }
    pub async fn genres(&self) -> Result<Vec<Genre>> {
        self.get("/api/genres", &()).await
    }
    pub async fn person(&self, id: u64) -> Result<PersonDetails> {
        self.get(&format!("/api/people/{id}"), &()).await
    }
    pub async fn search_people(
        &self,
        search: Option<String>,
        kind: Option<PersonKind>,
    ) -> Result<Vec<Person>> {
        self.get("/api/people/search", &SearchQuery { search, kind })
            .await
    }
    pub async fn publishers(&self) -> Result<Vec<Publisher>> {
        self.get("/api/publishers", &()).await
    }
    pub async fn studios(&self) -> Result<Vec<Studio>> {
        self.get("/api/studios", &()).await
    }
    pub async fn anime_constants(&self) -> Result<AnimeConstants> {
        self.get("/api/constants/anime", &()).await
    }
    pub async fn manga_constants(&self) -> Result<MangaConstants> {
        self.get("/api/constants/manga", &()).await
    }
    pub async fn user_rate_constants(&self) -> Result<UserRateConstants> {
        self.get("/api/constants/user_rate", &()).await
    }
    pub async fn club_constants(&self) -> Result<ClubConstants> {
        self.get("/api/constants/club", &()).await
    }
    pub async fn smileys(&self) -> Result<Vec<Smiley>> {
        self.get("/api/constants/smileys", &()).await
    }
    pub async fn active_users(&self) -> Result<Vec<UserId>> {
        self.get("/api/stats/active_users", &()).await
    }

    pub async fn list_mangas(&self, q: ListMangasQuery) -> Result<Vec<Manga>> {
        self.get("/api/mangas", &q).await
    }
    pub async fn manga(&self, id: u64) -> Result<MangaDetails> {
        self.get(&format!("/api/mangas/{id}"), &()).await
    }
    pub async fn manga_roles(&self, id: u64) -> Result<Vec<Role>> {
        self.get(&format!("/api/mangas/{id}/roles"), &()).await
    }
    pub async fn similar_mangas(&self, id: u64) -> Result<Vec<Manga>> {
        self.get(&format!("/api/mangas/{id}/similar"), &()).await
    }
    pub async fn related_mangas(&self, id: u64) -> Result<Vec<Related>> {
        self.get(&format!("/api/mangas/{id}/related"), &()).await
    }
    pub async fn manga_franchise(&self, id: u64) -> Result<Franchise> {
        self.get(&format!("/api/mangas/{id}/franchise"), &()).await
    }
    pub async fn manga_external_links(&self, id: u64) -> Result<Vec<ExternalLink>> {
        self.get(&format!("/api/mangas/{id}/external_links"), &())
            .await
    }
    #[deprecated(note = "use list_mangas with ListMangasQuery.search")]
    pub async fn search_mangas(&self, q: ListMangasQuery) -> Result<Vec<Manga>> {
        self.get("/api/mangas/search", &q).await
    }
    pub async fn manga_topics(&self, id: u64, q: PageLimit30Query) -> Result<Vec<Topic>> {
        self.get(&format!("/api/mangas/{id}/topics"), &q).await
    }
    pub async fn list_ranobe(&self, q: ListRanobeQuery) -> Result<Vec<Manga>> {
        self.get("/api/ranobe", &q).await
    }
    pub async fn ranobe(&self, id: u64) -> Result<MangaDetails> {
        self.get(&format!("/api/ranobe/{id}"), &()).await
    }
    pub async fn ranobe_roles(&self, id: u64) -> Result<Vec<Role>> {
        self.get(&format!("/api/ranobe/{id}/roles"), &()).await
    }
    pub async fn similar_ranobe(&self, id: u64) -> Result<Vec<Manga>> {
        self.get(&format!("/api/ranobe/{id}/similar"), &()).await
    }
    pub async fn related_ranobe(&self, id: u64) -> Result<Vec<Related>> {
        self.get(&format!("/api/ranobe/{id}/related"), &()).await
    }
    pub async fn ranobe_franchise(&self, id: u64) -> Result<Franchise> {
        self.get(&format!("/api/ranobe/{id}/franchise"), &()).await
    }
    pub async fn ranobe_external_links(&self, id: u64) -> Result<Vec<ExternalLink>> {
        self.get(&format!("/api/ranobe/{id}/external_links"), &())
            .await
    }
    pub async fn ranobe_topics(&self, id: u64, q: PageLimit30Query) -> Result<Vec<Topic>> {
        self.get(&format!("/api/ranobe/{id}/topics"), &q).await
    }

    // Клубы, комментарии, темы, рецензии.
    pub async fn list_clubs(&self, q: ListClubsQuery) -> Result<Vec<Club>> {
        self.get("/api/clubs", &q).await
    }
    pub async fn club(&self, id: u64) -> Result<ClubDetails> {
        self.get(&format!("/api/clubs/{id}"), &()).await
    }
    pub async fn patch_club(&self, id: u64, b: UpdateClubRequest) -> Result<ClubDetails> {
        self.patch_json(&format!("/api/clubs/{id}"), &b).await
    }
    pub async fn put_club(&self, id: u64, b: UpdateClubRequest) -> Result<ClubDetails> {
        self.put_json(&format!("/api/clubs/{id}"), &b).await
    }
    pub async fn club_animes(&self, id: u64, q: PageLimitQuery) -> Result<Vec<Anime>> {
        self.get(&format!("/api/clubs/{id}/animes"), &q).await
    }
    pub async fn club_mangas(&self, id: u64, q: PageLimitQuery) -> Result<Vec<Manga>> {
        self.get(&format!("/api/clubs/{id}/mangas"), &q).await
    }
    pub async fn club_ranobe(&self, id: u64, q: PageLimitQuery) -> Result<Vec<Manga>> {
        self.get(&format!("/api/clubs/{id}/ranobe"), &q).await
    }
    pub async fn club_characters(&self, id: u64, q: PageLimitQuery) -> Result<Vec<Character>> {
        self.get(&format!("/api/clubs/{id}/characters"), &q).await
    }
    pub async fn club_collections(&self, id: u64, q: PageLimitQuery) -> Result<Vec<Topic>> {
        self.get(&format!("/api/clubs/{id}/collections"), &q).await
    }
    pub async fn club_related_clubs(&self, id: u64, q: PageLimitQuery) -> Result<Vec<Club>> {
        self.get(&format!("/api/clubs/{id}/clubs"), &q).await
    }
    pub async fn club_members(&self, id: u64, q: PageLimitQuery) -> Result<Vec<User>> {
        self.get(&format!("/api/clubs/{id}/members"), &q).await
    }
    pub async fn club_images(&self, id: u64, q: PageLimitQuery) -> Result<Vec<ClubImage>> {
        self.get(&format!("/api/clubs/{id}/images"), &q).await
    }
    pub async fn join_club(&self, id: u64) -> Result<()> {
        self.post_empty(&format!("/api/clubs/{id}/join")).await
    }
    pub async fn leave_club(&self, id: u64) -> Result<()> {
        self.post_empty(&format!("/api/clubs/{id}/leave")).await
    }
    pub async fn forums(&self) -> Result<Vec<Forum>> {
        self.get("/api/forums", &()).await
    }
    pub async fn list_comments(&self, q: ListCommentsQuery) -> Result<Vec<Comment>> {
        self.get("/api/comments", &q).await
    }
    pub async fn comment(&self, id: u64) -> Result<Comment> {
        self.get(&format!("/api/comments/{id}"), &()).await
    }
    pub async fn create_comment(&self, b: CreateCommentRequest) -> Result<Comment> {
        self.post_json("/api/comments", &b).await
    }
    pub async fn patch_comment(&self, id: u64, b: UpdateCommentRequest) -> Result<Comment> {
        self.patch_json(&format!("/api/comments/{id}"), &b).await
    }
    pub async fn put_comment(&self, id: u64, b: UpdateCommentRequest) -> Result<Comment> {
        self.put_json(&format!("/api/comments/{id}"), &b).await
    }
    pub async fn delete_comment(&self, id: u64) -> Result<Notice> {
        self.delete_json(&format!("/api/comments/{id}")).await
    }
    pub async fn list_topics(&self, q: ListTopicsQuery) -> Result<Vec<Topic>> {
        self.get("/api/topics", &q).await
    }
    pub async fn topic_updates(&self, q: PageLimit30Query) -> Result<Vec<TopicUpdate>> {
        self.get("/api/topics/updates", &q).await
    }
    pub async fn hot_topics(&self, limit: Option<u64>) -> Result<Vec<Topic>> {
        self.get("/api/topics/hot", &LimitQuery { limit }).await
    }
    pub async fn topic(&self, id: u64) -> Result<Topic> {
        self.get(&format!("/api/topics/{id}"), &()).await
    }
    pub async fn create_topic(&self, b: CreateTopicRequest) -> Result<Topic> {
        self.post_json("/api/topics", &b).await
    }
    pub async fn patch_topic(&self, id: u64, b: UpdateTopicRequest) -> Result<Topic> {
        self.patch_json(&format!("/api/topics/{id}"), &b).await
    }
    pub async fn put_topic(&self, id: u64, b: UpdateTopicRequest) -> Result<Topic> {
        self.put_json(&format!("/api/topics/{id}"), &b).await
    }
    pub async fn delete_topic(&self, id: u64) -> Result<Notice> {
        self.delete_json(&format!("/api/topics/{id}")).await
    }
    pub async fn create_review(&self, b: CreateReviewRequest) -> Result<Review> {
        self.post_json("/api/reviews", &b).await
    }
    pub async fn patch_review(&self, id: u64, b: UpdateReviewRequest) -> Result<Review> {
        self.patch_json(&format!("/api/reviews/{id}"), &b).await
    }
    pub async fn put_review(&self, id: u64, b: UpdateReviewRequest) -> Result<Review> {
        self.put_json(&format!("/api/reviews/{id}"), &b).await
    }
    pub async fn delete_review(&self, id: u64) -> Result<Notice> {
        self.delete_json(&format!("/api/reviews/{id}")).await
    }

    // Социальные действия, диалоги и сообщения.
    pub async fn mark_appeared(&self, b: AppearRequest) -> Result<()> {
        self.post_unit("/api/appears", Some(&b)).await
    }
    pub async fn bans(&self) -> Result<Vec<Ban>> {
        self.get("/api/bans", &()).await
    }
    pub async fn dialogs(&self) -> Result<Vec<Dialog>> {
        self.get("/api/dialogs", &()).await
    }
    pub async fn dialog(&self, user: &str) -> Result<Vec<MessageWithParticipants>> {
        self.get(&format!("/api/dialogs/{}", path_segment(user)), &())
            .await
    }
    pub async fn delete_dialog(&self, user: &str) -> Result<Notice> {
        self.delete_json(&format!("/api/dialogs/{}", path_segment(user)))
            .await
    }
    pub async fn create_favorite(&self, t: FavouriteTarget) -> Result<SuccessNotice> {
        let mut p = format!("/api/favorites/{}/{}", t.linked_type, t.linked_id);
        if let Some(k) = t.kind {
            p.push('/');
            p.push_str(&k.to_string())
        };
        self.post_empty_json(&p).await
    }
    pub async fn delete_favorite(&self, t: FavouriteTarget) -> Result<SuccessNotice> {
        self.delete_json(&format!("/api/favorites/{}/{}", t.linked_type, t.linked_id))
            .await
    }
    pub async fn reorder_favorite(&self, id: u64, new_index: Option<u64>) -> Result<()> {
        self.post_unit(
            &format!("/api/favorites/{id}/reorder"),
            Some(&ReorderFavoriteRequest { new_index }),
        )
        .await
    }
    pub async fn add_friend(&self, id: u64) -> Result<Notice> {
        self.post_empty_json(&format!("/api/friends/{id}")).await
    }
    pub async fn delete_friend(&self, id: u64) -> Result<Notice> {
        self.delete_json(&format!("/api/friends/{id}")).await
    }
    #[deprecated(note = "use v2 ignore_user")]
    pub async fn ignore_user_v1(&self, id: u64) -> Result<Notice> {
        self.post_empty_json(&format!("/api/ignores/{id}")).await
    }
    #[deprecated(note = "use v2 unignore_user")]
    pub async fn unignore_user_v1(&self, id: u64) -> Result<Notice> {
        self.delete_json(&format!("/api/ignores/{id}")).await
    }
    pub async fn message(&self, id: u64) -> Result<MessageWithParticipants> {
        self.get(&format!("/api/messages/{id}"), &()).await
    }
    pub async fn create_message(&self, b: CreateMessageRequest) -> Result<MessageWithParticipants> {
        self.post_json("/api/messages", &b).await
    }
    pub async fn patch_message(
        &self,
        id: u64,
        b: UpdateMessageRequest,
    ) -> Result<MessageWithParticipants> {
        self.patch_json(&format!("/api/messages/{id}"), &b).await
    }
    pub async fn put_message(
        &self,
        id: u64,
        b: UpdateMessageRequest,
    ) -> Result<MessageWithParticipants> {
        self.put_json(&format!("/api/messages/{id}"), &b).await
    }
    pub async fn delete_message(&self, id: u64) -> Result<()> {
        self.delete_unit(&format!("/api/messages/{id}")).await
    }
    pub async fn mark_messages_read(&self, b: MarkMessagesReadRequest) -> Result<()> {
        self.post_unit("/api/messages/mark_read", Some(&b)).await
    }
    pub async fn read_all_messages(&self, b: MessageBulkRequest) -> Result<()> {
        self.post_unit("/api/messages/read_all", Some(&b)).await
    }
    pub async fn delete_all_messages(&self, b: MessageBulkRequest) -> Result<()> {
        self.post_unit("/api/messages/delete_all", Some(&b)).await
    }
    #[deprecated(note = "use v2 ignore_topic")]
    pub async fn create_topic_ignore_v1(&self, b: CreateTopicIgnoreV1Request) -> Result<ToggleUrl> {
        self.post_json("/api/topic_ignores", &b).await
    }
    #[deprecated(note = "use v2 unignore_topic")]
    pub async fn delete_topic_ignore_v1(&self, id: u64) -> Result<ToggleUrl> {
        self.delete_json(&format!("/api/topic_ignores/{id}")).await
    }

    // Пользователи и оценки.
    pub async fn list_users(&self, q: ListUsersQuery) -> Result<Vec<User>> {
        self.get("/api/users", &q).await
    }
    pub async fn user(&self, id: &str, is_nickname: bool) -> Result<UserDetails> {
        self.get(
            &format!("/api/users/{}", path_segment(id)),
            &IsNicknameQuery { is_nickname },
        )
        .await
    }
    pub async fn user_info(&self, id: &str) -> Result<UserBrief> {
        self.get(&format!("/api/users/{}/info", path_segment(id)), &())
            .await
    }
    pub async fn whoami(&self) -> Result<UserBrief> {
        self.get("/api/users/whoami", &()).await
    }
    pub async fn sign_out(&self) -> Result<String> {
        self.post_plain_text("/api/users/sign_out").await
    }
    pub async fn user_friends(&self, id: u64, q: PageLimitQuery) -> Result<Vec<User>> {
        self.get(&format!("/api/users/{id}/friends"), &q).await
    }
    pub async fn user_clubs(&self, id: u64) -> Result<Vec<Club>> {
        self.get(&format!("/api/users/{id}/clubs"), &()).await
    }
    pub async fn user_anime_rates(
        &self,
        id: u64,
        q: UserAnimeRatesQuery,
    ) -> Result<Vec<UserRateWithTarget>> {
        self.get(&format!("/api/users/{id}/anime_rates"), &q).await
    }
    pub async fn user_manga_rates(
        &self,
        id: u64,
        q: UserMangaRatesQuery,
    ) -> Result<Vec<UserRateWithTarget>> {
        self.get(&format!("/api/users/{id}/manga_rates"), &q).await
    }
    pub async fn user_favourites(&self, id: u64) -> Result<Favourites> {
        self.get(&format!("/api/users/{id}/favourites"), &()).await
    }
    pub async fn user_messages(
        &self,
        id: u64,
        q: UserMessagesQuery,
    ) -> Result<Vec<MessageWithParticipants>> {
        self.get(&format!("/api/users/{id}/messages"), &q).await
    }
    pub async fn unread_messages(&self, id: u64) -> Result<UnreadMessages> {
        self.get(&format!("/api/users/{id}/unread_messages"), &())
            .await
    }
    pub async fn user_history(&self, id: u64, q: UserHistoryQuery) -> Result<Vec<HistoryEntry>> {
        self.get(&format!("/api/users/{id}/history"), &q).await
    }
    pub async fn user_bans(&self, id: u64) -> Result<Vec<Ban>> {
        self.get(&format!("/api/users/{id}/bans"), &()).await
    }
    #[deprecated(note = "prefer v2 user_rate_v2")]
    pub async fn user_rate_v1(&self, id: u64) -> Result<UserRate> {
        self.get(&format!("/api/user_rates/{id}"), &()).await
    }
    #[deprecated(note = "prefer v2 create_user_rate_v2")]
    pub async fn create_user_rate_v1(
        &self,
        b: CreateUserRateRequest,
    ) -> Result<UserRateWithTarget> {
        self.post_json("/api/user_rates", &b).await
    }
    #[deprecated(note = "prefer v2 patch_user_rate_v2")]
    pub async fn patch_user_rate_v1(
        &self,
        id: u64,
        b: UpdateUserRateRequest,
    ) -> Result<UserRateWithTarget> {
        self.patch_json(&format!("/api/user_rates/{id}"), &b).await
    }
    #[deprecated(note = "prefer v2 put_user_rate_v2")]
    pub async fn put_user_rate_v1(
        &self,
        id: u64,
        b: UpdateUserRateRequest,
    ) -> Result<UserRateWithTarget> {
        self.put_json(&format!("/api/user_rates/{id}"), &b).await
    }
    #[deprecated(note = "prefer v2 increment_user_rate_v2")]
    pub async fn increment_user_rate_v1(&self, id: u64) -> Result<UserRateWithTarget> {
        self.post_empty_json(&format!("/api/user_rates/{id}/increment"))
            .await
    }
    #[deprecated(note = "prefer v2 delete_user_rate_v2")]
    pub async fn delete_user_rate_v1(&self, id: u64) -> Result<()> {
        self.delete_unit(&format!("/api/user_rates/{id}")).await
    }
    pub async fn cleanup_user_rates(&self, t: UserRateListType) -> Result<Notice> {
        self.delete_json(&format!("/api/user_rates/{t}/cleanup"))
            .await
    }
    pub async fn reset_user_rate_scores(&self, t: UserRateListType) -> Result<Notice> {
        self.delete_json(&format!("/api/user_rates/{t}/reset"))
            .await
    }

    // Styles, video CRUD и upload.
    pub async fn style(&self, id: u64) -> Result<Style> {
        self.get(&format!("/api/styles/{id}"), &()).await
    }
    pub async fn preview_style(&self, b: StylePreviewRequest) -> Result<Style> {
        self.post_json("/api/styles/preview", &b).await
    }
    pub async fn create_style(&self, b: CreateStyleRequest) -> Result<Style> {
        self.post_json("/api/styles", &b).await
    }
    pub async fn patch_style(&self, id: u64, b: UpdateStyleRequest) -> Result<Style> {
        self.patch_json(&format!("/api/styles/{id}"), &b).await
    }
    pub async fn put_style(&self, id: u64, b: UpdateStyleRequest) -> Result<Style> {
        self.put_json(&format!("/api/styles/{id}"), &b).await
    }
    pub async fn create_anime_video(&self, anime_id: u64, b: CreateVideoRequest) -> Result<Video> {
        self.post_json(&format!("/api/animes/{anime_id}/videos"), &b)
            .await
    }
    pub async fn delete_anime_video(&self, anime_id: u64, id: u64) -> Result<()> {
        self.delete_unit(&format!("/api/animes/{anime_id}/videos/{id}"))
            .await
    }
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
