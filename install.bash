#!/bin/bash

rustup update
cargo update

cargo clippy
cargo fmt
cargo build

sudo cp target/debug/spf /usr/bin/spf

echo -e "\ninstalled"
