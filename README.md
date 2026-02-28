# Backend API JWT (Rust + Axum)

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![MySQL](https://img.shields.io/badge/mysql-%2300f.svg?style=for-the-badge&logo=mysql&logoColor=white)

Backend API dengan autentikasi JWT menggunakan Rust, Axum web framework, dan MySQL database.

## Daftar Isi
- [Tujuan](#tujuan)
- [Fitur](#fitur)
- [Prasyarat](#prasyarat)
- [Quick Start](#quick-start)
- [Struktur Proyek](#struktur-proyek)
- [Konfigurasi Environment](#konfigurasi-environment)
- [Setup Database](#setup-database)
- [Menjalankan Aplikasi](#menjalankan-aplikasi)
- [Dependensi Utama](#dependensi-utama)
- [Catatan Endpoint](#catatan-endpoint)
- [Docker](#docker)
- [Troubleshooting](#troubleshooting)
- [Lisensi](#lisensi)
- [Kontribusi](#kontribusi)

## Tujuan
- Menyediakan backend API dengan autentikasi JWT.
- Terhubung ke MySQL via `sqlx`.
- Siap untuk validasi request, hashing password, dan CORS.

## Fitur
✅ Autentikasi JWT (JSON Web Token)  
✅ Password hashing dengan bcrypt  
✅ Validasi request dengan validator  
✅ Database MySQL dengan SQLx (compile-time checked)  
✅ Protected routes dengan middleware  
✅ Error handling yang comprehensive  
✅ Response format API yang konsisten  
✅ Environment configuration dengan dotenv  
✅ Migration system untuk database  
✅ CORS support (ready untuk frontend)  

## Prasyarat
- Rust toolchain (stable) - [Install Rust](https://rustup.rs/)
- Cargo (sudah termasuk dengan Rust)
- MySQL Server 5.7+ atau 8.0+
- SQLx CLI untuk menjalankan migrasi database

## Quick Start

1. **Clone repository**
   ```bash
   git clone https://github.com/rifainareswara/rust-backend-api-jwt.git
   cd rust-backend-api-jwt
   ```

2. **Setup environment**
   ```bash
   cp .env.example .env
   # Edit .env dan sesuaikan dengan konfigurasi Anda
   ```

3. **Install SQLx CLI**
   ```bash
   cargo install sqlx-cli --no-default-features --features mysql
   ```

4. **Setup database**
   ```bash
   # Buat database (pastikan MySQL sudah running)
   sqlx database create
   
   # Jalankan migrasi
   sqlx migrate run
   ```

5. **Jalankan aplikasi**
   ```bash
   cargo run
   ```

6. **Test endpoint**
   ```bash
   # Register user baru
   curl -X POST http://127.0.0.1:3001/api/register \
     -H "Content-Type: application/json" \
     -d '{"name":"Test User","email":"test@example.com","password":"password123"}'
   ```

## Struktur Proyek
```
backend-api-jwt/
	Cargo.toml
	README.md
	.env.example
  Dockerfile
  docker-compose.yml
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
Salin file `.env.example` menjadi `.env` di root proyek:

```bash
cp .env.example .env
```

Kemudian sesuaikan nilai-nilai di file `.env`:

```
APP_PORT=3001
DATABASE_URL=mysql://username:password@localhost:3306/db_backend_api_jwt
JWT_SECRET=your_jwt_secret_key_here_change_this_in_production
```

**Catatan:**
- Ganti `username` dan `password` dengan kredensial MySQL Anda.
- Ganti `JWT_SECRET` dengan secret key yang kuat untuk production.

## Setup Database

### 1. Install SQLx CLI
```bash
cargo install sqlx-cli --no-default-features --features mysql
```

### 2. Buat Database
```bash
# Buat database MySQL
mysql -u root -p -e "CREATE DATABASE db_backend_api_jwt;"
```

Atau gunakan SQLx CLI:
```bash
sqlx database create
```

### 3. Jalankan Migrasi
```bash
sqlx migrate run
```

Perintah ini akan membuat tabel `users` dengan struktur:
- `id` (BIGINT, auto increment, primary key)
- `name` (VARCHAR(100))
- `email` (VARCHAR(100), unique)
- `password` (TEXT)
- `created_at` (TIMESTAMP, default current timestamp)
- `updated_at` (TIMESTAMP, auto update)

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

## Konfigurasi CORS
Saat ini aplikasi menggunakan `CorsLayer` dari `tower-http` dengan konfigurasi default berikut:
- Mengizinkan semua origin (`allow_origin(Any)`)
- Mengizinkan semua HTTP method (`allow_methods(Any)`)
- Mengizinkan semua header (`allow_headers(Any)`)

Konfigurasi ini cocok untuk development. Untuk production, sebaiknya batasi origin, method, dan header sesuai kebutuhan frontend Anda.

## Catatan Endpoint
Endpoint yang sudah tersedia:

### Public Endpoints (Tidak perlu token)
- `POST /api/register` - Registrasi user baru
- `POST /api/login` - Login dan dapatkan token

### Protected Endpoints (Butuh header `Authorization: Bearer <token>`)
- `GET /api/users` - Mengambil list semua user
- `POST /api/users` - Menambahkan user baru (untuk admin/user yang sudah login)
- `GET /api/users/{id}` - Mengambil detail user berdasarkan ID
- `PUT /api/users/{id}` - Memperbarui data user berdasarkan ID
- `DELETE /api/users/{id}` - Menghapus user berdasarkan ID

Middleware auth sudah dipasang pada semua route user.

## Docker

Project ini sudah memiliki file container di root:
- `Dockerfile`
- `docker-compose.yml`

### Cara menjalankan dengan Docker Compose
```bash
docker compose up --build
```

### Status konfigurasi Docker saat ini
- Service `app` di `docker-compose.yml` publish port `8080:8080`.
- Service database di `docker-compose.yml` menggunakan **PostgreSQL** (`postgres:13`).
- `Dockerfile` saat ini juga menginstal dependency PostgreSQL (`libpq-dev`) dan expose port `8080`.

### Catatan penting sinkronisasi
Aplikasi Rust saat ini (kode runtime) menggunakan:
- `DATABASE_URL` untuk **MySQL** (via `sqlx` feature `mysql`)
- Port default `APP_PORT=3001`

Artinya konfigurasi Docker saat ini belum sepenuhnya sinkron dengan konfigurasi runtime aplikasi. Jika akan dipakai untuk development aktif, sesuaikan `Dockerfile`/`docker-compose.yml` agar menggunakan MySQL dan port aplikasi yang sama.

### Contoh Penggunaan

**Register (Public)**
```bash
curl -X POST http://127.0.0.1:3001/api/register \
  -H "Content-Type: application/json" \
  -d '{"name":"John Doe","email":"john@example.com","password":"password123"}'
```

Response:
```json
{
  "status": true,
  "message": "Register Berhasil",
  "data": {
    "id": 1,
    "name": "John Doe",
    "email": "john@example.com",
    "created_at": "2026-02-21T10:00:00Z",
    "updated_at": "2026-02-21T10:00:00Z"
  }
}
```

**Login (Public)**
```bash
curl -X POST http://127.0.0.1:3001/api/login \
  -H "Content-Type: application/json" \
  -d '{"email":"john@example.com","password":"password123"}'
```

Response:
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
    "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
  }
}
```

**List Users (Protected)**
```bash
curl -H "Authorization: Bearer <token>" http://127.0.0.1:3001/api/users
```

Response:
```json
{
  "status": true,
  "message": "List user",
  "data": [
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

**Create User (Protected)**
```bash
curl -X POST http://127.0.0.1:3001/api/users \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{"name":"Jane Doe","email":"jane@example.com","password":"password123"}'
```

Response:
```json
{
  "status": true,
  "message": "User berhasil ditambahkan",
  "data": {
    "id": 2,
    "name": "Jane Doe",
    "email": "jane@example.com",
    "created_at": "2026-02-21T11:00:00Z",
    "updated_at": "2026-02-21T11:00:00Z"
  }
}
```

**Detail User (Protected)**
```bash
curl -H "Authorization: Bearer <token>" http://127.0.0.1:3001/api/users/2
```

Response:
```json
{
  "status": true,
  "message": "Detail user",
  "data": {
    "id": 2,
    "name": "Jane Doe",
    "email": "jane@example.com",
    "created_at": "2026-02-21T11:00:00Z",
    "updated_at": "2026-02-21T11:00:00Z"
  }
}
```

**Update User (Protected)**
```bash
curl -X PUT http://127.0.0.1:3001/api/users/2 \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{"name":"Jane Updated","email":"jane.updated@example.com","password":"newpassword123"}'
```

Response:
```json
{
  "status": true,
  "message": "User berhasil diperbarui",
  "data": {
    "id": 2,
    "name": "Jane Updated",
    "email": "jane.updated@example.com",
    "created_at": "2026-02-21T11:00:00Z",
    "updated_at": "2026-02-28T09:30:00Z"
  }
}
```

**Delete User (Protected)**
```bash
curl -X DELETE http://127.0.0.1:3001/api/users/2 \
  -H "Authorization: Bearer <token>"
```

Response:
```json
{
  "status": true,
  "message": "User berhasil dihapus",
  "data": null
}
```

## Troubleshooting

### Error: "Database connection failed"
**Penyebab:** MySQL server tidak running atau kredensial di `.env` salah.

**Solusi:**
```bash
# Cek status MySQL
# macOS (Homebrew)
brew services list | grep mysql

# Atau restart MySQL
brew services restart mysql

# Linux (systemd)
sudo systemctl status mysql
sudo systemctl start mysql
```

Pastikan `DATABASE_URL` di `.env` sesuai dengan kredensial MySQL Anda.

### Error: "sqlx migrate run" gagal
**Penyebab:** Database belum dibuat atau SQLx CLI belum terinstall.

**Solusi:**
```bash
# Install SQLx CLI jika belum
cargo install sqlx-cli --no-default-features --features mysql

# Buat database terlebih dahulu
sqlx database create

# Kemudian jalankan migrasi
sqlx migrate run
```

### Error: "Port already in use"
**Penyebab:** Port 3001 sudah digunakan oleh aplikasi lain.

**Solusi:**
Edit file `.env` dan ubah `APP_PORT` ke port lain:
```
APP_PORT=3002
```

### Error: Compile error terkait sqlx
**Penyebab:** SQLx memerlukan database untuk compile-time verification.

**Solusi:**
```bash
# Jalankan database terlebih dahulu
# Kemudian compile ulang
cargo clean
cargo build
```

Atau disable compile-time checking dengan menambahkan di `Cargo.toml`:
```toml
[dependencies]
sqlx = { version = "0.8.6", features = ["mysql", "runtime-tokio", "macros", "chrono"], default-features = false }
```

### Token tidak valid / expired
**Penyebab:** Token JWT sudah expired (lebih dari 24 jam) atau JWT_SECRET berbeda.

**Solusi:**
- Login ulang untuk mendapatkan token baru
- Pastikan `JWT_SECRET` di `.env` konsisten dan tidak berubah

## Lisensi

MIT License - silakan gunakan untuk keperluan belajar dan development.

## Kontribusi

Pull requests are welcome! Untuk perubahan besar, silakan buka issue terlebih dahulu untuk diskusi.
