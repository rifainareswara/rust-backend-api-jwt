use axum::{
    Router,
    routing::get,
    middleware,
};

// import handler user
use crate::handlers::user_handler::index;

// import middleware auth
use crate::middlewares::auth_middleware::auth;

pub fn user_routes() -> Router {
    Router::new()
        // GET /api/users → list semua user
        .route("/api/users", get(index))
        
        // Semua route di atas WAJIB login
        .layer(middleware::from_fn(auth))
}
