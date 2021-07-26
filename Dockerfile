# Use the latest Rust stable release as base image
FROM rust:1.53.0

# Switch working directory to `app` on route of Docker image instance
WORKDIR /app
# Copy all files from working environment to Docker image
COPY . .
# Set offline mode for sqlx
ENV SQLX_OFFLINE true
# Build release binaries
RUN cargo build --release
# Docker instance is as `production`
ENV APP_ENVIRONMENT production
# Launch binary on execute of `docker run`
ENTRYPOINT ["./target/release/zerotoprod"]
