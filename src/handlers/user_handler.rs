use axum::{
    Extension,
    Json,
    http::StatusCode,
};
use sqlx::MySqlPool;
use bcrypt::hash;
use validator::Validate;
use std::collections::HashMap;
use serde_json::{json, Value};

// import model user
use crate::models::user::User;

// import util response API
use crate::utils::response::ApiResponse;

// import schema request dan response user
use crate::schemas::user_schema::{
    UserStoreRequest,
    UserResponse,
};

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

pub async fn store(
    Extension(db): Extension<MySqlPool>,
    Json(payload): Json<UserStoreRequest>,
) -> (StatusCode, Json<ApiResponse<Value>>) {

    // Validasi Request
    if let Err(errors) = payload.validate() {
        let mut field_errors: HashMap<String, Vec<String>> = HashMap::new();

        // kumpulkan semua error dari validasi
        for (field, errors) in errors.field_errors() {
            let messages = errors
                .iter()
                .filter_map(|e| e.message.as_ref())
                .map(|m| m.to_string())
                .collect::<Vec<String>>();

            field_errors.insert(field.to_string(), messages);
        }

        return (
            // kirim response 422 Unprocessable Entity
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ApiResponse {
                status: false,
                message: "Validasi Gagal".to_string(),
                data: Some(json!(field_errors)),
            }),
        );
    }

    // Hash Password Dengan Bcrypt
    let password = match hash(payload.password, 10) {
        Ok(hashed) => hashed,
        Err(_) => {
            return (
                // kirim response 500 Internal Server Error
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "Gagal mengenkripsi password",
                )),
            );
        }
    };

    // Insert Data User ke Database
    let result = sqlx::query!(
        "INSERT INTO users (name, email, password) VALUES (?, ?, ?)",
        payload.name,
        payload.email,
        password
    )
    .execute(&db)
    .await;

    match result {
        Ok(result) => {

            // get id user yang baru saja dibuat
            let user_id = result.last_insert_id() as i64;

            // Ambil data user berdasarkan id
            let user = sqlx::query!(
                r#"
                SELECT id, name, email, created_at, updated_at
                FROM users
                WHERE id = ?
                "#,
                user_id
            )
            .fetch_one(&db)
            .await;

            match user {
                Ok(user) => {
                    let response = UserResponse {
                        id: user.id,
                        name: user.name,
                        email: user.email,
                        created_at: user.created_at,
                        updated_at: user.updated_at,
                    };

                    (
                        // kirim response 201 Created
                        StatusCode::CREATED,
                        Json(ApiResponse::success(
                            "User berhasil ditambahkan",
                            json!(response),
                        )),
                    )
                }
                Err(_) => (
                    // kirim response 500 Internal Server Error
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::error(
                        "Gagal mengambil data user",
                    )),
                ),
            }
        }
        Err(e) => {
            if e.to_string().contains("Duplicate entry") {
                (
                    // kirim response 409 Conflict
                    StatusCode::CONFLICT,
                    Json(ApiResponse::error(
                        "Email sudah terdaftar",
                    )),
                )
            } else {
                (
                    // kirim response 500 Internal Server Error
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::error(
                        "Gagal menambahkan user",
                    )),
                )
            }
        }
    }
}
