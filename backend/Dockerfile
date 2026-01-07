# # ================================
# # Stage 1: Chef base
# # ================================
# FROM lukemathwalker/cargo-chef:latest-rust-alpine AS chef
# WORKDIR /app

# RUN apk add --no-cache \
#     musl-dev \
#     pkgconfig \
#     openssl-dev

# # ================================
# # Stage 2: Planner (dependency graph)
# # ================================
# FROM chef AS planner
# COPY Cargo.toml Cargo.lock ./
# COPY src ./src
# RUN cargo chef prepare --recipe-path recipe.json

# # ================================
# # Stage 3: Builder (cached deps + app)
# # ================================
# FROM chef AS builder
# COPY --from=planner /app/recipe.json recipe.json

# # Build dependencies only (cached forever unless deps change)
# RUN cargo chef cook \
#     --release \
#     --target x86_64-unknown-linux-musl \
#     --recipe-path recipe.json

# # Build the actual binary
# COPY Cargo.toml Cargo.lock ./
# COPY src ./src
# RUN cargo build \
#     --release \
#     --target x86_64-unknown-linux-musl \
#     --bin backend

# # Strip symbols aggressively
# RUN strip target/x86_64-unknown-linux-musl/release/backend

# # ================================
# # Stage 4: Runtime (scratch)
# # ================================
# FROM scratch

# WORKDIR /app
# COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/backend /backend

# EXPOSE 7070
# ENTRYPOINT ["/backend"]


#stage 1
FROM rust:1.91 as builder
WORKDIR /app
COPY . .
RUN cargo build --release


#stage 2
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/backend .
EXPOSE 7070
CMD ["./backend"]