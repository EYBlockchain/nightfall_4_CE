FROM ghcr.io/foundry-rs/foundry:v0.3.0 AS anvil
FROM alpine:3.18
ENV ANVIL_IP_ADDR=0.0.0.0
COPY --from=anvil /usr/local/bin/anvil /usr/local/bin/anvil
RUN apk update && apk add curl
# install additional ca-certificates e.g. zscaler certificate (can be removed if not needed)
COPY ./configuration/trust/* /usr/local/share/ca-certificates/
RUN chmod 644 /usr/local/share/ca-certificates/* && update-ca-certificates
# Although a mining interval (block time) is set here, the tests will override it.
CMD ["anvil", "--base-fee", "58000000000", "--block-time", "5"]