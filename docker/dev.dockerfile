FROM rust:slim-bookworm AS build

WORKDIR /build
COPY server/ server/

RUN cargo build

FROM ubuntu:latest

WORKDIR /lemmy
COPY --from=build /build/server/target/release/lemmy-search bin/lemmy-search
COPY ui/ ui/


EXPOSE 80
ENTRYPOINT [ "/bin/lemmy-search" ]
