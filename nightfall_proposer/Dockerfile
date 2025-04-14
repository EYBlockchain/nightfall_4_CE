FROM rust:1.84.1 AS builder
# install additional ca-certificates e.g. zscaler certificate (can be removed if not needed)
COPY configuration/trust/* /usr/local/share/ca-certificates/
RUN chmod 644 /usr/local/share/ca-certificates/* && update-ca-certificates
WORKDIR /app
RUN mkdir bin
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/home/root/app/target \
    cargo build --package nightfall_proposer --release && \
    mv /app/target/release/nightfall_proposer /app/bin/proposer
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/bin/proposer bin/
COPY configuration/bin/* configuration/bin/
COPY configuration/toml/addresses.toml configuration/toml/
COPY .env *.env nightfall.toml .
RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl3 wget
EXPOSE 3000
CMD ["/app/bin/proposer"]