# syntax=docker/dockerfile:1-labs
FROM rust:1.84.1 AS runtime
# install additional ca-certificates e.g. zscaler certificate (can be removed if not needed)
COPY ./configuration/trust/* /usr/local/share/ca-certificates/
RUN chmod 644 /usr/local/share/ca-certificates/* && update-ca-certificates

WORKDIR /app
RUN mkdir bin
COPY --exclude=configuration/bin/* . .

RUN cargo build --package nightfall_sync_test --release && \
    mv /app/target/release/nightfall_sync_test /app/bin/sync_test

CMD ["/app/bin/sync_test"]