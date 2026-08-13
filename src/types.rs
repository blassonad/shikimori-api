//! Публичные модели ответов Shikimori API.
//!
//! Поля дат остаются строками wire-format. Это сохраняет совместимость с
//! documented ISO-8601 строками, null и отдельными legacy `{}` значениями, не
//! добавляя неоговорённую зависимость для дат.

use serde::{Deserialize, Serialize};

/// Numeric user identifier.
pub type UserId = u64;

/// A compact image object used by anime, manga, people and characters.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Image {
    /// Original image URL/path.
    pub original: Option<String>,
    /// Preview image URL/path.
    pub preview: Option<String>,
    /// 96px image URL/path.
    pub x96: Option<String>,
    /// 48px image URL/path.
    pub x48: Option<String>,
}

/// User avatar image variants.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserImage {
    /// 160px image URL/path.
    pub x160: Option<String>,
    /// 148px image URL/path.
    pub x148: Option<String>,
    /// 80px image URL/path.
    pub x80: Option<String>,
    /// 64px image URL/path.
    pub x64: Option<String>,
    /// 48px image URL/path.
    pub x48: Option<String>,
    /// 32px image URL/path.
    pub x32: Option<String>,
    /// 16px image URL/path.
    pub x16: Option<String>,
}

/// Compact user representation embedded in API responses.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    /// User identifier.
    pub id: UserId,
    /// Public nickname.
    pub nickname: String,
    /// Legacy avatar URL/path.
    pub avatar: Option<String>,
    /// Avatar variants.
    pub image: Option<UserImage>,
    /// Last-seen timestamp in API wire-format.
    pub last_online_at: Option<String>,
    /// Profile URL.
    pub url: Option<String>,
}

/// Brief profile information.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserBrief {
    /// Shared compact user fields.
    #[serde(flatten)]
    pub user: User,
    /// Optional real name.
    pub name: Option<String>,
    /// Sex marker returned by API.
    pub sex: Option<String>,
    /// Website URL.
    pub website: Option<String>,
    /// Birth date in API wire-format.
    pub birth_on: Option<String>,
    /// Age in full years.
    pub full_years: Option<u64>,
    /// Locale code.
    pub locale: Option<String>,
}

/// A compact anime result.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Anime {
    /// Anime identifier.
    pub id: u64,
    /// Primary title.
    pub name: String,
    /// Russian title.
    pub russian: Option<String>,
    /// Poster images.
    pub image: Image,
    /// Relative Shikimori URL.
    pub url: String,
    /// API string kind; kept open for forward compatibility.
    pub kind: Option<String>,
    /// Decimal score represented by API as a string.
    pub score: Option<String>,
    /// API string status; kept open for forward compatibility.
    pub status: Option<String>,
    /// Total episode count.
    pub episodes: Option<u64>,
    /// Aired episode count.
    pub episodes_aired: Option<u64>,
    /// Airing date in API wire-format.
    pub aired_on: Option<String>,
    /// Release date in API wire-format.
    pub released_on: Option<String>,
}

/// Full anime record.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AnimeDetails {
    /// Shared anime fields.
    #[serde(flatten)]
    pub anime: Anime,
    /// Age rating.
    pub rating: Option<String>,
    /// English aliases. API may include null elements.
    pub english: Vec<Option<String>>,
    /// Japanese aliases. API may include null elements.
    pub japanese: Vec<Option<String>>,
    /// Alternative titles.
    pub synonyms: Vec<String>,
    /// Russian licence name.
    pub license_name_ru: Option<String>,
    /// Episode duration in minutes.
    pub duration: Option<u64>,
    /// Source markdown.
    pub description: Option<String>,
    /// Rendered description HTML.
    pub description_html: Option<String>,
    /// Description source attribution.
    pub description_source: Option<String>,
    /// Franchise name.
    pub franchise: Option<String>,
    /// Whether current user favourited the anime.
    pub favoured: Option<bool>,
    /// Whether announced.
    pub anons: Option<bool>,
    /// Whether ongoing.
    pub ongoing: Option<bool>,
    /// Thread identifier.
    pub thread_id: Option<u64>,
    /// Topic identifier.
    pub topic_id: Option<u64>,
    /// MyAnimeList identifier.
    pub myanimelist_id: Option<u64>,
    /// Last update timestamp.
    pub updated_at: Option<String>,
    /// Next episode timestamp.
    pub next_episode_at: Option<String>,
    /// Fansubber names.
    pub fansubbers: Vec<String>,
    /// Fandubber names.
    pub fandubbers: Vec<String>,
    /// Licensor names.
    pub licensors: Vec<String>,
    /// Genres.
    pub genres: Vec<Genre>,
    /// Studios.
    pub studios: Vec<Studio>,
    /// Videos embedded by API.
    pub videos: Vec<Video>,
    /// Screenshots embedded by API.
    pub screenshots: Vec<Screenshot>,
    /// Current user's rate where available.
    pub user_rate: Option<UserRate>,
}

/// A compact manga/ranobe record.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Manga {
    /// Manga identifier.
    pub id: u64,
    /// Primary title.
    pub name: String,
    /// Russian title.
    pub russian: Option<String>,
    /// Poster images.
    pub image: Image,
    /// Relative Shikimori URL.
    pub url: String,
    /// API string kind.
    pub kind: Option<String>,
    /// Decimal score string.
    pub score: Option<String>,
    /// API string status.
    pub status: Option<String>,
    /// Volume count.
    pub volumes: Option<u64>,
    /// Chapter count.
    pub chapters: Option<u64>,
    /// Airing date.
    pub aired_on: Option<String>,
    /// Release date.
    pub released_on: Option<String>,
}

/// Full manga or ranobe record.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MangaDetails {
    /// Shared manga fields.
    #[serde(flatten)]
    pub manga: Manga,
    /// English aliases.
    pub english: Vec<Option<String>>,
    /// Japanese aliases.
    pub japanese: Vec<Option<String>>,
    /// Alternative titles.
    pub synonyms: Vec<String>,
    /// Russian licence name.
    pub license_name_ru: Option<String>,
    /// Source markdown.
    pub description: Option<String>,
    /// Rendered description HTML.
    pub description_html: Option<String>,
    /// Description source attribution.
    pub description_source: Option<String>,
    /// Franchise name.
    pub franchise: Option<String>,
    /// Whether current user favourited it.
    pub favoured: Option<bool>,
    /// Whether announced.
    pub anons: Option<bool>,
    /// Whether ongoing.
    pub ongoing: Option<bool>,
    /// Thread identifier.
    pub thread_id: Option<u64>,
    /// Topic identifier.
    pub topic_id: Option<u64>,
    /// MyAnimeList identifier.
    pub myanimelist_id: Option<u64>,
    /// Scores statistics; server shape is not fixed in REST docs.
    #[serde(default)]
    pub rates_scores_stats: Vec<RateStatistic>,
    /// Status statistics; server shape is not fixed in REST docs.
    #[serde(default)]
    pub rates_statuses_stats: Vec<RateStatistic>,
    /// Licensor names.
    pub licensors: Vec<String>,
    /// Genres.
    pub genres: Vec<Genre>,
    /// Publishers.
    pub publishers: Vec<Publisher>,
    /// Current user's rate.
    pub user_rate: Option<UserRate>,
}

/// API rate statistic used in full entry records.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RateStatistic {
    /// Statistic label/name.
    pub name: Option<String>,
    /// Count/value.
    pub value: Option<u64>,
}

/// Genre metadata.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Genre {
    /// Genre identifier.
    pub id: u64,
    /// Default name.
    pub name: String,
    /// Russian name when available.
    pub russian: Option<String>,
    /// Target kind.
    pub kind: Option<String>,
}

/// Studio metadata.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Studio {
    /// Studio identifier.
    pub id: u64,
    /// Public name.
    pub name: String,
    /// Normalized name.
    pub filtered_name: Option<String>,
    /// Whether it is a real studio.
    pub real: Option<bool>,
    /// Image URL/path.
    pub image: Option<String>,
}

/// Publisher metadata.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Publisher {
    /// Publisher identifier.
    pub id: u64,
    /// Publisher name.
    pub name: String,
}

/// Character summary.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Character {
    /// Character identifier.
    pub id: u64,
    /// Primary name.
    pub name: String,
    /// Russian name.
    pub russian: Option<String>,
    /// Poster images.
    pub image: Image,
    /// Relative URL.
    pub url: String,
}

/// Detailed character record.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CharacterDetails {
    /// Compact character fields.
    #[serde(flatten)]
    pub character: Character,
    /// Alternative name.
    pub altname: Option<String>,
    /// Japanese name.
    pub japanese: Option<String>,
    /// Source description.
    pub description: Option<String>,
    /// HTML description.
    pub description_html: Option<String>,
    /// Description source.
    pub description_source: Option<String>,
    /// Favourite marker.
    pub favoured: Option<bool>,
    /// Thread id.
    pub thread_id: Option<u64>,
    /// Topic id.
    pub topic_id: Option<u64>,
    /// Last update timestamp.
    pub updated_at: Option<String>,
    /// Voice actors.
    pub seyu: Vec<Person>,
    /// Related anime with roles.
    pub animes: Vec<WorkRole>,
    /// Related manga with roles.
    pub mangas: Vec<WorkRole>,
}

/// Person summary.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Person {
    /// Person identifier.
    pub id: u64,
    /// Primary name.
    pub name: String,
    /// Russian name.
    pub russian: Option<String>,
    /// Poster images.
    pub image: Image,
    /// Relative URL.
    pub url: String,
}

/// Full person information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PersonDetails {
    /// Compact person fields.
    #[serde(flatten)]
    pub person: Person,
    /// Japanese name.
    pub japanese: Option<String>,
    /// Profession/title.
    pub job_title: Option<String>,
    /// Birth date, deliberately flexible in legacy responses.
    pub birth_on: Option<FlexibleDate>,
    /// Death date, deliberately flexible in legacy responses.
    pub deceased_on: Option<FlexibleDate>,
    /// Website.
    pub website: Option<String>,
    /// Grouped roles. REST shape may evolve; omitted fields are accepted.
    #[serde(default)]
    pub groupped_roles: Vec<GroupedRole>,
    /// Roles. REST shape may evolve; omitted fields are accepted.
    #[serde(default)]
    pub roles: Vec<Role>,
    /// Works. REST shape may evolve; omitted fields are accepted.
    #[serde(default)]
    pub works: Vec<WorkRole>,
    /// Topic id.
    pub topic_id: Option<u64>,
    /// Favourite markers.
    pub person_favoured: Option<bool>,
    /// Professional markers.
    pub producer: Option<bool>,
    /// Favourite producer marker.
    pub producer_favoured: Option<bool>,
    /// Mangaka marker.
    pub mangaka: Option<bool>,
    /// Favourite mangaka marker.
    pub mangaka_favoured: Option<bool>,
    /// Seiyu marker.
    pub seyu: Option<bool>,
    /// Favourite seiyu marker.
    pub seyu_favoured: Option<bool>,
    /// Last update timestamp.
    pub updated_at: Option<String>,
    /// Thread id.
    pub thread_id: Option<u64>,
    /// Birthday returned by legacy endpoint.
    pub birthday: Option<FlexibleDate>,
}

/// String/null/object-compatible legacy date field.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FlexibleDate {
    /// A textual date.
    Text(String),
    /// An empty/object legacy value.
    #[default]
    Unknown,
}

/// A role relationship between an entry and a person/character.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Role {
    /// Original roles.
    #[serde(default)]
    pub roles: Vec<String>,
    /// Russian roles.
    #[serde(default)]
    pub roles_russian: Vec<String>,
    /// Related character.
    pub character: Option<Character>,
    /// Related person.
    pub person: Option<Person>,
}

/// Grouped role information returned for a person.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GroupedRole {
    /// Group name.
    pub group: Option<String>,
    /// Roles within group.
    #[serde(default)]
    pub roles: Vec<Role>,
}

/// An entry and its role, returned from character/person references.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkRole {
    /// Entry id.
    pub id: u64,
    /// Entry name.
    pub name: String,
    /// Russian title.
    pub russian: Option<String>,
    /// Poster image.
    pub image: Image,
    /// Relative URL.
    pub url: String,
    /// Entry kind.
    pub kind: Option<String>,
    /// Score.
    pub score: Option<String>,
    /// Status.
    pub status: Option<String>,
    /// Episodes when anime.
    pub episodes: Option<u64>,
    /// Volumes when manga.
    pub volumes: Option<u64>,
    /// Chapters when manga.
    pub chapters: Option<u64>,
    /// Original roles.
    #[serde(default)]
    pub roles: Vec<String>,
    /// Single role string in some responses.
    pub role: Option<String>,
}

/// A screenshot pair.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Screenshot {
    /// Original URL/path.
    pub original: String,
    /// Preview URL/path.
    pub preview: String,
}

/// Video record.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Video {
    /// Video identifier.
    pub id: u64,
    /// Original URL.
    pub url: String,
    /// Poster URL.
    pub image_url: Option<String>,
    /// Embed/player URL.
    pub player_url: Option<String>,
    /// Display name.
    pub name: Option<String>,
    /// API string kind.
    pub kind: Option<String>,
    /// Hosting provider.
    pub hosting: Option<String>,
}

/// An external database link.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalLink {
    /// Link identifier when persisted by Shikimori.
    pub id: Option<u64>,
    /// Link kind.
    pub kind: String,
    /// Link URL.
    pub url: String,
    /// Source system.
    pub source: String,
    /// Linked entry id.
    pub entry_id: u64,
    /// Entry type.
    pub entry_type: String,
    /// Creation timestamp.
    pub created_at: Option<String>,
    /// Update timestamp.
    pub updated_at: Option<String>,
    /// Import timestamp.
    pub imported_at: Option<String>,
}

/// A direct related-entry relationship.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Related {
    /// Relationship type.
    pub relation: Option<String>,
    /// Russian relationship type.
    pub relation_russian: Option<String>,
    /// Related anime, where applicable.
    pub anime: Option<Anime>,
    /// Related manga/ranobe, where applicable.
    pub manga: Option<Manga>,
}

/// Franchise graph.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Franchise {
    /// Relationship links.
    #[serde(default)]
    pub links: Vec<FranchiseLink>,
    /// Graph nodes.
    #[serde(default)]
    pub nodes: Vec<FranchiseNode>,
    /// Current entry id.
    pub current_id: u64,
}

/// One franchise graph edge.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FranchiseLink {
    /// Edge id.
    pub id: u64,
    /// Source entity id.
    pub source_id: u64,
    /// Target entity id.
    pub target_id: u64,
    /// Source graph index.
    pub source: u64,
    /// Target graph index.
    pub target: u64,
    /// Edge weight.
    pub weight: i64,
    /// Relation kind.
    pub relation: String,
}

/// One franchise graph node.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FranchiseNode {
    /// Node entry id.
    pub id: u64,
    /// Unix timestamp/date field returned by server.
    pub date: Option<i64>,
    /// Display name.
    pub name: String,
    /// Image URL/path.
    pub image_url: Option<String>,
    /// Relative URL.
    pub url: String,
    /// Release year.
    pub year: Option<i64>,
    /// Human-readable kind.
    pub kind: String,
    /// Graph weight.
    pub weight: i64,
}

/// Calendar item for a scheduled episode.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalendarEntry {
    /// Next episode number.
    pub next_episode: Option<u64>,
    /// Next episode timestamp.
    pub next_episode_at: Option<String>,
    /// Episode duration.
    pub duration: Option<u64>,
    /// Related anime.
    pub anime: Anime,
}

/// Forum metadata.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Forum {
    /// Forum id.
    pub id: u64,
    /// Display position.
    pub position: i64,
    /// Name.
    pub name: String,
    /// URL slug.
    pub permalink: String,
    /// Relative URL.
    pub url: String,
}

/// Club logo image variants.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClubLogo {
    /// Original URL/path.
    pub original: Option<String>,
    /// Main URL/path.
    pub main: Option<String>,
    /// 96px URL/path.
    pub x96: Option<String>,
    /// 73px URL/path.
    pub x73: Option<String>,
    /// 48px URL/path.
    pub x48: Option<String>,
}

/// Compact club.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Club {
    /// Club id.
    pub id: u64,
    /// Club name.
    pub name: String,
    /// Logo variants.
    pub logo: ClubLogo,
    /// Censorship marker.
    pub is_censored: Option<bool>,
    /// Join policy.
    pub join_policy: Option<String>,
    /// Comment policy.
    pub comment_policy: Option<String>,
}

/// Full club.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ClubDetails {
    /// Compact club fields.
    #[serde(flatten)]
    pub club: Club,
    /// Source description.
    pub description: Option<String>,
    /// Rendered description HTML.
    pub description_html: Option<String>,
    /// Linked manga.
    pub mangas: Vec<Manga>,
    /// Linked characters.
    pub characters: Vec<Character>,
    /// Thread id.
    pub thread_id: Option<u64>,
    /// Topic id.
    pub topic_id: Option<u64>,
    /// Current user's role.
    pub user_role: Option<String>,
    /// Style id.
    pub style_id: Option<u64>,
    /// Members.
    pub members: Vec<User>,
    /// Linked anime.
    pub animes: Vec<Anime>,
    /// Club images.
    pub images: Vec<ClubImage>,
}

/// An image attached to a club.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClubImage {
    /// Image id.
    pub id: u64,
    /// Original URL.
    pub original_url: String,
    /// Main URL.
    pub main_url: String,
    /// Preview URL.
    pub preview_url: String,
    /// Whether current user may delete it.
    pub can_destroy: Option<bool>,
    /// Creator user id.
    pub user_id: Option<u64>,
}

/// A comment.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    /// Comment id.
    pub id: u64,
    /// Author id.
    pub user_id: u64,
    /// Target id.
    pub commentable_id: u64,
    /// Target type.
    pub commentable_type: String,
    /// Source BBCode/text.
    pub body: String,
    /// HTML body.
    pub html_body: Option<String>,
    /// Creation timestamp.
    pub created_at: String,
    /// Update timestamp.
    pub updated_at: String,
    /// Offtopic marker.
    pub is_offtopic: bool,
    /// Summary marker.
    pub is_summary: Option<bool>,
    /// Edit permission for current token.
    pub can_be_edited: Option<bool>,
    /// Author.
    pub user: Option<User>,
}

/// Topic thread.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Topic {
    /// Topic id.
    pub id: u64,
    /// Title.
    pub topic_title: Option<String>,
    /// Source text.
    pub body: Option<String>,
    /// HTML body.
    pub html_body: Option<String>,
    /// HTML footer.
    pub html_footer: Option<String>,
    /// Creation timestamp.
    pub created_at: Option<String>,
    /// Number of comments.
    pub comments_count: Option<u64>,
    /// Forum.
    pub forum: Option<Forum>,
    /// Topic author.
    pub user: Option<User>,
    /// Server topic subtype, open string.
    #[serde(rename = "type")]
    pub topic_type: Option<String>,
    /// Linked entity id.
    pub linked_id: Option<u64>,
    /// Linked entity type.
    pub linked_type: Option<String>,
    /// Whether viewed by current user.
    pub viewed: Option<bool>,
    /// Last viewed comment id.
    pub last_comment_viewed: Option<u64>,
    /// Event marker.
    pub event: Option<String>,
    /// Episode marker.
    pub episode: Option<u64>,
}

/// Short update-topic response.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TopicUpdate {
    /// Topic id.
    pub id: u64,
    /// Event target. Kept as optional anime-compatible summary where documented.
    pub linked: Option<Anime>,
    /// Update event.
    pub event: Option<String>,
    /// Episode number.
    pub episode: Option<u64>,
    /// Timestamp.
    pub created_at: Option<String>,
    /// Absolute URL.
    pub url: Option<String>,
}

/// Review resource.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Review {
    /// Review id.
    pub id: u64,
    /// Author id.
    pub user_id: u64,
    /// Anime id.
    pub anime_id: Option<u64>,
    /// Manga id.
    pub manga_id: Option<u64>,
    /// Body.
    pub body: String,
    /// Opinion value.
    pub opinion: String,
    /// Whether written before release.
    pub is_written_before_release: Option<bool>,
    /// Creation timestamp.
    pub created_at: String,
    /// Update timestamp.
    pub updated_at: String,
    /// Comment count.
    pub comments_count: Option<u64>,
    /// Upvote count.
    pub cached_votes_up: Option<i64>,
    /// Downvote count.
    pub cached_votes_down: Option<i64>,
    /// Change timestamp.
    pub changed_at: Option<String>,
}

/// Private or announcement message without mandatory participant data.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Message {
    /// Message id.
    pub id: u64,
    /// Message kind.
    pub kind: String,
    /// Read marker.
    pub read: bool,
    /// Source body.
    pub body: String,
    /// HTML body.
    pub html_body: Option<String>,
    /// Creation timestamp.
    pub created_at: String,
    /// Linked entity id.
    pub linked_id: Option<u64>,
    /// Linked entity type.
    pub linked_type: Option<String>,
}

/// Message enriched with sender and recipient.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageWithParticipants {
    /// Shared message fields.
    #[serde(flatten)]
    pub message: Message,
    /// Sender.
    pub from: Option<User>,
    /// Recipient.
    pub to: Option<User>,
}

/// Dialog summary.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Dialog {
    /// Other participant.
    pub target_user: User,
    /// Latest message.
    pub message: Message,
}

/// Ban record.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Ban {
    /// Ban id.
    pub id: u64,
    /// Banned user id.
    pub user_id: u64,
    /// Ban comment.
    pub comment: Option<Comment>,
    /// Moderator id.
    pub moderator_id: Option<u64>,
    /// Ban reason.
    pub reason: Option<String>,
    /// Creation timestamp.
    pub created_at: String,
    /// Duration in minutes.
    pub duration_minutes: Option<u64>,
    /// Banned user.
    pub user: Option<User>,
    /// Moderator.
    pub moderator: Option<User>,
}

/// User rate.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UserRate {
    /// User-rate id.
    pub id: u64,
    /// Owner id (reliably present in v2).
    pub user_id: Option<u64>,
    /// Target id (reliably present in v2).
    pub target_id: Option<u64>,
    /// Target type.
    pub target_type: Option<String>,
    /// Score 0–10.
    pub score: u8,
    /// List status.
    pub status: String,
    /// Rewatch count.
    pub rewatches: Option<u64>,
    /// Episode progress.
    pub episodes: Option<u64>,
    /// Volume progress.
    pub volumes: Option<u64>,
    /// Chapter progress.
    pub chapters: Option<u64>,
    /// User note.
    pub text: Option<String>,
    /// Rendered note.
    pub text_html: Option<String>,
    /// Creation timestamp.
    pub created_at: String,
    /// Update timestamp.
    pub updated_at: String,
}

/// User rate including embedded owner and target.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UserRateWithTarget {
    /// Shared rate fields.
    #[serde(flatten)]
    pub user_rate: UserRate,
    /// Owner.
    pub user: Option<User>,
    /// Anime target.
    pub anime: Option<Anime>,
    /// Manga target.
    pub manga: Option<Manga>,
}

/// User favourites grouped by kind.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Favourites {
    /// Favorite anime.
    pub animes: Vec<Anime>,
    /// Favorite manga.
    pub mangas: Vec<Manga>,
    /// Favorite ranobe.
    pub ranobe: Vec<Manga>,
    /// Favorite characters.
    pub characters: Vec<Character>,
    /// Favorite people.
    pub people: Vec<Person>,
    /// Favorite mangaka.
    pub mangakas: Vec<Person>,
    /// Favorite seiyu.
    pub seyu: Vec<Person>,
    /// Favorite producers.
    pub producers: Vec<Person>,
}

/// User history item.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// History item id.
    pub id: u64,
    /// Timestamp.
    pub created_at: String,
    /// Human-readable action.
    pub description: String,
    /// Target may be absent.
    pub target: Option<Anime>,
}

/// Unread message counters.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnreadMessages {
    /// Private message count.
    pub messages: u64,
    /// News count.
    pub news: u64,
    /// Notification count.
    pub notifications: u64,
}

/// User statistics group, intentionally sparse because REST server schema is nested/evolving.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UserStats {
    /// Whether user has anime.
    #[serde(rename = "has_anime?")]
    pub has_anime: Option<bool>,
    /// Whether user has manga.
    #[serde(rename = "has_manga?")]
    pub has_manga: Option<bool>,
}

/// Full user profile.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UserDetails {
    /// Compact user fields.
    #[serde(flatten)]
    pub user: User,
    /// Real name.
    pub name: Option<String>,
    /// Sex marker.
    pub sex: Option<String>,
    /// Age.
    pub full_years: Option<u64>,
    /// Localized last-online text.
    pub last_online: Option<String>,
    /// Website.
    pub website: Option<String>,
    /// Location.
    pub location: Option<String>,
    /// Banned marker.
    pub banned: Option<bool>,
    /// Profile source text.
    pub about: Option<String>,
    /// Profile HTML.
    pub about_html: Option<String>,
    /// Display information.
    pub common_info: Vec<String>,
    /// Whether comments shown.
    pub show_comments: Option<bool>,
    /// Friendship state.
    pub in_friends: Option<bool>,
    /// Ignore state.
    pub is_ignored: Option<bool>,
    /// Minimal stable user statistics.
    pub stats: Option<UserStats>,
    /// Style id.
    pub style_id: Option<u64>,
}

/// An achievement returned for a user.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Achievement {
    /// Achievement id.
    pub id: u64,
    /// Achievement position/progress level.
    pub level: Option<u64>,
    /// Progress value.
    pub progress: Option<f64>,
    /// User id.
    pub user_id: Option<u64>,
    /// Anime id.
    pub anime_id: Option<u64>,
    /// Achievement name.
    pub name: Option<String>,
}

/// A smiley mapping.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Smiley {
    /// BBCode token.
    pub bbcode: String,
    /// Image path.
    pub path: String,
}

/// Available anime constants.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnimeConstants {
    /// Allowed kinds.
    pub kind: Vec<String>,
    /// Allowed statuses.
    pub status: Vec<String>,
}

/// Available manga constants.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MangaConstants {
    /// Allowed kinds.
    pub kind: Vec<String>,
    /// Allowed statuses.
    pub status: Vec<String>,
}

/// Available user-rate constants.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRateConstants {
    /// Allowed statuses.
    pub status: Vec<String>,
}

/// Available club constants.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClubConstants {
    /// Join policies.
    pub join_policy: Vec<String>,
    /// Comment policies.
    pub comment_policy: Vec<String>,
    /// Image upload policies.
    pub image_upload_policy: Vec<String>,
}

/// Style resource.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Style {
    /// Style id; absent for preview.
    pub id: Option<u64>,
    /// Owner id; absent for preview.
    pub owner_id: Option<u64>,
    /// Owner type.
    pub owner_type: Option<String>,
    /// Style name.
    pub name: String,
    /// Source CSS.
    pub css: String,
    /// Compiled CSS where available.
    pub compiled_css: Option<String>,
    /// Creation timestamp.
    pub created_at: Option<String>,
    /// Update timestamp.
    pub updated_at: Option<String>,
}

/// Generic notice response.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Notice {
    /// Localized message.
    pub notice: String,
}

/// Favourite operation response.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuccessNotice {
    /// Operation result.
    pub success: bool,
    /// Localized notice.
    pub notice: String,
}

/// Legacy topic-ignore toggle instruction.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToggleUrl {
    /// Toggle object id when creating.
    pub id: Option<u64>,
    /// Follow-up route.
    pub url: String,
    /// Follow-up HTTP method.
    pub method: String,
}

/// Uploaded user image response.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UploadedUserImage {
    /// Image id.
    pub id: u64,
    /// Preview URL.
    pub preview: String,
    /// Original URL.
    pub url: String,
    /// BBCode insertion text.
    pub bbcode: String,
}

/// v2 topic/user ignore state.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct IgnoreState {
    /// Topic identifier, when endpoint operates on a topic.
    pub topic_id: Option<String>,
    /// User identifier, when endpoint operates on a user.
    pub user_id: Option<String>,
    /// Current ignore state.
    pub is_ignored: bool,
}

/// v2 moderator-request response for offtopic reports.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AbuseRequest {
    /// Request kind.
    pub kind: String,
    /// New/affected value.
    pub value: Option<bool>,
    /// Affected ids.
    #[serde(default)]
    pub affected_ids: Vec<u64>,
}

/// Episode release notification response.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpisodeNotification {
    /// Notification id.
    pub id: u64,
    /// Anime id.
    pub anime_id: u64,
    /// Episode number.
    pub episode: u64,
    /// Raw-availability marker.
    pub is_raw: bool,
    /// Subtitle-availability marker.
    pub is_subtitles: bool,
    /// Fandub-availability marker.
    pub is_fandub: bool,
    /// Anime365 marker.
    pub is_anime365: bool,
    /// Created topic id.
    pub topic_id: Option<u64>,
}
