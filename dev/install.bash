#!/bin/bash

set -e

# Update rust
rustup update
cargo update

# lint and format
cargo clippy
cargo fmt

# build
cargo build

# copy spf
sudo cp ./target/debug/spf /usr/bin/spf

echo -e "\ninstalled"
