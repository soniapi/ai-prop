# syntax=docker/dockerfile:1

FROM rust:1.80-slim-bullseye AS builder

WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    libpq-dev \
    protobuf-compiler \
    git \
    && rm -rf /var/lib/apt/lists/*

# The project depends on `../ai-infra`, so we need to set that up
# in the Docker context. We clone the public repo here.
RUN git clone https://github.com/soniapi/ai-infra /ai-infra

# Copy the ai_prop source code
COPY . /app

# Build the project
RUN cargo build --release --bin server

FROM debian:bullseye-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder
COPY --from=builder /app/target/release/server /app/server

# Expose ports (Render will inject PORT env var to override REST port)
EXPOSE 3000
EXPOSE 50051

ENTRYPOINT ["/app/server"]
