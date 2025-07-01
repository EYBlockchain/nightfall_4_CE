FROM ghcr.io/foundry-rs/foundry:v0.3.0 AS anvil
FROM alpine:3.18
ENV ANVIL_IP_ADDR=0.0.0.0
COPY --from=anvil /usr/local/bin/anvil /usr/local/bin/anvil
COPY --from=anvil /usr/local/bin/forge /usr/local/bin/forge

RUN apk update && apk add curl
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

