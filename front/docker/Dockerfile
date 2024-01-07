FROM ubuntu:22.04

RUN apt-get update && apt-get install -y pkg-config curl build-essential libssl-dev

# Install rust
ENV RUSTUP_HOME /opt/rustup_home
ENV CARGO_HOME /opt/cargo_home
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y --default-toolchain nightly --target wasm32-unknown-unknown
ENV PATH /opt/cargo_home/bin:$PATH

# Install tools
RUN cargo install trunk --version 0.18.3