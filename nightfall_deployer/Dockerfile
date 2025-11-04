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
RUN --mount=type=cache,id=cargo-registry-v4,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,id=cargo-git-v4,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,id=cargo-target-client-v4,target=/app/target,sharing=private \
    cargo build --package nightfall_deployer --release && \
    mv /app/target/release/nightfall_deployer /app/bin/deployer

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/bin/deployer bin/
COPY configuration/bin/*_vk configuration/bin/
COPY blockchain_assets blockchain_assets/
COPY .env *.env nightfall.toml foundry.toml ./


# RUN apt-get update && apt-get install -y --no-install-recommends \
    # libssl3

# Install libssl3 + Node.js (includes npm/npx via NodeSource)
RUN apt-get update && apt-get install -y --no-install-recommends \
      curl ca-certificates gnupg libssl3 \
 && curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
 && apt-get install -y --no-install-recommends nodejs \
 && npm i -g @openzeppelin/upgrades-core@^1.37.0 \
 && npm cache clean --force && rm -rf /var/lib/apt/lists/*


COPY --from=foundry /usr/local/bin/forge ./forge    
ENV PATH=/app:$PATH
RUN mkdir -p nightfall_deployer/blockchain_assets
RUN ln -s /app/blockchain_assets /app/nightfall_deployer/blockchain_assets

CMD ["/app/bin/deployer"]
