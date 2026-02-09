# Backend API JWT (Rust + Axum)

Dokumentasi ini merapikan alur proyek dari awal sampai saat ini. Server sudah memiliki route auth dasar (register/login) dan list user (protected), dan struktur, koneksi database, serta dependency siap untuk JWT, validasi, dan middleware.

## Tujuan
- Menyediakan backend API dengan autentikasi JWT.
- Terhubung ke MySQL via `sqlx`.
- Siap untuk validasi request, hashing password, dan CORS.

## Prasyarat
- Rust toolchain (stable)
- Cargo
- MySQL (jika akan konek DB)

## Struktur Proyek
```
backend-api-jwt/
	Cargo.toml
	README.md
	src/
		main.rs
		config/
			mod.rs
			database.rs
		middlewares/
			mod.rs
			auth_middleware.rs
		models/
			mod.rs
			user.rs
		schemas/
			mod.rs
			register_schema.rs
			login_schema.rs
		utils/
			mod.rs
			jwt.rs
			response.rs
	migrations/
		20260205084808_create_users_table.sql
	Dokumentasi/
		ALUR.md
```

## Konfigurasi Environment
Buat file `.env` di root proyek:

```
APP_PORT=3000
DATABASE_URL=mysql://root:@localhost:3306/db_backend_api_jwt
JWT_SECRET=ubah_ke_secret_kamu
```

## Menjalankan Aplikasi
### Mode normal
```
cargo run
```

### Live reload (opsional)
```
cargo install cargo-watch
cargo watch -q -c -w src/ -x run
```

Output awal yang diharapkan:
```
Database Connected Successfully!
Server running on http://127.0.0.1:3000
```

## Alur dan Detail Teknis
Ringkasan alur dan detail teknis dipindahkan ke dokumen terpisah agar README tetap singkat.

Lihat [Dokumentasi/ALUR.md](Dokumentasi/ALUR.md).

## Dependensi Utama
- `axum`: web framework
- `tokio`: async runtime
- `serde` + `serde_json`: serialisasi data
- `sqlx`: koneksi MySQL
- `bcrypt`: hashing password
- `jsonwebtoken`: JWT
- `dotenvy`: load env
- `chrono`: tanggal dan waktu
- `tower-http`: middleware (CORS)
- `validator`: validasi request

## Catatan Endpoint
Endpoint yang sudah tersedia:

- `POST /api/register`
- `POST /api/login`
- `GET /api/users` (butuh header `Authorization: Bearer <token>`)

Middleware auth sudah dipasang pada route user.

Contoh request list user:
```
curl -H "Authorization: Bearer <token>" http://127.0.0.1:3000/api/users
```


