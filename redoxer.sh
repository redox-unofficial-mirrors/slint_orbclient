#!/usr/bin/env bash

set -ex

rm -rf target/redoxer
mkdir -p target/redoxer

redoxer install \
    --no-track \
    --path examples/simple \
    --root "target/redoxer"

cd target/redoxer

redoxer exec \
    --gui \
    --folder . \
    ./bin/simple
