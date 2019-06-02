ARG BASE_IMAGE=ekidd/rust-musl-builder:stable-openssl11
FROM ${BASE_IMAGE} AS builder
RUN cargo install diesel_cli --no-default-features --features "postgres"
ADD . ./
RUN sudo chown -R rust:rust /home/rust
RUN cargo build --release

RUN mkdir -p /build-out

RUN cp target/x86_64-unknown-linux-musl/release/order-backen-rust /build-out/

RUN ls /build-out/


# Now, we need to build our _real_ Docker container, copying in `using-diesel`.
FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/order-backend-rust \
    /usr/local/bin/
RUN diesel migration redo
CMD /usr/local/bin/order-backend-rust