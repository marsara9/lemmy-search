FROM lukemathwalker/cargo-chef:latest-rust-slim AS chef
WORKDIR /cache

FROM chef as planner

COPY server/ ./
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as cacher

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        pkg-config libssl-dev

COPY --from=planner /cache/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:slim-bookworm AS build

WORKDIR /build
COPY server/ server/
COPY --from=cacher /cache/target/ server/target/

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        pkg-config libssl-dev

RUN cargo build --manifest-path=server/Cargo.toml --verbose --release

FROM ubuntu:latest

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates

WORKDIR /lemmy
COPY ui/ ui/
COPY config/ config/
COPY --from=build /build/server/target/release/lemmy-search bin/lemmy-search

EXPOSE 8000
ENTRYPOINT [ "/lemmy/bin/lemmy-search", "/lemmy/ui" ]
