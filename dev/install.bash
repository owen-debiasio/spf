#!/bin/bash

set -e

usage() {
    cat << EOF
Usage: $(basename "$0") [OPTIONS]

Build and install the spf package

Options:
    --help          Show this help message and exit
    --skip-format   Skip running the code formatter before building
    --yes           Automatically answer yes to installation prompts
    --debug         Enable debug output during installation
EOF
}

VERSION="v$(cargo pkgid | cut -d "#" -f2)"
ARCH="$(uname -m)"

DEBUG_PATH="./target/debug/spf"

SKIP_FORMAT=""
YES=""
DEBUG=""

for arg in "$@"; do
    case "$arg" in
        --help)
            usage
            exit 0
            ;;
        --skip-format)
            SKIP_FORMAT="--skip-format"
            ;;
        --yes)
            YES="--yes"
            ;;
        --debug)
            DEBUG="--debug"
            ;;
        *)
            echo "Error: Unknown option '$arg'" >&2
            usage >&2
            exit 1
            ;;
    esac
done

if [[ "$ARCH" != "x86_64" && "$ARCH" != "aarch64" ]]; then
    if [[ -z "$SKIP_FORMAT" ]]; then
        ./dev/format.bash
    fi

    cargo build --release

    sudo cp "$DEBUG_PATH" /usr/bin/spf
else
    ./packages/build.py --version="$VERSION" --arch=auto --type=spf $SKIP_FORMAT

    sudo "$DEBUG_PATH" install ./packages/artifacts/spf-"$VERSION"-"$ARCH"-linux.spf $DEBUG $YES
fi

echo -e "\n> Installed"
