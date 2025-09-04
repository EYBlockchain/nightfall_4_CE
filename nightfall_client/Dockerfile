# syntax=docker/dockerfile:1-labs
FROM rust:1.84.1 AS builder
# install additional ca-certificates e.g. zscaler certificate (can be removed if not needed)
COPY configuration/trust/* /usr/local/share/ca-certificates/
RUN chmod 644 /usr/local/share/ca-certificates/* && update-ca-certificates

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    rm -rf /usr/local/cargo/registry/src/index.crates.io-*/http-types-2.12.0 || true

WORKDIR /app
RUN mkdir bin
COPY --exclude=configuration/bin/*  . .
COPY configuration/bin/proving_key configuration/bin/deposit_proving_key configuration/bin/
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build --package nightfall_client --release && \
    mv /app/target/release/nightfall_client /app/bin/client
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/bin/client bin/
COPY configuration/bin/proving_key configuration/bin/deposit_proving_key configuration/bin/
# COPY configuration/toml/addresses.toml configuration/toml/
COPY .env *.env nightfall.toml .
RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl3 wget
EXPOSE 3000
CMD ["/app/bin/client"]