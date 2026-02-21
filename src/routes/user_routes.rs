use axum::{
    Router,
    routing::{get, post},
    middleware,
};

// import handler user
use crate::handlers::user_handler::{
    index,
    store
};

// import middleware auth
use crate::middlewares::auth_middleware::auth;

pub fn user_routes() -> Router {
    Router::new()
        // GET /api/users → list semua user
        .route("/api/users", get(index))

        // POST /api/users → tambah user
        .route("/api/users", post(store))
        
        // Semua route di atas WAJIB login
        .layer(middleware::from_fn(auth))
}
