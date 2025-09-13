# syntax=docker/dockerfile:1-labs
FROM ghcr.io/foundry-rs/foundry:v1.1.0 AS foundry
FROM rust:1.88.0 AS runtime
# install additional ca-certificates e.g. zscaler certificate (can be removed if not needed)
COPY ./configuration/trust/* /usr/local/share/ca-certificates/
RUN chmod 644 /usr/local/share/ca-certificates/* && update-ca-certificates

WORKDIR /app
RUN mkdir bin 
COPY --from=foundry /usr/local/bin/forge ./forge
COPY --exclude=configuration/bin/* . .
ENV PATH=/app:$PATH
WORKDIR /app/nightfall_test
RUN --mount=type=cache,id=cargo-registry-v4,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,id=cargo-git-v4,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,id=cargo-target-client-v4,target=/app/target,sharing=private \
    cargo build --package nightfall_test --release && \
    mv /app/target/release/nightfall_test /app/bin/test
CMD ["/app/bin/test"]