//! REST-методы Shikimori API v2.
//!
//! В v2 собраны action-oriented routes, которые заменяют часть legacy v1 API:
//! ignore/unignore user и topic, moderation abuse requests, episode release
//! notifications и актуальный CRUD user rates. Все methods принадлежат
//! [`ShikimoriClient`] и возвращают [`Result<T>`].
//!
//! ## Переход с v1
//!
//! - Используйте [`ShikimoriClient::ignore_user`] и
//!   [`ShikimoriClient::unignore_user`] вместо v1 ignore routes.
//! - Используйте [`ShikimoriClient::ignore_topic`] и
//!   [`ShikimoriClient::unignore_topic`] вместо legacy topic ignores.
//! - Используйте `*_user_rate_v2` methods вместо deprecated CRUD v1 user rates.
//!
//! ## Особые контракты
//!
//! `create_episode_notification` принимает отдельный private endpoint token в
//! [`EpisodeNotificationRequest::token`]. Это **не** OAuth flow и не заменяет
//! bearer token, который применяется к обычным защищённым REST-вызовам.
//!
//! Для review conversion crate преднамеренно использует documented detail route
//! `/api/v2/abuse_requests/convert_review`: v2 index называет его `review`, но
//! detailed endpoint specification указывает `convert_review`.
//!
//! Полная HTTP-матрица: <https://github.com/blassonad/shikimori-api/blob/feat/complete-rest-client/docs/endpoint-matrix.md>.

use crate::{client::ShikimoriClient, params::*, types::*, Result};

impl ShikimoriClient {
    /// Выполняет `POST /api/v2/topics/{topic_id}/ignore`.
    ///
    /// Создаёт ignore state текущего авторизованного пользователя для topic и
    /// возвращает обновлённый [`IgnoreState`].
    pub async fn ignore_topic(&self, topic_id: u64) -> Result<IgnoreState> {
        self.post_empty_json(&format!("/api/v2/topics/{topic_id}/ignore"))
            .await
    }

    /// Выполняет `DELETE /api/v2/topics/{topic_id}/ignore`.
    ///
    /// Снимает ignore state текущего авторизованного пользователя для topic.
    pub async fn unignore_topic(&self, topic_id: u64) -> Result<IgnoreState> {
        self.delete_json(&format!("/api/v2/topics/{topic_id}/ignore"))
            .await
    }

    /// Выполняет `POST /api/v2/users/{user_id}/ignore`.
    ///
    /// Создаёт ignore state для user и заменяет deprecated v1 ignore route.
    pub async fn ignore_user(&self, user_id: u64) -> Result<IgnoreState> {
        self.post_empty_json(&format!("/api/v2/users/{user_id}/ignore"))
            .await
    }

    /// Выполняет `DELETE /api/v2/users/{user_id}/ignore`.
    ///
    /// Снимает ignore state и заменяет deprecated v1 unignore route.
    pub async fn unignore_user(&self, user_id: u64) -> Result<IgnoreState> {
        self.delete_json(&format!("/api/v2/users/{user_id}/ignore"))
            .await
    }

    /// Выполняет `POST /api/v2/abuse_requests/offtopic`.
    ///
    /// Создаёт moderation request с обязательным `comment_id` для отметки
    /// comment как off-topic.
    pub async fn request_offtopic(&self, comment_id: u64) -> Result<AbuseRequest> {
        self.post_json(
            "/api/v2/abuse_requests/offtopic",
            &AbuseRequestInput {
                comment_id: Some(comment_id),
                topic_id: None,
                reason: None,
            },
        )
        .await
    }

    /// Выполняет `POST /api/v2/abuse_requests/convert_review`.
    ///
    /// Просит moderators преобразовать comment/topic в review. V2 index
    /// называет действие `review`, но detailed specification фиксирует
    /// `convert_review`; клиент намеренно использует detailed route.
    pub async fn request_review_conversion(&self, body: AbuseRequestInput) -> Result<()> {
        self.post_unit("/api/v2/abuse_requests/convert_review", Some(&body))
            .await
    }

    /// Выполняет `POST /api/v2/abuse_requests/abuse`.
    ///
    /// Отправляет moderation abuse request с данными [`AbuseRequestInput`].
    pub async fn request_abuse(&self, body: AbuseRequestInput) -> Result<()> {
        self.post_unit("/api/v2/abuse_requests/abuse", Some(&body))
            .await
    }

    /// Выполняет `POST /api/v2/abuse_requests/spoiler`.
    ///
    /// Отправляет moderation spoiler request с данными [`AbuseRequestInput`].
    pub async fn request_spoiler_review(&self, body: AbuseRequestInput) -> Result<()> {
        self.post_unit("/api/v2/abuse_requests/spoiler", Some(&body))
            .await
    }

    /// Выполняет `POST /api/v2/episode_notifications`.
    ///
    /// Уведомляет Shikimori о выпуске anime episode. Поле
    /// [`EpisodeNotificationRequest::token`] — документированный private token
    /// endpoint-а, а не OAuth flow, реализуемый crate.
    pub async fn create_episode_notification(
        &self,
        body: EpisodeNotificationRequest,
    ) -> Result<EpisodeNotification> {
        self.post_json("/api/v2/episode_notifications", &body).await
    }

    /// Выполняет `GET /api/v2/user_rates`.
    ///
    /// Возвращает v2 user rates с optional user, target, status и pagination
    /// filters из [`ListUserRatesV2Query`].
    pub async fn list_user_rates_v2(&self, query: ListUserRatesV2Query) -> Result<Vec<UserRate>> {
        self.get("/api/v2/user_rates", &query).await
    }

    /// Выполняет `GET /api/v2/user_rates/{id}` и возвращает одну v2 user rate.
    pub async fn user_rate_v2(&self, id: u64) -> Result<UserRate> {
        self.get(&format!("/api/v2/user_rates/{id}"), &()).await
    }

    /// Выполняет `POST /api/v2/user_rates`.
    ///
    /// Создаёт v2 user rate из nested JSON [`CreateUserRateRequest`].
    pub async fn create_user_rate_v2(&self, body: CreateUserRateRequest) -> Result<UserRate> {
        self.post_json("/api/v2/user_rates", &body).await
    }

    /// Выполняет `PATCH /api/v2/user_rates/{id}`.
    ///
    /// Обновляет только переданные optional fields из [`UpdateUserRateRequest`].
    pub async fn patch_user_rate_v2(
        &self,
        id: u64,
        body: UpdateUserRateRequest,
    ) -> Result<UserRate> {
        self.patch_json(&format!("/api/v2/user_rates/{id}"), &body)
            .await
    }

    /// Выполняет `PUT /api/v2/user_rates/{id}` и обновляет v2 user rate.
    pub async fn put_user_rate_v2(&self, id: u64, body: UpdateUserRateRequest) -> Result<UserRate> {
        self.put_json(&format!("/api/v2/user_rates/{id}"), &body)
            .await
    }

    /// Выполняет `POST /api/v2/user_rates/{id}/increment`.
    ///
    /// Увеличивает episode либо chapter progress на единицу согласно target type.
    pub async fn increment_user_rate_v2(&self, id: u64) -> Result<UserRate> {
        self.post_empty_json(&format!("/api/v2/user_rates/{id}/increment"))
            .await
    }

    /// Выполняет `DELETE /api/v2/user_rates/{id}`.
    ///
    /// Удаляет v2 user rate; документированный успешный status — `204 No Content`.
    pub async fn delete_user_rate_v2(&self, id: u64) -> Result<()> {
        self.delete_unit(&format!("/api/v2/user_rates/{id}")).await
    }
}
