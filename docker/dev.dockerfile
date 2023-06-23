FROM lukemathwalker/cargo-chef:latest-rust-slim AS chef
WORKDIR /cache

FROM chef as planner

# We only pay the installation cost once, 
# it will be cached from the second build onwards
#RUN cargo install cargo-chef 
COPY server/ ./
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as cacher

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        pkg-config libssl-dev

#RUN cargo install cargo-chef
COPY --from=planner /cache/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json

FROM rust:slim-bookworm AS build

WORKDIR /build
COPY server/ server/
COPY --from=cacher /cache/target/ server/target/

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        pkg-config libssl-dev

RUN cargo build --manifest-path=server/Cargo.toml --verbose

FROM ubuntu:latest

WORKDIR /lemmy
COPY ui/ ui/
COPY config/ config/
COPY --from=build /build/server/target/debug/lemmy-search bin/lemmy-search

EXPOSE 8000
ENTRYPOINT [ "/lemmy/bin/lemmy-search", "/lemmy/ui" ]
