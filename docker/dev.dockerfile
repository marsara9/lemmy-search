FROM rust:slim-bookworm AS build

WORKDIR /build
COPY crawler/ crawler/
COPY server/ server/

RUN cargo build --manifest-path=crawler/Cargo.toml
RUN cargo build --manifest-path=server/Cargo.toml

FROM ubuntu:latest

WORKDIR /lemmy
COPY ui/ ui/
COPY --from=build /build/crawler/target/debug/lemmy-crawler bin/lemmy-crawler
COPY --from=build /build/server/target/debug/lemmy-search bin/lemmy-search

EXPOSE 80
ENTRYPOINT [ "/lemmy/bin/lemmy-search" ]
