# Change from 1.83 to latest
FROM rust:latest as builder

WORKDIR /app
RUN apt-get update && apt-get install -y libsqlite3-dev pkg-config

# 1. Create a dummy project to cache dependencies
RUN cargo new --bin dummy_project
WORKDIR /app/dummy_project
COPY Cargo.toml Cargo.lock* ./

# 2. Build only the dependencies
RUN cargo build --release
RUN rm src/*.rs

# 3. Copy your real source code
COPY src ./src
RUN touch src/main.rs && cargo build --release

# Stage 2: Run (remains the same)
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y libsqlite3-0 ca-certificates && rm -rf /var/lib/apt/lists/*

# IMPORTANT: Ensure 'your_project_name' matches the [package] name in Cargo.toml
COPY --from=builder /app/dummy_project/target/release/backend .

EXPOSE 7070
CMD ["./backend"]