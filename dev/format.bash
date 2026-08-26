#!/bin/bash

set -e

prettier --config .prettierrc -w .
markdownlint-cli2 "**/*.md" --fix

rustup update
cargo update

cargo clippy
cargo fmt

rustup target add aarch64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-gnu

find . -name "*.bash" -exec shellcheck {} +
shfmt -w -i 4 -ci -sr .
