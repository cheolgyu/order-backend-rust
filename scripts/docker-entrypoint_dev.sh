#!/bin/sh


echo "entrypoint.sh start============="
rustc --version
cargo --version
diesel --version
echo "=============diesel migration run============="
diesel migration run
echo "=============cargo watch -x run============="
cargo watch -x run
echo "end============="