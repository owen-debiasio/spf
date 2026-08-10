#!/bin/bash
set -e

[[ $# -eq 1 ]] || { echo "Provide package version (MUST START WITH v)"; exit 1; }
[[ "$1" == v* ]] || { echo "Version must start with 'v'"; exit 1; }

PACKAGE="spf-$1-x86_64-linux"

# Get Rust ready
echo "Installing dependencies..."

# Update rust
rustup update
cargo update

# Lint and format
cargo clippy
cargo fmt

# Build
cargo build --release

# Write metadata
echo -e "
:::META DEFINE START:::
PROJECT_NAME = spf
VERSION = $1
DESCRIPTION = Simple package manager
LICENSE = gplv3
AUTHORS = Owen Debiasio
ARCH = x86_64
:::META DEFINE END:::

:::PATH DEFINE START:::
./target/release/spf:/usr/bin/spf
:::PATH DEFINE END:::" > spfbuildcfg

# Create .spf package
./target/release/spf create spfbuildcfg ./packages/"$PACKAGE".spf

# Remove no longer needed metadata
rm spfbuildcfg

# Copy binary
cp ./target/release/spf "./packages/$PACKAGE"

echo "done"
