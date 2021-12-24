#/bin/bash

set -eu



cargo build --release

for n in {0..50}
do
    echo "================"
    echo "test_num #${n}"
    test_num=`printf '%04g' $n`
    "../target/release/a" < "tools/in/${test_num}.txt" > "tools/out/${test_num}.txt"
    cd tools
    cargo run --release --bin vis "in/${test_num}.txt" "out/${test_num}.txt"
    cd ..
done
