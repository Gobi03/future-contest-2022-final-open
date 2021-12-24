#/bin/bash

set -eu

test_num="0025"

cargo build --release
"../target/release/a" < "tools/in/${test_num}.txt" > "tools/out/${test_num}.txt"

cd tools
cargo run --release --bin vis "in/${test_num}.txt" "out/${test_num}.txt"
