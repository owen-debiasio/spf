#!/bin/bash

set -e

# Update rust
rustup update
cargo update

# lint and format
./dev/format.bash

# build
cargo build

# copy spf
sudo cp ./target/debug/spf /usr/bin/spf

echo -e "\ninstalled"
