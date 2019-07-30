#https://shaneutt.com/blog/rust-fast-small-docker-image-builds/
# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------
# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------
FROM cargo-build

FROM alpine:edge
RUN apk add --no-cache libpq

#RUN apk search -v 'libpq'
#RUN apk add --no-cache libpq 


RUN addgroup -g 1000 myapp

RUN adduser -D -s /bin/sh -u 1000 -G myapp myapp

WORKDIR /home/myapp/bin/

COPY --from=cargo-build /usr/src/myapp/target/x86_64-unknown-linux-musl/release/order-backend-rust .
COPY --from=cargo-build /usr/src/myapp/.env .
RUN chown myapp:myapp order-backend-rust

#USER myapp


CMD ["./order-backend-rust"]