use axum::{Router, Extension};
use dotenvy::dotenv;
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};

mod config;
mod routes;
mod handlers;
mod schemas;
mod utils;
mod models;
mod middlewares;

#[tokio::main]
async fn main() {
    
    // Load environment variables from .env file
    dotenv().ok();

    // koneksi ke database
    let db = config::database::connect().await;

    // Konfigurasi CORS
    let cors = CorsLayer::new()
        .allow_origin(Any) // Izinkan semua origin
        .allow_methods(Any) // Izinkan semua method (GET, POST, dll)
        .allow_headers(Any);

    // Buat router dasar
    let app = Router::new()
        .merge(routes::auth_routes::auth_routes())
        .merge(routes::user_routes::user_routes())
        .layer(Extension(db))
        .layer(cors);

    // Ambil port dari environment variable, default 3000
    let port = std::env::var("APP_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3001);

    // Alamat server
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    
    // Tampilkan alamat server di console
    println!("Server running on http://{}", addr);
    
    // Jalankan server
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app
    ).await.unwrap();
}
