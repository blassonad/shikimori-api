use shikimori_api::types::{Anime, EpisodeNotification, UserRate};
use shikimori_api::{CommentableType, EpisodeNotificationRequest, NewEpisodeNotification};
use shikimori_api::{CreateCommentRequest, NewComment, RateTargetType, UserRateStatus};

#[test]
fn parses_documented_compact_anime_fixture() {
    let mut fixture = br#"{
        "id": 56,
        "name": "Test",
        "russian": "Test RU",
        "image": {"original":"/o.jpg","preview":"/p.jpg","x96":"/96.jpg","x48":"/48.jpg"},
        "url": "/animes/56-test",
        "kind": "tv",
        "score": "8.0",
        "status": "released",
        "episodes": 26,
        "episodes_aired": 26,
        "aired_on": "1998-04-03",
        "released_on": "1999-04-24",
        "unknown_future_field": "ignored"
    }"#
    .to_vec();
    let anime: Anime = simd_json::serde::from_slice(&mut fixture).unwrap();
    assert_eq!(anime.id, 56);
    assert_eq!(anime.name, "Test");
    assert_eq!(anime.episodes, Some(26));
}

#[test]
fn parses_v2_user_rate_fixture() {
    let mut fixture = br#"{
        "id": 13, "user_id": 23456789, "target_id": 12, "target_type": "Anime",
        "score": 0, "status": "completed", "rewatches": 0, "episodes": 12,
        "volumes": 0, "chapters": 0, "text": null, "text_html": "",
        "created_at": "2022-11-26T17:19:28.708+03:00",
        "updated_at": "2022-11-26T17:19:28.708+03:00"
    }"#
    .to_vec();
    let rate: UserRate = simd_json::serde::from_slice(&mut fixture).unwrap();
    assert_eq!(rate.target_id, Some(12));
    assert_eq!(rate.status, "completed");
}

#[test]
fn serializes_nested_comment_using_documented_wire_names() {
    let request = CreateCommentRequest {
        comment: NewComment {
            body: "A thoughtful comment".into(),
            commentable_id: 270119,
            commentable_type: CommentableType::Topic,
            is_offtopic: Some(false),
        },
        frontend: None,
        broadcast: None,
    };
    let value = simd_json::serde::to_string(&request).unwrap();
    assert!(value.contains("\"comment\""));
    assert!(value.contains("\"commentable_type\":\"Topic\""));
    assert!(!value.contains("frontend"));
}

#[test]
fn serializes_episode_notification_with_its_private_token_field() {
    let request = EpisodeNotificationRequest {
        episode_notification: NewEpisodeNotification {
            anime_id: 35,
            episode: 3,
            aired_at: "2022-11-19T17:19:31+03:00".into(),
            is_fandub: Some(true),
            is_raw: Some(false),
            is_subtitles: None,
            is_anime365: Some(true),
        },
        token: "not-a-real-token".into(),
    };
    let json = simd_json::serde::to_string(&request).unwrap();
    assert!(json.contains("\"episode_notification\""));
    assert!(json.contains("\"is_fandub\":true"));
    assert!(json.contains("\"token\":\"not-a-real-token\""));
}

#[test]
fn parses_episode_notification_response() {
    let mut fixture = br#"{
        "id":1,"anime_id":35,"episode":3,"is_raw":false,
        "is_subtitles":false,"is_fandub":true,"is_anime365":true,"topic_id":123
    }"#
    .to_vec();
    let response: EpisodeNotification = simd_json::serde::from_slice(&mut fixture).unwrap();
    assert!(response.is_fandub);
    assert_eq!(response.topic_id, Some(123));
}

#[test]
fn fixed_input_enum_has_documented_status_value() {
    assert_eq!(UserRateStatus::Watching.to_string(), "watching");
    assert_eq!(RateTargetType::Anime.to_string(), "Anime");
}
