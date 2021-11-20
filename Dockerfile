FROM rust as builder

RUN mkdir /workdir
WORKDIR /workdir

COPY Cargo.toml /workdir
COPY Cargo.lock /workdir
COPY src/dummy.rs /workdir/src/main.rs
RUN cargo build --release

COPY src /workdir/src

RUN cargo build --release

COPY /workdir/target/release/kudos /main

CMD ["/main"]