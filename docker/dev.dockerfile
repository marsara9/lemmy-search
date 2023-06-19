FROM rust:slim-bookworm AS build

WORKDIR /build
COPY server/ server/

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        pkg-config libssl-dev libpq-dev

RUN cargo build --manifest-path=server/Cargo.toml --verbose

FROM ubuntu:latest

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        libpq-dev

WORKDIR /lemmy
COPY ui/ ui/
COPY config/ config/
COPY --from=build /build/server/target/debug/lemmy-search bin/lemmy-search

EXPOSE 8000
ENTRYPOINT [ "/lemmy/bin/lemmy-search", "/lemmy/ui" ]
