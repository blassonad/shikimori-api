//! Методы Shikimori REST API v2.

use crate::{client::ShikimoriClient, params::*, types::*, Result};

impl ShikimoriClient {
    /// Ignores a topic through `POST /api/v2/topics/:topic_id/ignore`.
    pub async fn ignore_topic(&self, topic_id: u64) -> Result<IgnoreState> {
        self.post_empty_json(&format!("/api/v2/topics/{topic_id}/ignore"))
            .await
    }

    /// Removes a topic ignore through `DELETE /api/v2/topics/:topic_id/ignore`.
    pub async fn unignore_topic(&self, topic_id: u64) -> Result<IgnoreState> {
        self.delete_json(&format!("/api/v2/topics/{topic_id}/ignore"))
            .await
    }

    /// Ignores a user through `POST /api/v2/users/:user_id/ignore`.
    pub async fn ignore_user(&self, user_id: u64) -> Result<IgnoreState> {
        self.post_empty_json(&format!("/api/v2/users/{user_id}/ignore"))
            .await
    }

    /// Removes a user ignore through `DELETE /api/v2/users/:user_id/ignore`.
    pub async fn unignore_user(&self, user_id: u64) -> Result<IgnoreState> {
        self.delete_json(&format!("/api/v2/users/{user_id}/ignore"))
            .await
    }

    /// Asks moderators to mark a required comment as off-topic.
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

    /// Asks moderators to convert a comment/topic to a review.
    ///
    /// The detailed official route is `convert_review`; the v2 index labels it
    /// as `review`. The detailed route is used deliberately.
    pub async fn request_review_conversion(&self, body: AbuseRequestInput) -> Result<()> {
        self.post_unit("/api/v2/abuse_requests/convert_review", Some(&body))
            .await
    }

    /// Sends an abuse report to moderators.
    pub async fn request_abuse(&self, body: AbuseRequestInput) -> Result<()> {
        self.post_unit("/api/v2/abuse_requests/abuse", Some(&body))
            .await
    }

    /// Sends a spoiler report to moderators.
    pub async fn request_spoiler_review(&self, body: AbuseRequestInput) -> Result<()> {
        self.post_unit("/api/v2/abuse_requests/spoiler", Some(&body))
            .await
    }

    /// Notifies Shikimori about an anime episode release.
    ///
    /// The `token` in the typed body is the endpoint's documented private token,
    /// not an OAuth flow implemented by this crate.
    pub async fn create_episode_notification(
        &self,
        body: EpisodeNotificationRequest,
    ) -> Result<EpisodeNotification> {
        self.post_json("/api/v2/episode_notifications", &body).await
    }

    /// Lists v2 user rates with optional user, target, status and pagination filters.
    pub async fn list_user_rates_v2(&self, query: ListUserRatesV2Query) -> Result<Vec<UserRate>> {
        self.get("/api/v2/user_rates", &query).await
    }

    /// Gets one v2 user rate.
    pub async fn user_rate_v2(&self, id: u64) -> Result<UserRate> {
        self.get(&format!("/api/v2/user_rates/{id}"), &()).await
    }

    /// Creates a v2 user rate.
    pub async fn create_user_rate_v2(&self, body: CreateUserRateRequest) -> Result<UserRate> {
        self.post_json("/api/v2/user_rates", &body).await
    }

    /// Partially updates a v2 user rate.
    pub async fn patch_user_rate_v2(
        &self,
        id: u64,
        body: UpdateUserRateRequest,
    ) -> Result<UserRate> {
        self.patch_json(&format!("/api/v2/user_rates/{id}"), &body)
            .await
    }

    /// Replaces/updates a v2 user rate.
    pub async fn put_user_rate_v2(&self, id: u64, body: UpdateUserRateRequest) -> Result<UserRate> {
        self.put_json(&format!("/api/v2/user_rates/{id}"), &body)
            .await
    }

    /// Increments episode or chapter progress by one through the v2 route.
    pub async fn increment_user_rate_v2(&self, id: u64) -> Result<UserRate> {
        self.post_empty_json(&format!("/api/v2/user_rates/{id}/increment"))
            .await
    }

    /// Deletes a v2 user rate. The documented successful status is 204.
    pub async fn delete_user_rate_v2(&self, id: u64) -> Result<()> {
        self.delete_unit(&format!("/api/v2/user_rates/{id}")).await
    }
}
