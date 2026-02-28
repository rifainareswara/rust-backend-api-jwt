# Alur Proyek Backend API JWT

Dokumen ini menyimpan detail teknis agar README tetap ringkas.

## Urutan Pengerjaan yang Disarankan
Urutan ini menjaga dependency antar modul agar tidak saling bertabrakan:

1. **Buat proyek baru**
	- `cargo new backend-api-jwt`
	- Masuk ke folder proyek.
2. **Pasang dependency utama**
	- `axum`, `tokio`, `serde`, `serde_json`, `sqlx`, `dotenvy`, `bcrypt`, `jsonwebtoken`, `chrono`, `tower-http`, `validator`.
3. **Siapkan environment**
	- Buat `.env` dan isi `APP_PORT`, `DATABASE_URL`, `JWT_SECRET`.
4. **Konfigurasi database**
	- Buat `src/config/database.rs` dan `src/config/mod.rs`.
5. **Server dasar**
	- Setup `Router::new()` dan `Extension(db)` di `src/main.rs`.
6. **Migrasi database**
	- `sqlx migrate add create_users_table`
	- `sqlx migrate run`
7. **Model data**
	- `src/models/user.rs` dan `src/models/mod.rs`.
8. **Schema request/response**
	- `src/schemas/register_schema.rs`, `src/schemas/login_schema.rs`, `src/schemas/user_schema.rs`, dan `src/schemas/mod.rs`.
9. **Utility**
	- `src/utils/jwt.rs`, `src/utils/response.rs`, `src/utils/mod.rs`.
10. **Middleware auth**
	- `src/middlewares/auth_middleware.rs` dan `src/middlewares/mod.rs`.
11. **Handler auth dan user**
	- `src/handlers/register_handler.rs`, `src/handlers/login_handler.rs`, `src/handlers/user_handler.rs`, dan `src/handlers/mod.rs`.
12. **Route auth dan user**
	- `src/routes/auth_routes.rs`, `src/routes/user_routes.rs`, dan `src/routes/mod.rs`.
13. **Wire route ke Router**
	- Merge semua route di `src/main.rs`.

## Alur Aplikasi Saat Ini
1. Memuat variabel environment dari `.env` menggunakan `dotenvy`.
2. Membuat koneksi MySQL melalui `config::database::connect()`.
3. Menyiapkan router Axum dan merge route auth dan user.
4. Menyimpan koneksi DB di layer `Extension`.
5. Membaca `APP_PORT` dari env (default `3001`).
6. Menjalankan server HTTP pada `127.0.0.1:<port>`.

## Komponen yang Sudah Disiapkan
Daftar modul yang sudah tersedia:

- **Schemas**
	- `RegisterRequest` dengan validasi panjang nama, email, dan password.
	- `RegisterResponse` untuk response data user (id, name, email, created_at, updated_at).
	- `LoginRequest` dengan validasi email dan password.
	- `LoginResponse` berisi data user (id, name, email) dan token.
	- `UserStoreRequest` dengan validasi nama, email, dan password (di `user_schema.rs`).
	- `UserResponse` untuk format data user dengan timestamp (di `user_schema.rs`).
- **Middleware Auth**
	- `auth` membaca token dari header `Authorization: Bearer <token>`.
	- Menggunakan `verify_token()` dan menyimpan `claims` di `extensions`.
- **Utils**
	- `jwt` untuk `generate_token()` dan `verify_token()`.
	- `response` untuk format `ApiResponse`.
- **Handlers**
	- `register_handler` untuk membuat user.
	- `login_handler` untuk autentikasi dan token.
	- `user_handler` untuk list user.
- **Routes**
	- `auth_routes`: `/api/register` dan `/api/login` (public).
	- `user_routes`: `/api/users` (protected, butuh token).

## Migrasi Database (SQLx)
Migrasi untuk tabel `users` sudah dibuat dan dijalankan.

Perintah yang digunakan:
```
sqlx migrate add create_users_table
sqlx migrate run
```

File migrasi:
- `migrations/20260205084808_create_users_table.sql`

Skema tabel `users`:
- `id` (BIGINT, auto increment, primary key)
- `name` (VARCHAR(100))
- `email` (VARCHAR(100), unique)
- `password` (TEXT)
- `created_at` (TIMESTAMP, default current timestamp)
- `updated_at` (TIMESTAMP, auto update)

## Model User
Model user sudah dibuat di `src/models/user.rs` untuk kebutuhan API.

Field yang tersedia:
- `id`
- `name`
- `email`
- `created_at`
- `updated_at`

Model `User` digunakan langsung oleh `user_handler` untuk serialisasi response list user.

## Alur Pengerjaan Dari Awal Sampai Saat Ini
Berikut urutan kerja yang dilakukan dari awal proyek sampai kondisi sekarang:

1. **Buat proyek baru**
	- `cargo new backend-api-jwt`
	- Masuk ke folder proyek.
2. **Siapkan live reload (opsional)**
	- `cargo install cargo-watch`
	- `cargo watch -q -c -w src/ -x run`
3. **Tambah web framework**
	- `cargo add axum`
4. **Tambah async runtime**
	- `cargo add tokio --features full`
5. **Serialisasi data**
	- `cargo add serde --features derive`
	- `cargo add serde_json`
6. **Koneksi database**
	- `cargo add sqlx --features mysql,runtime-tokio,macros,chrono`
7. **Password hashing**
	- `cargo add bcrypt`
8. **JWT**
	- `cargo add jsonwebtoken --features aws_lc_rs`
9. **Load env**
	- `cargo add dotenvy`
10. **Tanggal dan waktu**
	- `cargo add chrono --features serde`
11. **HTTP middleware dan CORS**
	- `cargo add tower-http --features cors`
12. **Validasi request**
	- `cargo add validator --features derive`
13. **Konfigurasi environment**
	- Buat `.env` dan isi `APP_PORT`, `DATABASE_URL`, `JWT_SECRET`.
14. **Setup koneksi database**
	- Buat `src/config/database.rs` dan `src/config/mod.rs`.
15. **Router dan server dasar**
	- Buat router kosong di `src/main.rs` dan jalankan server.
16. **Buat migrasi users**
	- `sqlx migrate add create_users_table`
	- `sqlx migrate run`
17. **Buat model user**
	- Tambah `src/models/user.rs` dan `src/models/mod.rs`.
18. **Buat schema register**
	- Tambah `src/schemas/register_schema.rs` dan `src/schemas/mod.rs`.
19. **Tambah util JWT dan response**
	- Tambah `src/utils/jwt.rs`, `src/utils/response.rs`, dan `src/utils/mod.rs`.
20. **Tambah middleware auth**
	- Tambah `src/middlewares/auth_middleware.rs` dan `src/middlewares/mod.rs`.
21. **Tambah schema login dan user**
	- Tambah `src/schemas/login_schema.rs`, `src/schemas/user_schema.rs`, dan update `src/schemas/mod.rs`.
22. **Tambah handler auth dan user**
	- Tambah `src/handlers/register_handler.rs`, `src/handlers/login_handler.rs`, `src/handlers/user_handler.rs`, dan `src/handlers/mod.rs`.
23. **Tambah route auth dan user**
	- Tambah `src/routes/auth_routes.rs`, `src/routes/user_routes.rs`, dan `src/routes/mod.rs`.
	- Merge semua route ke `Router` di `src/main.rs`.

## Alur Register
1. Client mengirim `POST /api/register`.
2. Payload divalidasi (nama, email, password).
3. Password di-hash dengan `bcrypt`.
4. Data user disimpan ke tabel `users`.
5. Ambil kembali data user dan kirim response `201 Created`.
6. Jika email sudah ada, kirim `409 Conflict`.

## Alur Login
1. Client mengirim `POST /api/login`.
2. Payload divalidasi (email, password).
3. Ambil user berdasarkan email.
4. Verifikasi password dengan `bcrypt`.
5. Generate JWT (24 jam) dengan `JWT_SECRET`.
6. Kirim response `200 OK` berisi data user dan token.
7. Jika email/password salah, kirim `401 Unauthorized`.

## Alur List Users (Protected)
1. Client mengirim `GET /api/users` dengan header `Authorization: Bearer <token>`.
2. Middleware `auth` memverifikasi token dan menyimpan `claims` ke `extensions`.
3. Handler mengambil semua data user dari database.
4. Kirim response `200 OK` berisi daftar user.

## Catatan Environment
- `JWT_SECRET` dipakai untuk sign/verify JWT. Jika tidak diset, default ke `secret`.
