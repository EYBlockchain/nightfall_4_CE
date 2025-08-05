# syntax=docker/dockerfile:1-labs
FROM ghcr.io/foundry-rs/foundry:v1.1.0 AS foundry
FROM rust:1.88.0 AS builder

# install additional ca-certificates e.g. zscaler certificate (can be removed if not needed)
COPY ./configuration/trust/* /usr/local/share/ca-certificates/
RUN chmod 644 /usr/local/share/ca-certificates/* && update-ca-certificates

WORKDIR /app
RUN mkdir -p bin
COPY --from=foundry /usr/local/bin/forge ./forge
COPY --exclude=configuration/bin/*  . .
COPY configuration/bin/*_vk configuration/bin/
ENV PATH=/app:$PATH
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build --package nightfall_deployer --release && \
    mv /app/target/release/nightfall_deployer /app/bin/deployer

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/bin/deployer bin/
COPY configuration/bin/*_vk configuration/bin/
# COPY configuration/toml/addresses.toml configuration/toml/
COPY blockchain_assets blockchain_assets/
COPY .env *.env nightfall.toml foundry.toml ./
RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl3
COPY --from=foundry /usr/local/bin/forge ./forge    
ENV PATH=/app:$PATH
RUN mkdir -p nightfall_deployer/blockchain_assets
RUN ln -s /app/blockchain_assets /app/nightfall_deployer/blockchain_assets

CMD ["/app/bin/deployer"]
