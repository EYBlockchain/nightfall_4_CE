# syntax=docker/dockerfile:1-labs
FROM ghcr.io/foundry-rs/foundry:v0.3.0 AS foundry
FROM rust:1.84.1 AS runtime
# install additional ca-certificates e.g. zscaler certificate (can be removed if not needed)
COPY ./configuration/trust/* /usr/local/share/ca-certificates/
RUN chmod 644 /usr/local/share/ca-certificates/* && update-ca-certificates

WORKDIR /app
RUN mkdir bin 
COPY --from=foundry /usr/local/bin/forge ./forge
COPY --exclude=configuration/bin/* . .
ENV PATH=/app:$PATH
WORKDIR /app/nightfall_test
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/home/root/app/target \
    cargo build --package nightfall_test --release && \
    mv /app/target/release/nightfall_test /app/bin/test
CMD ["/app/bin/test"]