use axum::{
    Router,
    routing::{get, post, put, delete},
    middleware,
};

// import handler user
use crate::handlers::user_handler::{
    index,
    store,
    show,
    update,
    destroy
};

// import middleware auth
use crate::middlewares::auth_middleware::auth;

pub fn user_routes() -> Router {
    Router::new()
        // GET /api/users → list semua user
        .route("/api/users", get(index))

        // POST /api/users → tambah user
        .route("/api/users", post(store))

        // GET /api/users/{id} → detail user
        .route("/api/users/{id}", get(show))

        // PUT /api/users/{id} → update user
        .route("/api/users/{id}", put(update))

        // DELETE /api/users/{id} → hapus user
        .route("/api/users/{id}", delete(destroy))
        
        // Semua route di atas WAJIB login
        .layer(middleware::from_fn(auth))
}
