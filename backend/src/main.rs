mod database;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use database::{
    add_new_quote, add_new_song, get_album_by_name, get_all_albums, get_all_quotes, get_all_songs,
    get_quote_by_name, get_random_quote, get_song_by_name, Album, Quote, Song,
};
use serde_json::json;
use sqlx::PgPool;
use std::collections::HashMap;
use tower_http::services::{ServeDir, ServeFile};

async fn add_quote_handler(
    Query(query): Query<HashMap<String, String>>,
    State(state): State<DatabaseState>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    if !query.contains_key("album") || !query.contains_key("song") || !query.contains_key("quote") {
        return Err((StatusCode::BAD_REQUEST, "Missing query parameters"));
    }

    let album_name = query.get("album").unwrap();
    let song_name = query.get("song").unwrap();
    let quote_name = query.get("quote").unwrap();

    let album = get_album_by_name(&state.pool, album_name).await.unwrap();
    add_new_song(&state.pool, album.id, song_name).await;
    let song = get_song_by_name(&state.pool, album.id, song_name)
        .await
        .unwrap();
    add_new_quote(&state.pool, song.id, quote_name).await;
    let quote = get_quote_by_name(&state.pool, song.id, quote_name)
        .await
        .unwrap();

    Ok(Json(json!({
        "success": true,
        "quote": quote,
        "album": album,
        "song": song
    })))
}

async fn get_albums_handler(State(state): State<DatabaseState>) -> Json<Vec<Album>> {
    let albums = get_all_albums(&state.pool).await;
    Json(albums)
}

async fn get_songs_handler(State(state): State<DatabaseState>) -> Json<Vec<Song>> {
    let songs = get_all_songs(&state.pool).await;
    Json(songs)
}

async fn get_quotes_handler(State(state): State<DatabaseState>) -> Json<Vec<Quote>> {
    let quotes = get_all_quotes(&state.pool).await;
    Json(quotes)
}

async fn get_random_quote_handler(State(state): State<DatabaseState>) -> Json<Quote> {
    let quote = get_random_quote(&state.pool).await;
    Json(quote)
}

/// Database state
#[derive(Clone)]
struct DatabaseState {
    pool: PgPool,
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] db: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Failed to run migrations");
    let state = DatabaseState { pool: db };
    let router = Router::new()
        .nest_service(
            "/",
            ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html")),
        )
        .route("/get/albums", get(get_albums_handler))
        .route("/get/songs", get(get_songs_handler))
        .route("/add/quote", get(add_quote_handler))
        .route("/add/quote", post(add_quote_handler))
        .route("/get/quote", get(get_quotes_handler))
        .route("/get/random/quote", get(get_random_quote_handler))
        .with_state(state);

    Ok(router.into())
}
