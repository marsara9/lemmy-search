FROM rust:slim-bookworm AS build

WORKDIR /build
COPY server/ server/

RUN cargo build --manifest-path=server/Cargo.toml

FROM ubuntu:latest

WORKDIR /lemmy
COPY ui/ ui/
COPY --from=build /build/server/target/debug/lemmy-search bin/lemmy-search


EXPOSE 80
ENTRYPOINT [ "/bin/lemmy-search" ]
