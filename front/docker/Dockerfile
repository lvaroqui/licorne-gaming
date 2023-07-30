FROM alpine:3.18

RUN apk add --no-cache rustup gcc

# Install rust
ENV RUSTUP_HOME /opt/rustup
ENV CARGO_HOME /opt/cargo
RUN rustup-init -y --default-toolchain nightly --target wasm32-unknown-unknown
ENV PATH /opt/cargo/bin:$PATH

# Install tools
RUN apk add --no-cache rustup musl-dev
RUN cargo install --locked trunk