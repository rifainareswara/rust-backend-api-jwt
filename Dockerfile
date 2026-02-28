
# Use a Rust base image
FROM rust:latest as builder

# Set the working directory
WORKDIR /app

# Copy Cargo.toml and Cargo.lock to leverage Docker cache
COPY Cargo.toml Cargo.lock ./

# Install any dependencies
RUN apt-get update && apt-get install -y libpq-dev

# Build dummy project to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release
RUN rm -rf src/main.rs target/release/deps/backend-api-jwt*

# Copy the source code
COPY src ./src

# Build the application
RUN cargo build --release

# --- Final image ---
FROM debian:bookworm-slim

# Install PostgreSQL client libraries
RUN apt-get update && apt-get install -y libpq-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/backend-api-jwt .

# Expose the port your application listens on (e.g., 8080)
EXPOSE 8080

# Run the application
CMD ["./backend-api-jwt"]
