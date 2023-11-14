FROM rust:1-slim-bookworm as builder

ENV SQLX_OFFLINE=true
WORKDIR /usr/src/app
COPY . .
# Will build and cache the binary and dependent crates in release mode
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release && mv ./target/release/flyio-rust ./flyio-rust

# Runtime image
FROM debian:bookworm-slim

# Run as "app" user
RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/src/app/flyio-rust /app/flyio-rust
COPY static /app/static

# Run the app
CMD ./flyio-rust
