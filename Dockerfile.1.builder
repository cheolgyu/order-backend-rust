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


WORKDIR /usr/src/myapp
COPY . .

RUN rustup target add x86_64-unknown-linux-musl
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV OPENSSL_DIR=/usr/include/openssl
RUN cargo build --release
#RUN cargo build --target x86_64-unknown-linux-musl --release
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# docker push 410450153592.dkr.ecr.ap-northeast-2.amazonaws.com/builder:latest