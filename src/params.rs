//! Типы входных данных REST-операций: query, path-значения и тела запросов.

use serde::Serialize;

/// Internal/public trait used to serialize explicit query DTOs without a URL crate.
pub trait QueryParameters {
    /// Appends only present parameters as `(key, value)` pairs.
    fn append_to(&self, output: &mut Vec<(String, String)>);

    /// Converts a query DTO to key-value pairs.
    fn pairs(&self) -> Vec<(String, String)> {
        let mut output = Vec::new();
        self.append_to(&mut output);
        output
    }
}

impl QueryParameters for () {
    fn append_to(&self, _: &mut Vec<(String, String)>) {}
}

fn optional<T: ToString>(output: &mut Vec<(String, String)>, key: &str, value: &Option<T>) {
    if let Some(value) = value {
        output.push((key.to_owned(), value.to_string()));
    }
}

fn optional_bool(output: &mut Vec<(String, String)>, key: &str, value: Option<bool>) {
    if let Some(value) = value {
        output.push((key.to_owned(), value.to_string()));
    }
}

fn optional_csv<T: ToString>(
    output: &mut Vec<(String, String)>,
    key: &str,
    value: &Option<Vec<T>>,
) {
    if let Some(value) = value {
        output.push((
            key.to_owned(),
            value
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(","),
        ));
    }
}

/// A conventional page and limit query.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PageLimitQuery {
    /// One-based page number.
    pub page: Option<u64>,
    /// Maximum result count; endpoint-specific documented maximums apply.
    pub limit: Option<u64>,
}

impl QueryParameters for PageLimitQuery {
    fn append_to(&self, output: &mut Vec<(String, String)>) {
        optional(output, "page", &self.page);
        optional(output, "limit", &self.limit);
    }
}

/// Required query for achievements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AchievementsQuery {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub user_id: u64,
}
impl QueryParameters for AchievementsQuery {
    fn append_to(&self, o: &mut Vec<(String, String)>) {
        o.push(("user_id".into(), self.user_id.to_string()));
    }
}

/// Censorship switch used by the calendar route.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CensoredQuery {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub censored: Option<bool>,
}
impl QueryParameters for CensoredQuery {
    fn append_to(&self, o: &mut Vec<(String, String)>) {
        optional_bool(o, "censored", self.censored);
    }
}

/// One optional numeric limit.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LimitQuery {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub limit: Option<u64>,
}
impl QueryParameters for LimitQuery {
    fn append_to(&self, o: &mut Vec<(String, String)>) {
        optional(o, "limit", &self.limit);
    }
}

/// Lookup setting for `/api/users/:id`; nickname lookup is enabled only when true.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct IsNicknameQuery {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub is_nickname: bool,
}
impl QueryParameters for IsNicknameQuery {
    fn append_to(&self, o: &mut Vec<(String, String)>) {
        if self.is_nickname {
            o.push(("is_nickname".into(), "1".into()));
        }
    }
}

/// Generic text search with the optional person-kind filter.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SearchQuery {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub search: Option<String>,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub kind: Option<PersonKind>,
}
impl QueryParameters for SearchQuery {
    fn append_to(&self, o: &mut Vec<(String, String)>) {
        optional(o, "search", &self.search);
        optional(o, "kind", &self.kind);
    }
}

/// Favourite-reorder body.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub struct ReorderFavoriteRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub new_index: Option<u64>,
}

/// Resource type used by legacy cleanup/reset user-rate routes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserRateListType {
    /// Вариант, поддерживаемый публичным API.
    Anime,
    /// Вариант, поддерживаемый публичным API.
    Manga,
}
impl ::std::fmt::Display for UserRateListType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(match self {
            Self::Anime => "anime",
            Self::Manga => "manga",
        })
    }
}

/// List anime query for `GET /api/animes` and legacy `/api/animes/search`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ListAnimesQuery {
    /// One-based page (1..=100000).
    pub page: Option<u64>,
    /// Limit (maximum 50).
    pub limit: Option<u64>,
    /// Sort order.
    pub order: Option<AnimeOrder>,
    /// Deprecated legacy type field.
    pub legacy_type: Option<String>,
    /// One or more include/exclude anime kinds.
    pub kind: Option<Vec<ListValue<AnimeKind>>>,
    /// One or more include/exclude statuses.
    pub status: Option<Vec<ListValue<AnimeStatus>>>,
    /// Seasons/years as documented CSV values.
    pub season: Option<Vec<String>>,
    /// Minimum score.
    pub score: Option<f64>,
    /// Duration class.
    pub duration: Option<AnimeDuration>,
    /// Age rating.
    pub rating: Option<AgeRating>,
    /// Genre ids.
    pub genre: Option<Vec<u64>>,
    /// V2 genre ids.
    pub genre_v2: Option<Vec<u64>>,
    /// Studio ids.
    pub studio: Option<Vec<u64>>,
    /// Franchise names.
    pub franchise: Option<Vec<String>>,
    /// Censorship filter.
    pub censored: Option<bool>,
    /// Current user's list status.
    pub mylist: Option<UserRateStatus>,
    /// Exact anime ids.
    pub ids: Option<Vec<u64>>,
    /// Ids to omit.
    pub exclude_ids: Option<Vec<u64>>,
    /// Search phrase.
    pub search: Option<String>,
}

impl QueryParameters for ListAnimesQuery {
    fn append_to(&self, output: &mut Vec<(String, String)>) {
        optional(output, "page", &self.page);
        optional(output, "limit", &self.limit);
        optional(output, "order", &self.order);
        optional(output, "type", &self.legacy_type);
        optional_csv(output, "kind", &self.kind);
        optional_csv(output, "status", &self.status);
        optional_csv(output, "season", &self.season);
        optional(output, "score", &self.score);
        optional(output, "duration", &self.duration);
        optional(output, "rating", &self.rating);
        optional_csv(output, "genre", &self.genre);
        optional_csv(output, "genre_v2", &self.genre_v2);
        optional_csv(output, "studio", &self.studio);
        optional_csv(output, "franchise", &self.franchise);
        optional_bool(output, "censored", self.censored);
        optional(output, "mylist", &self.mylist);
        optional_csv(output, "ids", &self.ids);
        optional_csv(output, "exclude_ids", &self.exclude_ids);
        optional(output, "search", &self.search);
    }
}

/// List manga query for `GET /api/mangas` and legacy search.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ListMangasQuery {
    /// One-based page.
    pub page: Option<u64>,
    /// Limit (maximum 50).
    pub limit: Option<u64>,
    /// Sort order.
    pub order: Option<MangaOrder>,
    /// Deprecated legacy type field.
    pub legacy_type: Option<String>,
    /// Include/exclude manga kinds.
    pub kind: Option<Vec<ListValue<MangaKind>>>,
    /// Include/exclude statuses.
    pub status: Option<Vec<ListValue<MangaStatus>>>,
    /// Seasons/years.
    pub season: Option<Vec<String>>,
    /// Minimum score.
    pub score: Option<f64>,
    /// Genre ids.
    pub genre: Option<Vec<u64>>,
    /// V2 genre ids.
    pub genre_v2: Option<Vec<u64>>,
    /// Publisher ids.
    pub publisher: Option<Vec<u64>>,
    /// Franchise names.
    pub franchise: Option<Vec<String>>,
    /// Censorship filter.
    pub censored: Option<bool>,
    /// Current user's list status.
    pub mylist: Option<UserRateStatus>,
    /// Exact manga ids.
    pub ids: Option<Vec<u64>>,
    /// Ids to omit.
    pub exclude_ids: Option<Vec<u64>>,
    /// Search phrase.
    pub search: Option<String>,
}

impl QueryParameters for ListMangasQuery {
    fn append_to(&self, output: &mut Vec<(String, String)>) {
        optional(output, "page", &self.page);
        optional(output, "limit", &self.limit);
        optional(output, "order", &self.order);
        optional(output, "type", &self.legacy_type);
        optional_csv(output, "kind", &self.kind);
        optional_csv(output, "status", &self.status);
        optional_csv(output, "season", &self.season);
        optional(output, "score", &self.score);
        optional_csv(output, "genre", &self.genre);
        optional_csv(output, "genre_v2", &self.genre_v2);
        optional_csv(output, "publisher", &self.publisher);
        optional_csv(output, "franchise", &self.franchise);
        optional_bool(output, "censored", self.censored);
        optional(output, "mylist", &self.mylist);
        optional_csv(output, "ids", &self.ids);
        optional_csv(output, "exclude_ids", &self.exclude_ids);
        optional(output, "search", &self.search);
    }
}

/// List ranobe query for `GET /api/ranobe`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ListRanobeQuery {
    /// One-based page.
    pub page: Option<u64>,
    /// Limit (maximum 50).
    pub limit: Option<u64>,
    /// Sort order.
    pub order: Option<MangaOrder>,
    /// Include/exclude statuses.
    pub status: Option<Vec<ListValue<MangaStatus>>>,
    /// Seasons/years.
    pub season: Option<Vec<String>>,
    /// Minimum score.
    pub score: Option<f64>,
    /// Genre ids.
    pub genre: Option<Vec<u64>>,
    /// Publisher ids.
    pub publisher: Option<Vec<u64>>,
    /// Franchise names.
    pub franchise: Option<Vec<String>>,
    /// Censorship filter.
    pub censored: Option<bool>,
    /// Current user's list status.
    pub mylist: Option<UserRateStatus>,
    /// Exact ids.
    pub ids: Option<Vec<u64>>,
    /// Ids to omit.
    pub exclude_ids: Option<Vec<u64>>,
    /// Search phrase.
    pub search: Option<String>,
}

impl QueryParameters for ListRanobeQuery {
    fn append_to(&self, output: &mut Vec<(String, String)>) {
        optional(output, "page", &self.page);
        optional(output, "limit", &self.limit);
        optional(output, "order", &self.order);
        optional_csv(output, "status", &self.status);
        optional_csv(output, "season", &self.season);
        optional(output, "score", &self.score);
        optional_csv(output, "genre", &self.genre);
        optional_csv(output, "publisher", &self.publisher);
        optional_csv(output, "franchise", &self.franchise);
        optional_bool(output, "censored", self.censored);
        optional(output, "mylist", &self.mylist);
        optional_csv(output, "ids", &self.ids);
        optional_csv(output, "exclude_ids", &self.exclude_ids);
        optional(output, "search", &self.search);
    }
}

/// List clubs query.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ListClubsQuery {
    /// One-based page.
    pub page: Option<u64>,
    /// Limit (maximum 30).
    pub limit: Option<u64>,
    /// Search phrase.
    pub search: Option<String>,
}
impl QueryParameters for ListClubsQuery {
    fn append_to(&self, o: &mut Vec<(String, String)>) {
        optional(o, "page", &self.page);
        optional(o, "limit", &self.limit);
        optional(o, "search", &self.search);
    }
}

/// Query for a related resource with an endpoint-specific page/limit maximum.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PageLimit30Query {
    /// One-based page.
    pub page: Option<u64>,
    /// Limit (maximum 30 for the routes using this type).
    pub limit: Option<u64>,
}
impl QueryParameters for PageLimit30Query {
    fn append_to(&self, o: &mut Vec<(String, String)>) {
        optional(o, "page", &self.page);
        optional(o, "limit", &self.limit);
    }
}

/// List comments query.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListCommentsQuery {
    /// Required target id.
    pub commentable_id: u64,
    /// Required target type (Topic or User).
    pub commentable_type: CommentListTarget,
    /// One-based page.
    pub page: Option<u64>,
    /// Limit (maximum 30).
    pub limit: Option<u64>,
    /// Descending order marker.
    pub desc: Option<bool>,
}
impl QueryParameters for ListCommentsQuery {
    fn append_to(&self, o: &mut Vec<(String, String)>) {
        o.push(("commentable_id".into(), self.commentable_id.to_string()));
        o.push(("commentable_type".into(), self.commentable_type.to_string()));
        optional(o, "page", &self.page);
        optional(o, "limit", &self.limit);
        optional_bool(o, "desc", self.desc);
    }
}

/// Topic listing query.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ListTopicsQuery {
    /// One-based page.
    pub page: Option<u64>,
    /// Limit (maximum 30).
    pub limit: Option<u64>,
    /// Forum selection.
    pub forum: Option<TopicForum>,
    /// Linked item id; use with linked_type.
    pub linked_id: Option<u64>,
    /// Linked item type; use with linked_id.
    pub linked_type: Option<TopicLinkedType>,
    /// Detailed topic subtype filter.
    pub topic_type: Option<TopicType>,
}
impl QueryParameters for ListTopicsQuery {
    fn append_to(&self, o: &mut Vec<(String, String)>) {
        optional(o, "page", &self.page);
        optional(o, "limit", &self.limit);
        optional(o, "forum", &self.forum);
        optional(o, "linked_id", &self.linked_id);
        optional(o, "linked_type", &self.linked_type);
        optional(o, "type", &self.topic_type);
    }
}

/// User listing query.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ListUsersQuery {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub page: Option<u64>,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub limit: Option<u64>,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub search: Option<String>,
}
impl QueryParameters for ListUsersQuery {
    fn append_to(&self, o: &mut Vec<(String, String)>) {
        optional(o, "page", &self.page);
        optional(o, "limit", &self.limit);
        optional(o, "search", &self.search);
    }
}

/// User rate list query for v2.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ListUserRatesV2Query {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub user_id: Option<u64>,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub target_id: Option<u64>,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub target_type: Option<RateTargetType>,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub status: Option<UserRateStatus>,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub page: Option<u64>,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub limit: Option<u64>,
}
impl QueryParameters for ListUserRatesV2Query {
    fn append_to(&self, o: &mut Vec<(String, String)>) {
        optional(o, "user_id", &self.user_id);
        optional(o, "target_id", &self.target_id);
        optional(o, "target_type", &self.target_type);
        optional(o, "status", &self.status);
        optional(o, "page", &self.page);
        optional(o, "limit", &self.limit);
    }
}

/// Query for a user's anime rate list.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct UserAnimeRatesQuery {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub page: Option<u64>,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub limit: Option<u64>,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub status: Option<UserRateStatus>,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub censored: Option<bool>,
}
impl QueryParameters for UserAnimeRatesQuery {
    fn append_to(&self, o: &mut Vec<(String, String)>) {
        optional(o, "page", &self.page);
        optional(o, "limit", &self.limit);
        optional(o, "status", &self.status);
        optional_bool(o, "censored", self.censored);
    }
}

/// Query for a user's manga rate list.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct UserMangaRatesQuery {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub page: Option<u64>,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub limit: Option<u64>,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub censored: Option<bool>,
}
impl QueryParameters for UserMangaRatesQuery {
    fn append_to(&self, o: &mut Vec<(String, String)>) {
        optional(o, "page", &self.page);
        optional(o, "limit", &self.limit);
        optional_bool(o, "censored", self.censored);
    }
}

/// Query for current user's message folder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserMessagesQuery {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub message_type: MessageFolder,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub page: Option<u64>,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub limit: Option<u64>,
}
impl QueryParameters for UserMessagesQuery {
    fn append_to(&self, o: &mut Vec<(String, String)>) {
        o.push(("type".into(), self.message_type.to_string()));
        optional(o, "page", &self.page);
        optional(o, "limit", &self.limit);
    }
}

/// User history query.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct UserHistoryQuery {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub page: Option<u64>,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub limit: Option<u64>,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub target_id: Option<u64>,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub target_type: Option<RateTargetType>,
}
impl QueryParameters for UserHistoryQuery {
    fn append_to(&self, o: &mut Vec<(String, String)>) {
        optional(o, "page", &self.page);
        optional(o, "limit", &self.limit);
        optional(o, "target_id", &self.target_id);
        optional(o, "target_type", &self.target_type);
    }
}

/// Typed list include/exclude value; `Exclude(x)` serializes as `!x`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListValue<T> {
    /// Вариант, поддерживаемый публичным API.
    Include(T),
    /// Вариант, поддерживаемый публичным API.
    Exclude(T),
}
impl<T: ToString> ::std::fmt::Display for ListValue<T> {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Include(value) => formatter.write_str(&value.to_string()),
            Self::Exclude(value) => write!(formatter, "!{}", value.to_string()),
        }
    }
}

macro_rules! string_enum {
    ($name:ident { $($variant:ident => $value:literal),+ $(,)? }) => {
        #[doc = concat!("Строго типизированное input-перечисление Shikimori REST API `", stringify!($name), "`.")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name {
            $(
                #[doc = concat!("Wire value `", $value, "`, поддерживаемое Shikimori REST API.")]
                $variant,
            )+
        }
        impl ::std::fmt::Display for $name { fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result { f.write_str(match self { $(Self::$variant => $value),+ }) } }
        impl ::serde::Serialize for $name { fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error> where S: ::serde::Serializer { serializer.serialize_str(&self.to_string()) } }
    };
}

string_enum!(AnimeOrder { Id=>"id", IdDesc=>"id_desc", Ranked=>"ranked", Kind=>"kind", Popularity=>"popularity", Name=>"name", AiredOn=>"aired_on", Episodes=>"episodes", Status=>"status", Random=>"random", RankedRandom=>"ranked_random", RankedShiki=>"ranked_shiki", CreatedAt=>"created_at", CreatedAtDesc=>"created_at_desc", UpdatedAt=>"updated_at", UpdatedAtDesc=>"updated_at_desc" });
string_enum!(MangaOrder { Id=>"id", IdDesc=>"id_desc", Ranked=>"ranked", Kind=>"kind", Popularity=>"popularity", Name=>"name", AiredOn=>"aired_on", Volumes=>"volumes", Chapters=>"chapters", Status=>"status", Random=>"random", RankedRandom=>"ranked_random", RankedShiki=>"ranked_shiki", CreatedAt=>"created_at", CreatedAtDesc=>"created_at_desc", UpdatedAt=>"updated_at", UpdatedAtDesc=>"updated_at_desc" });
string_enum!(AnimeKind { Tv=>"tv", Movie=>"movie", Ova=>"ova", Ona=>"ona", Special=>"special", TvSpecial=>"tv_special", Music=>"music", Pv=>"pv", Cm=>"cm", Tv13=>"tv_13", Tv24=>"tv_24", Tv48=>"tv_48" });
string_enum!(AnimeStatus { Anons=>"anons", Ongoing=>"ongoing", Released=>"released" });
string_enum!(MangaKind { Manga=>"manga", Manhwa=>"manhwa", Manhua=>"manhua", LightNovel=>"light_novel", Novel=>"novel", OneShot=>"one_shot", Doujin=>"doujin" });
string_enum!(MangaStatus { Anons=>"anons", Ongoing=>"ongoing", Released=>"released", Paused=>"paused", Discontinued=>"discontinued" });
string_enum!(AnimeDuration { Short=>"S", Medium=>"D", Long=>"F" });
string_enum!(AgeRating { None=>"none", G=>"g", Pg=>"pg", Pg13=>"pg_13", R=>"r", RPlus=>"r_plus", Rx=>"rx" });
string_enum!(UserRateStatus { Planned=>"planned", Watching=>"watching", Rewatching=>"rewatching", Completed=>"completed", OnHold=>"on_hold", Dropped=>"dropped" });
string_enum!(PersonKind { Seyu=>"seyu", Mangaka=>"mangaka", Producer=>"producer" });
string_enum!(CommentListTarget { Topic=>"Topic", User=>"User" });
string_enum!(CommentableType { Topic=>"Topic", User=>"User", Anime=>"Anime", Manga=>"Manga", Character=>"Character", Person=>"Person", Article=>"Article", Club=>"Club", ClubPage=>"ClubPage", Collection=>"Collection", Critique=>"Critique", Review=>"Review" });
string_enum!(RateTargetType { Anime=>"Anime", Manga=>"Manga" });
string_enum!(FavouriteType { Anime=>"Anime", Manga=>"Manga", Ranobe=>"Ranobe", Person=>"Person", Character=>"Character" });
string_enum!(PersonFavouriteKind { Common=>"common", Seyu=>"seyu", Mangaka=>"mangaka", Producer=>"producer", Person=>"person" });
string_enum!(ClubCommentPolicy { Free=>"free", Members=>"members", Admins=>"admins" });
string_enum!(ClubTopicPolicy { Members=>"members", Admins=>"admins" });
string_enum!(ClubImageUploadPolicy { Members=>"members", Admins=>"admins" });
string_enum!(StyleOwnerType { User=>"User", Club=>"Club" });
string_enum!(ReviewOpinion { Positive=>"positive", Neutral=>"neutral", Negative=>"negative" });
string_enum!(MessageFolder { Inbox=>"inbox", Private=>"private", Sent=>"sent", News=>"news", Notifications=>"notifications" });
string_enum!(VideoKind { Pv=>"pv", CharacterTrailer=>"character_trailer", Cm=>"cm", Op=>"op", Ed=>"ed", OpEdClip=>"op_ed_clip", Clip=>"clip", Other=>"other", EpisodePreview=>"episode_preview" });
string_enum!(TopicForum { All=>"all", Animanga=>"animanga", Site=>"site", Games=>"games", Vn=>"vn", Contests=>"contests", Offtopic=>"offtopic", Clubs=>"clubs", MyClubs=>"my_clubs", Critiques=>"critiques", News=>"news", Collections=>"collections", Articles=>"articles", Cosplay=>"cosplay" });
string_enum!(TopicLinkedType { Anime=>"Anime", Manga=>"Manga", Ranobe=>"Ranobe", Character=>"Character", Person=>"Person", Club=>"Club", ClubPage=>"ClubPage", Critique=>"Critique", Review=>"Review", Contest=>"Contest", CosplayGallery=>"CosplayGallery", Collection=>"Collection", Article=>"Article" });
string_enum!(TopicType { Topic=>"Topic", ClubUserTopic=>"Topics::ClubUserTopic", EntryTopic=>"Topics::EntryTopic", AnimeTopic=>"Topics::EntryTopics::AnimeTopic", ArticleTopic=>"Topics::EntryTopics::ArticleTopic", CharacterTopic=>"Topics::EntryTopics::CharacterTopic", ClubPageTopic=>"Topics::EntryTopics::ClubPageTopic", ClubTopic=>"Topics::EntryTopics::ClubTopic", CollectionTopic=>"Topics::EntryTopics::CollectionTopic", ContestTopic=>"Topics::EntryTopics::ContestTopic", CosplayGalleryTopic=>"Topics::EntryTopics::CosplayGalleryTopic", MangaTopic=>"Topics::EntryTopics::MangaTopic", PersonTopic=>"Topics::EntryTopics::PersonTopic", RanobeTopic=>"Topics::EntryTopics::RanobeTopic", CritiqueTopic=>"Topics::EntryTopics::CritiqueTopic", ReviewTopic=>"Topics::EntryTopics::ReviewTopic", NewsTopic=>"Topics::NewsTopic", ContestStatusTopic=>"Topics::NewsTopics::ContestStatusTopic" });
string_enum!(TopicRootType { Topic=>"Topic" });
string_enum!(PrivateMessageKind { Private=>"Private" });
string_enum!(BulkMessageKind { News=>"news", Notifications=>"notifications" });

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct AppearRequest {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub ids: Option<String>,
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct UpdateClubRequest {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub club: ClubInput,
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct ClubInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub display_images: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub comment_policy: Option<ClubCommentPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub topic_policy: Option<ClubTopicPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub image_upload_policy: Option<ClubImageUploadPolicy>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct CreateCommentRequest {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub comment: NewComment,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub frontend: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub broadcast: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct NewComment {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub body: String,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub commentable_id: u64,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub commentable_type: CommentableType,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub is_offtopic: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct UpdateCommentRequest {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub comment: CommentPatch,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub frontend: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct CommentPatch {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct CreateTopicRequest {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub topic: NewTopic,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct NewTopic {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub body: String,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub forum_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub linked_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub linked_type: Option<TopicLinkedType>,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub title: String,
    #[serde(rename = "type")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub topic_type: TopicRootType,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub user_id: u64,
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct UpdateTopicRequest {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub topic: TopicPatch,
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct TopicPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub linked_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub linked_type: Option<TopicLinkedType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct CreateReviewRequest {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub review: NewReview,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub frontend: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct NewReview {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub anime_id: u64,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub body: String,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub opinion: ReviewOpinion,
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct UpdateReviewRequest {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub review: ReviewPatch,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub frontend: Option<bool>,
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct ReviewPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub opinion: Option<ReviewOpinion>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct CreateMessageRequest {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub message: NewMessage,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub frontend: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct NewMessage {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub body: String,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub from_id: u64,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub kind: PrivateMessageKind,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub to_id: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct UpdateMessageRequest {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub message: MessagePatch,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub frontend: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct MessagePatch {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub body: String,
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct MarkMessagesReadRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub ids: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub is_read: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct MessageBulkRequest {
    #[serde(rename = "type")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub message_type: BulkMessageKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub frontend: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct FavouriteTarget {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub linked_type: FavouriteType,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub linked_id: u64,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub kind: Option<PersonFavouriteKind>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct CreateUserRateRequest {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub user_rate: NewUserRate,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct NewUserRate {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub user_id: u64,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub target_id: u64,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub target_type: RateTargetType,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub status: Option<UserRateStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub score: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub chapters: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub episodes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub volumes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub rewatches: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub text: Option<String>,
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct UpdateUserRateRequest {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub user_rate: UserRatePatch,
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct UserRatePatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub status: Option<UserRateStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub score: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub chapters: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub episodes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub volumes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub rewatches: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct CreateTopicIgnoreV1Request {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub topic_ignore: TopicIgnoreInput,
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct TopicIgnoreInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub topic_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub user_id: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct CreateStyleRequest {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub style: NewStyle,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct NewStyle {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub css: String,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub name: String,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub owner_id: u64,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub owner_type: StyleOwnerType,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct StylePreviewRequest {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub style: StyleCss,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct StyleCss {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub css: String,
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct UpdateStyleRequest {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub style: StylePatch,
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct StylePatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub css: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct CreateVideoRequest {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub video: NewVideo,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct NewVideo {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub kind: VideoKind,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub name: String,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub url: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct AbuseRequestInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub comment_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub topic_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct EpisodeNotificationRequest {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub episode_notification: NewEpisodeNotification,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub token: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Структура данных публичного контракта Shikimori REST API.
pub struct NewEpisodeNotification {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub anime_id: u64,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub episode: u64,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub aired_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub is_fandub: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub is_raw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub is_subtitles: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub is_anime365: Option<bool>,
}

/// Metadata and bytes to upload with `POST /api/user_images`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserImageUpload {
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub filename: String,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub bytes: Vec<u8>,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub linked_type: Option<String>,
    /// Поле, передаваемое или возвращаемое в contract Shikimori REST API.
    pub content_type: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn anime_filter_supports_documented_subtraction_mode() {
        let query = ListAnimesQuery {
            kind: Some(vec![
                ListValue::Include(AnimeKind::Tv),
                ListValue::Exclude(AnimeKind::Movie),
            ]),
            ..Default::default()
        };
        assert_eq!(query.pairs(), vec![("kind".into(), "tv,!movie".into())]);
    }
}
