#!/bin/bash
set -e

[[ $# -eq 1 ]] || {
    echo "Provide package version (MUST START WITH v)"
    exit 1
}
[[ "$1" == v* ]] || {
    echo "Version must start with 'v'"
    exit 1
}

# Get Rust ready
echo "Installing dependencies..."

./dev/format.bash
./packages/clean.bash

PACKAGE="spf-$1-x86_64-linux"

# Build
cargo build --release

echo building x86_64

# Write metadata
echo -e "
:::META DEFINE START:::
PROJECT_NAME = spf
VERSION = $1
DESCRIPTION = Simple package manager
LICENSE = gplv3
AUTHORS = Owen DeBiasio
ARCH = x86_64
:::META DEFINE END:::

:::PATH DEFINE START:::
./target/release/spf:/usr/bin/spf
changes.md:/usr/share/doc/spf/changes.md
README.md:/usr/share/doc/spf/README.md
LICENSE:/usr/share/licenses/spf/LICENSE
:::PATH DEFINE END:::" > spfbuildcfg

# Create .spf package
./target/release/spf create spfbuildcfg ./packages/"$PACKAGE".spf

# Remove no longer needed metadata
rm spfbuildcfg

# Create .deb package
cargo-deb -o "./packages/$PACKAGE.deb"

# Create .rpm package
cargo-generate-rpm -o "./packages/$PACKAGE.rpm"

# Copy binary
cp ./target/release/spf "./packages/$PACKAGE"

echo building aarch64

PACKAGE="spf-$1-aarch64-linux"

# Build
cargo build --target aarch64-unknown-linux-gnu --release

# Write metadata
echo -e "
:::META DEFINE START:::
PROJECT_NAME = spf
VERSION = $1
DESCRIPTION = Simple package manager
LICENSE = gplv3
AUTHORS = Owen Debiasio
ARCH = aarch64
:::META DEFINE END:::

:::PATH DEFINE START:::
./target/aarch64-unknown-linux-gnu/release/spf:/usr/bin/spf
changes.md:/usr/share/doc/spf/changes.md
README.md:/usr/share/doc/spf/README.md
LICENSE:/usr/share/licenses/spf/LICENSE
:::PATH DEFINE END:::" > spfbuildcfg

# Create .spf package
./target/release/spf create spfbuildcfg ./packages/"$PACKAGE".spf

# Remove no longer needed metadata
rm spfbuildcfg

# Create .deb package
cargo-deb --target aarch64-unknown-linux-gnu -o "./packages/$PACKAGE.deb"

# Create .rpm package
cargo-generate-rpm --target aarch64-unknown-linux-gnu -o "./packages/$PACKAGE.rpm"

# Copy binary
cp ./target/aarch64-unknown-linux-gnu/release/spf "./packages/$PACKAGE"
echo "done"
