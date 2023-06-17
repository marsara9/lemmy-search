FROM rust:slim-bookworm AS build

WORKDIR /build
COPY Cargo.toml .
COPY crates/ crates/

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        pkg-config libssl-dev

RUN cargo build

FROM ubuntu:latest

WORKDIR /lemmy
COPY ui/ ui/
COPY --from=build /build/target/debug/liblemmy_search_common.rlib bin/liblemmy_search_common.rlib
COPY --from=build /build/target/debug/lemmy-search-crawler bin/lemmy-search-crawler
COPY --from=build /build/target/debug/lemmy-search bin/lemmy-search

EXPOSE 8000
ENTRYPOINT [ "/lemmy/bin/lemmy-search", "/lemmy/ui" ]
