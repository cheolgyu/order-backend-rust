FROM ubuntu:18.04

RUN apt-get update -y && \
    apt-get install -y \
    musl-tools \
    libssl-dev \
    pkg-config \
    curl \
    libpq-dev

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH=/root/.cargo/bin:$PATH

# sccache
RUN export RUSTC_WRAPPER=sccache
RUN cargo install sccache
RUN sccache --start-server
#
RUN cargo install diesel_cli --no-default-features --features postgres

ENV PKG_CONFIG_ALLOW_CROSS=1
ENV OPENSSL_DIR=/usr/include/openssl

WORKDIR /usr/src/myapp
COPY Cargo.toml Cargo.toml

RUN rustup target add x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/order-backend-rust*
COPY . .
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl
# $(aws ecr get-login --no-include-email --region ap-northeast-2)
# docker tag base:latest 410450153592.dkr.ecr.ap-northeast-2.amazonaws.com/base:latest
# docker push 410450153592.dkr.ecr.ap-northeast-2.amazonaws.com/base:latest

