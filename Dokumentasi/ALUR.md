# Alur Proyek Backend API JWT

Dokumen ini menyimpan detail teknis agar README tetap ringkas.

## Daftar Isi
- [Urutan Pengerjaan yang Disarankan](#urutan-pengerjaan-yang-disarankan)
- [Alur Aplikasi Saat Ini](#alur-aplikasi-saat-ini)
- [Komponen yang Sudah Disiapkan](#komponen-yang-sudah-disiapkan)
- [Migrasi Database (SQLx)](#migrasi-database-sqlx)
- [Model User](#model-user)
- [Alur Pengerjaan Dari Awal Sampai Saat Ini](#alur-pengerjaan-dari-awal-sampai-saat-ini)
- [Alur Register](#alur-register)
- [Alur Login](#alur-login)
- [Alur List Users (Protected)](#alur-list-users-protected)
- [Alur Tambah User (Protected)](#alur-tambah-user-protected)
- [Alur Detail User (Protected)](#alur-detail-user-protected)
- [Alur Update User (Protected)](#alur-update-user-protected)
- [Alur Hapus User (Protected)](#alur-hapus-user-protected)
- [Format Response](#format-response-error-validasi)
- [Catatan Environment](#catatan-environment)
- [Best Practices](#best-practices)
- [Pengembangan Lebih Lanjut](#pengembangan-lebih-lanjut)

## Urutan Pengerjaan yang Disarankan
Urutan ini menjaga dependency antar modul agar tidak saling bertabrakan:

1. **Buat proyek baru**
	- `cargo new backend-api-jwt`
	- Masuk ke folder proyek.
2. **Pasang dependency utama**
	- `axum`, `tokio`, `serde`, `serde_json`, `sqlx`, `dotenvy`, `bcrypt`, `jsonwebtoken`, `chrono`, `tower-http`, `validator`.
3. **Siapkan environment**
	- Salin `.env.example` menjadi `.env`.
	- Sesuaikan nilai `APP_PORT`, `DATABASE_URL`, `JWT_SECRET` di file `.env`.
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
	- `src/schemas/register_schema.rs` dan `src/schemas/mod.rs`.
9. **Utility**
	- `src/utils/jwt.rs`, `src/utils/response.rs`, `src/utils/mod.rs`.
10. **Middleware auth**
	- `src/middlewares/auth_middleware.rs` dan `src/middlewares/mod.rs`.
11. **Schema login**
	- `src/schemas/login_schema.rs` dan `src/schemas/mod.rs`.
12. **Handler dan route auth**
	- Buat endpoint register/login dan wire ke `Router`.
13. **Handler dan route user**
	- Buat endpoint list user dan pasang middleware auth.

## Alur Aplikasi Saat Ini
1. Memuat variabel environment dari `.env` menggunakan `dotenvy`.
2. Membuat koneksi MySQL melalui `config::database::connect()`.
3. Menyiapkan router Axum dan merge route auth dan user.
4. Menyimpan koneksi DB di layer `Extension`.
5. Membaca `APP_PORT` dari env (default `3001` jika tidak diset).
6. Menjalankan server HTTP pada `127.0.0.1:<port>`.

## Komponen yang Sudah Disiapkan
Daftar modul yang sudah tersedia:

- **Schemas**
	- `register_schema.rs`
		- `RegisterRequest` dengan validasi panjang nama (min 3 karakter), email, dan password (min 6 karakter).
		- `RegisterResponse` untuk response data user (id, name, email, created_at, updated_at).
	- `login_schema.rs`
		- `LoginRequest` dengan validasi email dan password (min 6 karakter).
		- `UserResponse` berisi data user (id, name, email).
		- `LoginResponse` berisi data user dan token.
	- `user_schema.rs`
		- `UserStoreRequest` dengan validasi nama (min 3 karakter), email, dan password (min 6 karakter).
    - `UserUpdateRequest` dengan validasi nama (min 3 karakter), email valid, dan password opsional.
		- `UserResponse` untuk response data user (id, name, email, created_at, updated_at).
- **Middleware Auth**
	- `auth` membaca token dari header `Authorization: Bearer <token>`.
	- Menggunakan `verify_token()` dan menyimpan `claims` di `extensions`.
- **Utils**
	- `jwt` untuk `generate_token()` dan `verify_token()`.
		- Token berlaku selama 24 jam.
		- Menggunakan `JWT_SECRET` dari environment (default: "secret").
	- `response` untuk format `ApiResponse` (status, message, data).
- **Handlers**
	- `register_handler.rs`
		- `register` untuk membuat user baru melalui endpoint public (hash password dengan bcrypt).
	- `login_handler.rs`
		- `login` untuk autentikasi dan generate JWT token.
	- `user_handler.rs`
		- `index` untuk mengambil list semua user (descending by id).
		- `store` untuk menambahkan user baru (hanya untuk user yang sudah login).
    - `show` untuk mengambil detail user berdasarkan id.
    - `update` untuk memperbarui user berdasarkan id (opsional update password).
    - `destroy` untuk menghapus user berdasarkan id.
- **Routes**
	- Auth Routes (public):
		- `POST /api/register` - Registrasi user baru.
		- `POST /api/login` - Login dan dapatkan token.
	- User Routes (protected dengan auth middleware):
		- `GET /api/users` - List semua user.
		- `POST /api/users` - Tambah user baru.
    - `GET /api/users/{id}` - Detail user berdasarkan id.
    - `PUT /api/users/{id}` - Perbarui user berdasarkan id.
    - `DELETE /api/users/{id}` - Hapus user berdasarkan id.

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
	- Salin `.env.example` menjadi `.env`.
	- Sesuaikan nilai `APP_PORT`, `DATABASE_URL`, `JWT_SECRET` di file `.env`.
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
21. **Tambah schema login**
	- Tambah `src/schemas/login_schema.rs` dan update `src/schemas/mod.rs`.
22. **Tambah handler dan route auth**
	- Tambah `src/handlers/register_handler.rs`, `src/handlers/login_handler.rs`.
	- Tambah `src/routes/auth_routes.rs` dengan endpoint register dan login.
	- Merge `auth_routes()` ke `Router` di `main.rs`.
23. **Tambah handler dan route user**
	- Tambah `src/handlers/user_handler.rs` dengan handler `index` dan `store`.
	- Tambah `src/routes/user_routes.rs` dengan endpoint GET dan POST `/api/users`.
	- Pasang middleware auth pada semua route user.
	- Merge `user_routes()` ke `Router` di `main.rs`.

## Alur Register
1. Client mengirim `POST /api/register` dengan body JSON:
   ```json
   {
     "name": "John Doe",
     "email": "john@example.com",
     "password": "password123"
   }
   ```
2. Payload divalidasi menggunakan `validator`:
   - Nama minimal 3 karakter.
   - Email harus format valid.
   - Password minimal 6 karakter.
3. Jika validasi gagal, kirim response `422 Unprocessable Entity` dengan detail error per field.
4. Password di-hash dengan `bcrypt` (cost: 10).
5. Data user disimpan ke tabel `users`.
6. Ambil kembali data user berdasarkan `last_insert_id` dan kirim response `201 Created`.
7. Jika email sudah ada (duplicate entry), kirim `409 Conflict`.

## Alur Login
1. Client mengirim `POST /api/login` dengan body JSON:
   ```json
   {
     "email": "john@example.com",
     "password": "password123"
   }
   ```
2. Payload divalidasi menggunakan `validator`:
   - Email harus format valid.
   - Password minimal 6 karakter.
3. Jika validasi gagal, kirim response `422 Unprocessable Entity` dengan detail error per field.
4. Ambil user berdasarkan email dari database.
5. Jika user tidak ditemukan, kirim `401 Unauthorized`.
6. Verifikasi password dengan `bcrypt::verify()` terhadap hash di database.
7. Jika password tidak cocok, kirim `401 Unauthorized`.
8. Generate JWT token (berlaku 24 jam) menggunakan `JWT_SECRET`.
9. Kirim response `200 OK` berisi data user (id, name, email) dan token:
   ```json
   {
     "status": true,
     "message": "Login Berhasil",
     "data": {
       "user": {
         "id": 1,
         "name": "John Doe",
         "email": "john@example.com"
       },
       "token": "eyJ0eXAiOiJKV1QiLCJhbGc..."
     }
   }
   ```

## Alur List Users (Protected)
1. Client mengirim `GET /api/users` dengan header:
   ```
   Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGc...
   ```
2. Middleware `auth` memverifikasi token:
   - Jika header tidak ada atau format salah, kirim `401 Unauthorized`.
   - Jika token tidak valid atau expired, kirim `401 Unauthorized`.
   - Jika token valid, ekstrak claims dan simpan ke `extensions`.
3. Handler mengambil semua data user dari database (ORDER BY id DESC).
4. Kirim response `200 OK` berisi array daftar user:
   ```json
   {
     "status": true,
     "message": "List user",
     "data": [
       {
         "id": 2,
         "name": "Jane Doe",
         "email": "jane@example.com",
         "created_at": "2026-02-21T10:30:00Z",
         "updated_at": "2026-02-21T10:30:00Z"
       },
       {
         "id": 1,
         "name": "John Doe",
         "email": "john@example.com",
         "created_at": "2026-02-21T10:00:00Z",
         "updated_at": "2026-02-21T10:00:00Z"
       }
     ]
   }
   ```

## Alur Tambah User (Protected)
1. Client mengirim `POST /api/users` dengan header:
   ```
   Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGc...
   ```
   Dan body JSON:
   ```json
   {
     "name": "Jane Doe",
     "email": "jane@example.com",
     "password": "password123"
   }
   ```
2. Middleware `auth` memverifikasi token (sama seperti alur list users).
3. Payload divalidasi menggunakan `validator`:
   - Nama minimal 3 karakter.
   - Email harus format valid.
   - Password minimal 6 karakter.
4. Jika validasi gagal, kirim response `422 Unprocessable Entity` dengan detail error per field.
5. Password di-hash dengan `bcrypt` (cost: 10).
6. Data user disimpan ke tabel `users`.
7. Ambil kembali data user berdasarkan `last_insert_id` dan kirim response `201 Created`:
   ```json
   {
     "status": true,
     "message": "User berhasil ditambahkan",
     "data": {
       "id": 3,
       "name": "Jane Doe",
       "email": "jane@example.com",
       "created_at": "2026-02-21T11:00:00Z",
       "updated_at": "2026-02-21T11:00:00Z"
     }
   }
   ```
8. Jika email sudah ada (duplicate entry), kirim `409 Conflict`.

**Perbedaan dengan `/api/register`:**
- `/api/register` adalah endpoint public untuk self-registration.
- `/api/users` (POST) adalah endpoint protected untuk menambahkan user (fungsi admin).
- Keduanya menggunakan validasi dan hashing password yang sama.

## Alur Detail User (Protected)
1. Client mengirim `GET /api/users/{id}` dengan header `Authorization: Bearer <token>`.
2. Middleware `auth` memverifikasi token.
3. Handler `show` mencari user berdasarkan `id`.
4. Jika user ditemukan, kirim `200 OK` dengan message `Detail user`.
5. Jika user tidak ditemukan, kirim `404 Not Found`.

## Alur Update User (Protected)
1. Client mengirim `PUT /api/users/{id}` dengan header `Authorization: Bearer <token>`.
2. Body divalidasi:
  - `name` minimal 3 karakter.
  - `email` harus valid.
  - `password` opsional, jika diisi minimal 6 karakter.
3. Handler `update` memeriksa apakah user ada.
4. Handler mengecek email unik (tidak boleh dipakai user lain).
5. Jika `password` diisi, password di-hash dengan bcrypt sebelum update.
6. Jika berhasil, kirim `200 OK` dengan message `User berhasil diperbarui`.
7. Jika user tidak ditemukan, kirim `404 Not Found`.
8. Jika email duplikat, kirim `409 Conflict`.

## Alur Hapus User (Protected)
1. Client mengirim `DELETE /api/users/{id}` dengan header `Authorization: Bearer <token>`.
2. Middleware `auth` memverifikasi token.
3. Handler `destroy` memeriksa apakah user ada.
4. Jika user ada, data dihapus dari database.
5. Jika berhasil, kirim `200 OK` dengan message `User berhasil dihapus` dan data `null`.
6. Jika user tidak ditemukan, kirim `404 Not Found`.

## Format Response Error Validasi
Ketika validasi gagal (status 422), response akan berisi detail error per field:
```json
{
  "status": false,
  "message": "Validasi Gagal",
  "data": {
    "name": ["Nama minimal 3 karakter"],
    "email": ["Email tidak valid"],
    "password": ["Password minimal 6 karakter"]
  }
}
```

## Format Response Error Umum
Untuk error lainnya (401, 409, 500), response menggunakan format:
```json
{
  "status": false,
  "message": "Pesan error",
  "data": null
}
```

## Catatan Environment
- `APP_PORT`: Port server (default: `3001`).
- `DATABASE_URL`: Connection string MySQL.
- `JWT_SECRET`: Secret key untuk sign/verify JWT. Jika tidak diset, default ke `"secret"`.
  - **PENTING**: Ganti dengan secret yang kuat pada production.

## File .gitignore
File yang diabaikan oleh git:
```
/target        # Binary dan build artifacts dari Cargo
.env           # Environment variables (berisi kredensial)
```

**Catatan Keamanan:**
- File `.env` tidak boleh di-commit ke repository karena berisi informasi sensitif.
- Gunakan `.env.example` sebagai template untuk sharing ke tim.
- Setiap developer harus membuat `.env` sendiri dengan kredensial mereka.

## Best Practices

### Keamanan
1. **JWT Secret**: Gunakan string random minimal 32 karakter untuk production.
   ```bash
   # Generate JWT secret dengan openssl
   openssl rand -hex 32
   ```

2. **Password**: Selalu hash password sebelum disimpan ke database (sudah diimplementasi dengan bcrypt cost 10).

3. **HTTPS**: Pada production, selalu gunakan HTTPS untuk melindungi token di transit.

4. **Token Expiration**: Token di-set 24 jam. Sesuaikan dengan kebutuhan:
   - Aplikasi sensitif: 15 menit - 1 jam
   - Aplikasi umum: 24 jam - 7 hari

### Development
1. **Environment Variables**: Jangan hardcode kredensial di source code.

2. **Database Migrations**: Selalu gunakan migration untuk perubahan schema database.
   ```bash
   # Buat migration baru
   sqlx migrate add nama_migration
   
   # Jalankan migration
   sqlx migrate run
   
   # Rollback migration (jika diperlukan)
   sqlx migrate revert
   ```

3. **Error Handling**: Jangan expose stack trace atau detail error database ke client pada production.

4. **Logging**: Untuk production, tambahkan logging middleware untuk monitoring.

### Code Organization
Struktur yang sudah diterapkan:
- `config/` - Konfigurasi aplikasi (database, dll)
- `handlers/` - Business logic untuk setiap endpoint
- `middlewares/` - Middleware seperti auth, logging, dll
- `models/` - Database models
- `routes/` - Route definitions
- `schemas/` - Request/Response validation schemas
- `utils/` - Helper functions (JWT, response format, dll)

Keuntungan struktur ini:
- **Separation of Concerns**: Setiap komponen punya tanggung jawab yang jelas.
- **Maintainability**: Mudah untuk menemukan dan mengubah kode.
- **Scalability**: Mudah untuk menambah fitur baru.
- **Testability**: Setiap komponen dapat di-test secara terpisah.

## Pengembangan Lebih Lanjut

Fitur yang bisa ditambahkan:
1. **Email Verification**: Verifikasi email saat registrasi.
2. **Password Reset**: Forgot password flow.
3. **Refresh Token**: Implementasi refresh token untuk keamanan lebih baik.
4. **Role-Based Access Control (RBAC)**: Admin, user, dll.
5. **Rate Limiting**: Batasi jumlah request per IP.
6. **Logging**: Structured logging dengan library seperti `tracing`.
7. **API Documentation**: Auto-generate dengan `utoipa` (OpenAPI/Swagger).
8. **Testing**: Unit tests dan integration tests.
9. **Docker**: Containerize aplikasi.
10. **CI/CD**: Setup GitHub Actions untuk automated testing dan deployment.
