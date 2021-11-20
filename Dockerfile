FROM rust:1.53-alpine3.13 as builder
RUN apk add --no-cache musl-dev

RUN mkdir /workdir
WORKDIR /workdir


COPY Cargo.toml .
COPY Cargo.lock .
COPY src/dummy.rs src/main.rs
RUN cargo build
RUN cargo build --release

COPY src src
RUN rm src/dummy.rs target/release/kudos
RUN cargo build --release


FROM alpine:3.13

COPY --from=builder /workdir/target/release/kudos /main

VOLUME /data
EXPOSE 3030
ENV FILENAME "/data/kudos_db.json"

CMD ["/main"]