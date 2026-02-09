use axum::{
    Extension,
    Json,
    http::StatusCode,
};
use sqlx::MySqlPool;
use serde_json::{json, Value};

// import model user
use crate::models::user::User;

// import util response API
use crate::utils::response::ApiResponse;

pub async fn index(
    Extension(db): Extension<MySqlPool>,
) -> (StatusCode, Json<ApiResponse<Value>>) {

    // Ambil seluruh data user
    let users = match sqlx::query_as!(
        User,
        r#"
        SELECT id, name, email, created_at, updated_at
        FROM users
        ORDER BY id DESC
        "#
    )
    .fetch_all(&db)
    .await
    {
        Ok(users) => users,
        Err(e) => {
            eprintln!("Database error: {}", e);
            return (
                // kirim response 500 Internal Server Error
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "Gagal mengambil data user",
                )),
            );
        }
    };

    (
        // kirim response 200 OK
        StatusCode::OK,
        Json(ApiResponse::success(
            "List user",
            json!(users),
        )),
    )
}
