#!/bin/bash

set -e

prettier --config .prettierrc -w .
markdownlint-cli2 "**/*.md" --fix

cargo update

cargo clippy
cargo fmt

find . -name "*.bash" -exec shellcheck {} +
shfmt -w -i 4 -ci -sr .
