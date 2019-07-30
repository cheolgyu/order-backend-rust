#ARG BASE_IMAGE=410450153592.dkr.ecr.ap-northeast-2.amazonaws.com/builder:latest
ARG BASE_IMAGE=410450153592.dkr.ecr.ap-northeast-2.amazonaws.com/base:latest
FROM ${BASE_IMAGE} AS builder
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /usr/src/myapp
ADD . ./
ENV PKG_CONFIG_ALLOW_CROSS=1
RUN apt-get update && apt-get install -y  musl-tools
RUN ldd /usr/src/myapp/target/x86_64-unknown-linux-musl/release/order-backend-rust
# RUN cargo build --target x86_64-unknown-linux-musl --release
RUN cargo build --target x86_64-unknown-linux-musl
RUN ls -lh /usr/src/myapp/target/x86_64-unknown-linux-musl/release/order-backend-rust

RUN mkdir -p /build-out
WORKDIR  /usr/src/myapp/target/release/
RUN ls 
RUN cp order-backend-rust /build-out/
RUN cp order-backend-rust.d /build-out/
RUN ls /build-out/

# Now, we need to build our _real_ Docker container, copying in `using-diesel`.
FROM alpine:latest
RUN apk --no-cache add ca-certificates

COPY --from=builder \
    /build-out \
    /usr/local/bin/order-backend-rust/

WORKDIR /usr/local/bin/order-backend-rust/
RUN chmod +x ./*
CMD /usr/local/bin/order-backend-rust/order-backend-rust
