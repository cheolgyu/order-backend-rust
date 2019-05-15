FROM rust:1.34.2-stretch as builder

RUN export RUSTC_WRAPPER=sccache 
RUN export RUST_BACKTRACE=1
WORKDIR /usr/src/app
RUN cargo install sccache
RUN sccache --start-server
RUN cargo install cargo-watch
RUN ls -al

COPY . .
RUN cargo install diesel_cli --no-default-features --features postgres
#CMD ["cargo watch -x run"]