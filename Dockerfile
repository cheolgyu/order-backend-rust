FROM rust:1.34 as builder

RUN export RUSTC_WRAPPER=sccache 
WORKDIR /usr/src/myapp
RUN cargo install sccache
RUN sccache --start-server
RUN cargo install cargo-watch
RUN ls -al


WORKDIR /usr/src/myapp
COPY . .

#RUN cargo install diesel_cli --no-default-features --features "postgres"
#RUN cargo install --path .
#RUN sccache --show-stats

#CMD ["myapp"]
CMD ["cargo watch -x run"]