FROM alpine:3.18

RUN apk add --no-cache rustup musl-dev gcc curl

# Install rust
ENV RUSTUP_HOME /opt/rustup_home
ENV CARGO_HOME /opt/cargo_home
RUN rustup-init -y --default-toolchain nightly --target wasm32-unknown-unknown
ENV PATH /opt/cargo_home/bin:$PATH

# Install tools
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | sh
RUN cargo binstall --no-confirm trunk