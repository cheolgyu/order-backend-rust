#!/bin/sh


echo "12entrypoint_op.sh start============="
rustc --version
cargo --version
diesel --version
##echo "=============cargo build --release============="
cargo build --release
echo "=============diesel migration run============="
diesel migration run
echo "=============cargo build --release============="
./target/release/api
#cargo run --release
echo "end============="