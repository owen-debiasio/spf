#!/bin/bash

set -e

VERSION="v$(cargo pkgid | cut -d "#" -f2)"
ARCH="$(uname -m)"

DEBUG_PATH="./target/debug/spf"

if [[ "$ARCH" != "x86_64" && "$ARCH" != "aarch64" ]]; then
    ./dev/format.bash

    cargo build

    sudo cp "$DEBUG_PATH" /usr/bin/spf
else
    ./packages/build.py --version="$VERSION" --arch="$ARCH" --type=spf

    sudo "$DEBUG_PATH" install ./packages/artifacts/spf-"$VERSION"-"$ARCH"-linux.spf
fi

echo -e "\n> Installed"
