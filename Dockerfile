# Use the latest Rust stable release as base image
FROM rust:1.53.0

# Switch working directory to `app` on route of Docker image instance
WORKDIR /app
# Copy all files from working environment to Docker image
COPY . .
# Build release binaries
RUN cargo build --release
# Launch binary on execute of `docker run`
ENTRYPOINT ["./target/release/zerotoprod"]
