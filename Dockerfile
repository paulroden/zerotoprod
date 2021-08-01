# Builder stage
## Use the latest Rust stable release as base image
FROM lukemathwalker/cargo-chef:latest-rust-1.53.0 AS planner
WORKDIR /app
COPY . .
# Generate a 'lock' file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM lukemathwalker/cargo-chef:latest-rust-1.53.0 AS cacher
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.53.0 AS builder
WORKDIR /app
# Cpopy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
COPY . .
ENV SQLX_OFFLINE true
# Build application, with deps already cached :)
RUN cargo build --release --bin zerotoprod

# Runtime stage
FROM debian:buster-slim AS runtime
WORKDIR /app
# Install OpenSSL - it is dynamically linked by some of our dependencies
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/zerotoprod zerotoprod
COPY configuration configuration
ENV APP_ENVIRONMENT production
# Launch binary on execute of `docker run`
ENTRYPOINT ["./zerotoprod"]
