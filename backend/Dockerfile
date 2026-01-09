# Stage 1: Build the Rust app
FROM rust:latest AS builder

# Set working directory
WORKDIR /usr/src/app

# Docker caching
COPY Cargo.toml Cargo.lock ./

# dummy src for cargo fetch caching
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Fetch dependencies
RUN cargo fetch

# Remove dummy and copy actual source code
RUN rm -rf src
COPY src ./src

# Build in release mode
RUN cargo build --release

# Stage 2: Minimal runtime using distroless
FROM gcr.io/distroless/cc-debian12

# Set working directory for persistent data
WORKDIR /data

# Copy the built binary
COPY --from=builder /usr/src/app/target/release/backend /usr/local/bin/backend 

# Expose the app port
EXPOSE 7070

# Run the binary
CMD ["backend"]