# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

RUN apt-get update

RUN apt-get install musl-tools -y

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/order-backend-rust

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/order-backend-rust*

COPY . .

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

RUN addgroup -g 1000 order-backend-rust

RUN adduser -D -s /bin/sh -u 1000 -G order-backend-rust order-backend-rust

WORKDIR /home/order-backend-rust/bin/

COPY --from=cargo-build /usr/src/order-backend-rust/target/x86_64-unknown-linux-musl/release/order-backend-rust .

RUN chown order-backend-rust:order-backend-rust order-backend-rust

USER order-backend-rust

CMD ["./order-backend-rust"]