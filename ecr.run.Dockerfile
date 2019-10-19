#https://shaneutt.com/blog/rust-fast-small-docker-image-builds/
# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------
# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

ARG BASE_IMAGE=410450153592.dkr.ecr.ap-northeast-2.amazonaws.com/base:latest
FROM ${BASE_IMAGE} AS cargo-build

COPY . .

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:edge
RUN apk add --no-cache libpq
RUN apk add --no-cache openssl

RUN addgroup -g 1000 myapp

RUN adduser -D -s /bin/sh -u 1000 -G myapp myapp

WORKDIR /home/myapp/bin/

COPY --from=cargo-build /usr/src/myapp/target/x86_64-unknown-linux-musl/release/order-backend-rust .
COPY --from=cargo-build /usr/src/myapp/.env.production .env
RUN chown myapp:myapp order-backend-rust

CMD ["./order-backend-rust"]

# $(aws ecr get-login --no-include-email --region ap-northeast-2)
# docker tag prod-order-backend-rust:latest 410450153592.dkr.ecr.ap-northeast-2.amazonaws.com/order-backend-rust:latest
# docker push 410450153592.dkr.ecr.ap-northeast-2.amazonaws.com/order-backend-rust:latest

# $(aws ecr get-login --no-include-email --region ap-northeast-2)
# docker pull 410450153592.dkr.ecr.ap-northeast-2.amazonaws.com/order-backend-rust:latest