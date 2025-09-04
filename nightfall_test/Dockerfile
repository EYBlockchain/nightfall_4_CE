# syntax=docker/dockerfile:1-labs
FROM ghcr.io/foundry-rs/foundry:v0.3.0 AS foundry
FROM rust:1.84.1 AS runtime
# install additional ca-certificates e.g. zscaler certificate (can be removed if not needed)
COPY ./configuration/trust/* /usr/local/share/ca-certificates/
RUN chmod 644 /usr/local/share/ca-certificates/* && update-ca-certificates

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    rm -rf /usr/local/cargo/registry/src/index.crates.io-*/http-types-2.12.0 || true

WORKDIR /app
RUN mkdir bin 
COPY --from=foundry /usr/local/bin/forge ./forge
COPY --exclude=configuration/bin/* . .
ENV PATH=/app:$PATH
WORKDIR /app/nightfall_test
# RUN --mount=type=cache,target=/usr/local/cargo/registry \
#     --mount=type=cache,target=/app/target \
#     --mount=type=cache,target=/usr/local/cargo/git \
#     cargo build --package nightfall_test --release && \
RUN --mount=type=cache,id=cargo-registry-v4,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,id=cargo-git-v4,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,id=cargo-target-client-v4,target=/app/target,sharing=private \
    cargo build --package nightfall_test --release && \
    mv /app/target/release/nightfall_test /app/bin/test
CMD ["/app/bin/test"]