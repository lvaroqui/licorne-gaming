FROM ubuntu:22.04

RUN apt-get update
RUN apt-get install -y curl build-essential libpq-dev bash

# Install rust
ENV RUSTUP_HOME /opt/rustup_home
ENV CARGO_HOME /opt/cargo_home
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH /opt/cargo_home/bin:$PATH

# Install tools
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch
