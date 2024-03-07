FROM rust:bookworm as builder

WORKDIR /app

# Copy & Build
COPY . .
RUN cargo build

# Extract Binary
FROM docker.io/debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates tini libpq-dev libcurl4

# Copy the binary from the builder stage
COPY --from=builder /app/target/debug/test-area /usr/local/bin/app

# Set the binary as the container's entrypoint
ENTRYPOINT ["app"]
