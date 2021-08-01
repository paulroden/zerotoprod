# Builder stage
## Use the latest Rust stable release as base image
FROM rust:1.53.0 AS builder
WORKDIR /app
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

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
