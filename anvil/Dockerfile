FROM ghcr.io/foundry-rs/foundry:v1.1.0 AS anvil
FROM debian:bullseye-slim
ENV ANVIL_IP_ADDR=0.0.0.0
COPY --from=anvil /usr/local/bin/anvil /usr/local/bin/anvil
COPY --from=anvil /usr/local/bin/forge /usr/local/bin/forge

RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
# install additional ca-certificates e.g. zscaler certificate (can be removed if not needed)
COPY ./configuration/trust/* /usr/local/share/ca-certificates/
RUN chmod 644 /usr/local/share/ca-certificates/* && update-ca-certificates
# Although a mining interval (block time) is set here, the tests will override it.

# Copy entrypoint script
COPY ./anvil/entrypoint.sh entrypoint.sh
RUN chmod +x entrypoint.sh

COPY blockchain_assets blockchain_assets/
COPY foundry.toml ./
COPY .env ./

# Set entrypoint
ENTRYPOINT ["./entrypoint.sh"]

