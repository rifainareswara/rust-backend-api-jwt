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
		handlers/
			mod.rs
			register_handler.rs
			login_handler.rs
			user_handler.rs
		middlewares/
			mod.rs
			auth_middleware.rs
		models/
			mod.rs
			user.rs
		routes/
			mod.rs
			auth_routes.rs
			user_routes.rs
		schemas/
			mod.rs
			register_schema.rs
			login_schema.rs
			user_schema.rs
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
APP_PORT=3001
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
Server running on http://127.0.0.1:3001
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

### Public Endpoints (Tidak perlu token)
- `POST /api/register` - Registrasi user baru
- `POST /api/login` - Login dan dapatkan token

### Protected Endpoints (Butuh header `Authorization: Bearer <token>`)
- `GET /api/users` - Mengambil list semua user
- `POST /api/users` - Menambahkan user baru (untuk admin/user yang sudah login)

Middleware auth sudah dipasang pada semua route user.

### Contoh Penggunaan

**Register (Public)**
```bash
curl -X POST http://127.0.0.1:3001/api/register \
  -H "Content-Type: application/json" \
  -d '{"name":"John Doe","email":"john@example.com","password":"password123"}'
```

**Login (Public)**
```bash
curl -X POST http://127.0.0.1:3001/api/login \
  -H "Content-Type: application/json" \
  -d '{"email":"john@example.com","password":"password123"}'
```

**List Users (Protected)**
```bash
curl -H "Authorization: Bearer <token>" http://127.0.0.1:3001/api/users
```

**Create User (Protected)**
```bash
curl -X POST http://127.0.0.1:3001/api/users \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{"name":"Jane Doe","email":"jane@example.com","password":"password123"}'
```


