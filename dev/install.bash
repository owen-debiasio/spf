#!/bin/bash

set -e

VERSION="v$(cargo pkgid | cut -d "#" -f2)"
ARCH="$(uname -m)"

DEBUG_PATH="./target/debug/spf"

if [[ "$ARCH" != "x86_64" && "$ARCH" != "aarch64" ]]; then
    if [[ "$1" != "--no-format" ]]; then
        ./dev/format.bash
    fi

    cargo build

    sudo cp "$DEBUG_PATH" /usr/bin/spf
else
    if [[ "$1" != "--skip-format" ]]; then
        ./packages/build.py --version="$VERSION" --arch=auto --type=spf
    else
        ./packages/build.py --version="$VERSION" --arch=auto --type=spf --skip-format
    fi

    sudo "$DEBUG_PATH" install ./packages/artifacts/spf-"$VERSION"-"$ARCH"-linux.spf
fi

echo -e "\n> Installed"
