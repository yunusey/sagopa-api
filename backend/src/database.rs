use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Song {
    pub album_id: i32,
    pub name: String,
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Album {
    pub name: String,
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Quote {
    pub song_id: i32,
    pub quote: String,
    pub id: i32,
}

/// Get all albums as a vector.
pub async fn get_all_albums(pool: &sqlx::PgPool) -> Vec<Album> {
    sqlx::query_as::<_, Album>(
        r#"
        SELECT id, name FROM albums;
        "#,
    )
    .fetch_all(pool)
    .await
    .expect("Failed to fetch albums")
    .into_iter()
    .collect()
}

/// Get album by id
#[allow(dead_code)]
pub async fn get_album_by_id(pool: &sqlx::PgPool, id: i32) -> Option<Album> {
    sqlx::query_as::<_, Album>(
        r#"
        SELECT id, name FROM albums
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .expect("Failed to fetch albums")
}

/// Get album by name
pub async fn get_album_by_name(pool: &sqlx::PgPool, name: &str) -> Option<Album> {
    sqlx::query_as::<_, Album>(
        r#"
        SELECT id, name FROM albums
        WHERE name = $1
        "#,
    )
    .bind(name)
    .fetch_optional(pool)
    .await
    .expect("Failed to fetch albums")
}

/// Get all songs as a vector
pub async fn get_all_songs(pool: &sqlx::PgPool) -> Vec<Song> {
    sqlx::query_as::<_, Song>(
        r#"
        SELECT album_id, name, id FROM songs
        "#,
    )
    .fetch_all(pool)
    .await
    .expect("Failed to fetch songs")
}

#[allow(dead_code)]
pub async fn get_song_by_id(pool: &sqlx::PgPool, id: i32) -> Option<Song> {
    sqlx::query_as::<_, Song>(
        r#"
        SELECT album_id, name, id FROM songs
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .expect("Failed to fetch songs")
}

/// Get song by name
pub async fn get_song_by_name(pool: &sqlx::PgPool, album_id: i32, name: &str) -> Option<Song> {
    sqlx::query_as::<_, Song>(
        r#"
        SELECT album_id, name, id FROM songs
        WHERE album_id = $1 AND name = $2
        "#,
    )
    .bind(album_id)
    .bind(name)
    .fetch_optional(pool)
    .await
    .expect("Failed to fetch songs")
}

/// Add a new song (if it exists, returns None)
pub async fn add_new_song(pool: &sqlx::PgPool, album_id: i32, name: &str) {
    let _ = sqlx::query_as::<_, Song>(
        r#"
        INSERT INTO songs (album_id, name)
        SELECT $1, $2
        WHERE NOT EXISTS (
            SELECT 1 FROM songs WHERE album_id = $1 AND name = $2
        )
        RETURNING id, album_id, name
        "#,
    )
    .bind(album_id)
    .bind(name)
    .fetch_one(pool)
    .await;
}

/// Add a new quote
pub async fn add_new_quote(pool: &sqlx::PgPool, song_id: i32, quote: &str) {
    let _ = sqlx::query(
        r#"
        INSERT INTO quotes (song_id, quote)
        SELECT $1, $2
        WHERE NOT EXISTS (
            SELECT 1 FROM quotes WHERE song_id = $1 AND quote = $2
        )
        "#,
    )
    .bind(song_id)
    .bind(quote)
    .execute(pool)
    .await;
}

/// Get quote by name
pub async fn get_quote_by_name(pool: &sqlx::PgPool, song_id: i32, name: &str) -> Option<Quote> {
    sqlx::query_as::<_, Quote>(
        r#"
        SELECT id, song_id, quote FROM quotes
        WHERE song_id = $1 AND quote = $2
        "#,
    )
    .bind(song_id)
    .bind(name)
    .fetch_optional(pool)
    .await
    .expect("Failed to fetch quotes")
}

/// Get random quote
pub async fn get_random_quote(pool: &sqlx::PgPool) -> Quote {
    sqlx::query_as::<_, Quote>(
        r#"
        SELECT id, song_id, quote FROM quotes
        ORDER BY random() LIMIT 1
        "#,
    )
    .fetch_one(pool)
    .await
    .expect("Failed to fetch quotes")
}

/// Get all quotes
pub async fn get_all_quotes(pool: &sqlx::PgPool) -> Vec<Quote> {
    sqlx::query_as::<_, Quote>(
        r#"
        SELECT id, song_id, quote FROM quotes
        "#,
    )
    .fetch_all(pool)
    .await
    .expect("Failed to fetch quotes")
}
