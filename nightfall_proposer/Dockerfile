FROM rust:1.88.0 AS builder
WORKDIR /app
RUN mkdir bin
COPY . .
RUN --mount=type=cache,id=cargo-registry-v4,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,id=cargo-git-v4,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,id=cargo-target-client-v4,target=/app/target,sharing=private \
    cargo build --package nightfall_proposer --release && \
    mv /app/target/release/nightfall_proposer /app/bin/proposer
FROM debian:bookworm-slim
WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl3 \
    wget \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# install additional ca-certificates e.g. zscaler certificate (can be removed if not needed)
COPY configuration/trust/* /usr/local/share/ca-certificates/
RUN chmod 644 /usr/local/share/ca-certificates/* && update-ca-certificates
# Copy application binaries and configuration
COPY --from=builder /app/bin/proposer bin/
COPY configuration/bin/* configuration/bin/
RUN mkdir -p configuration/bin/keys
COPY configuration/bin/keys/* configuration/bin/keys
COPY .env *.env nightfall.toml .

EXPOSE 3000
CMD ["/app/bin/proposer"]