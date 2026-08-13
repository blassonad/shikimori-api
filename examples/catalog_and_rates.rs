//! Запуск: `cargo run --example catalog_and_rates`.
//!
//! Установите `SHIKIMORI_ACCESS_TOKEN`, только если хотите раскомментировать
//! изменяющую v2-операцию; пример по умолчанию выполняет один публичный GET.

use shikimori_api::{AnimeKind, ClientConfig, ListAnimesQuery, ListValue, ShikimoriClient};

#[tokio::main]
async fn main() -> Result<(), shikimori_api::Error> {
    let mut builder = ClientConfig::builder("shikimori-api-rust-example/0.1");
    if let Ok(token) = std::env::var("SHIKIMORI_ACCESS_TOKEN") {
        builder = builder.access_token(token);
    }
    let client = ShikimoriClient::new(builder.build()?);

    let query = ListAnimesQuery {
        search: Some("Cowboy Bebop".into()),
        kind: Some(vec![ListValue::Include(AnimeKind::Tv)]),
        limit: Some(10),
        ..Default::default()
    };
    let page = client.list_animes(query).await?;

    println!(
        "Получено {} записей (сервер может вернуть N+1 для пагинации):",
        page.len()
    );
    for anime in page {
        println!(
            "#{:>6} {:<40} status={:?}",
            anime.id, anime.name, anime.status
        );
    }

    // Изменяющие вызовы намеренно не выполняются автоматически. Пример их формы:
    // use shikimori_api::{CreateUserRateRequest, NewUserRate, RateTargetType, UserRateStatus};
    // let rate = client.create_user_rate_v2(CreateUserRateRequest {
    //     user_rate: NewUserRate {
    //         user_id: 123,
    //         target_id: 1,
    //         target_type: RateTargetType::Anime,
    //         status: Some(UserRateStatus::Watching),
    //         score: None, chapters: None, episodes: Some(1), volumes: None,
    //         rewatches: None, text: None,
    //     },
    // }).await?;
    // println!("Создана/возвращена оценка {}", rate.id);

    Ok(())
}
