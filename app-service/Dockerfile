# Start with image that has the Rust toolchain installed
FROM rust:1.92 AS chef
USER root
# Add cargo-chef to cache dependencies

# RUN apk add --no-cache musl-dev libressl-dev curl bash && \
#     cargo install cargo-chef
    
# RUN apt-get update && \
RUN apt-get update && \
    apt-get install build-essential libssl-dev pkg-config curl bash -y && \
    rm -rf /var/lib/apt/lists/* && \
    cargo install cargo-chef
    
WORKDIR /app

FROM chef AS planner

COPY . .

# Capture info needed to build dependencies

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .

# Install `dx`
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli --root /.cargo -y --force
ENV PATH="/.cargo/bin:$PATH"

# RUN curl -sSL https://raw.githubusercontent.com/DioxusLabs/dioxus/refs/heads/main/.github/install.sh | bash

# RUN cargo build --release --bin app-service
RUN dx bundle --web --release

# We do not need the Rust toolchain to run the binary!
# Start with a minimal image and copy over the binary and assets folder.
FROM debian:latest AS runtime
WORKDIR /app
COPY --from=builder /app/default.toml /app/default.toml
COPY --from=builder /app/target/release/app-service /usr/local/bin
COPY --from=builder /app/target/dx/app-service/release/web/ /app
COPY --from=builder /app/assets /app/assets

ENV PORT=8080
ENV IP=0.0.0.0

EXPOSE 8080

ENV AUTH_SERVICE_HOST_NAME=auth-service
ENTRYPOINT ["/usr/local/bin/app-service"]
