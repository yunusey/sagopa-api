use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde_json::json;
use sqlx::PgPool;
use std::collections::HashMap;
use tower_http::services::{ServeDir, ServeFile};

async fn add_new_video(
    Query(query): Query<HashMap<String, String>>,
    State(state): State<DatabaseState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if !query.contains_key("name") || !query.contains_key("word") {
        return Err((StatusCode::BAD_REQUEST, "Missing query parameters"));
    }

    let name = query.get("name").unwrap();
    let word = query.get("word").unwrap();

    match sqlx::query(
        r#"
        INSERT INTO videos (name, word)
        SELECT $1, $2
        WHERE NOT EXISTS (
            SELECT 1 FROM videos WHERE name = $1 AND word = $2
        );
        "#,
    )
    .bind(name)
    .bind(word)
    .execute(&state.pool)
    .await
    {
        Ok(_) => {}
        Err(e) => {
            println!("Failed to add video: {:?}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to add video"));
        }
    }

    Ok(Json(json!({
        "name": name.to_string(),
        "word": word.to_string(),
    })))
}

async fn get_videos(State(state): State<DatabaseState>) -> Json<HashMap<String, Vec<String>>> {
    let videos = sqlx::query_as::<_, (String, String)>(
        r#"
        SELECT DISTINCT name, word FROM videos;
        "#,
    )
    .fetch_all(&state.pool)
    .await
    .expect("Failed to fetch videos");

    println!("{:?}", videos);
    let mut result: HashMap<String, Vec<String>> = HashMap::new();
    for (name, word) in videos {
        if result.contains_key(&name) {
            result.get_mut(&name).unwrap().push(word);
        } else {
            result.insert(name, vec![word]);
        }
    }

    Json(result)
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
        .route("/add", get(add_new_video))
        .route("/get", get(get_videos))
        .with_state(state);

    Ok(router.into())
}
