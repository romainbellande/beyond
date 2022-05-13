FROM rust:1-buster

ARG UID

RUN adduser --home /home/rust --uid $UID rust

USER rust

WORKDIR /home/rust

RUN rustup default nightly && rustup update

RUN cargo install cargo-watch

WORKDIR /home/rust/server
