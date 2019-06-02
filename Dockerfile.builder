ARG BASE_IMAGE=ekidd/rust-musl-builder
FROM ${BASE_IMAGE} AS builder
RUN rustup show
RUN rustup toolchain install stable-x86_64-unknown-linux-gnu
RUN rustup update && rustc -V &&cargo -V && rustup show
RUN rustup show

# sccache
RUN export RUSTC_WRAPPER=sccache
RUN cargo install sccache
RUN sccache --start-server
# 


RUN cargo install  --target=x86_64-unknown-linux-gnu diesel_cli --no-default-features --features postgres
ADD . ./
RUN sudo chown -R rust:rust /home/rust
RUN cargo build --target=x86_64-unknown-linux-musl --release

