#!/bin/bash

set -e

if [ -f ./.SPF_DEV_CFG ]; then
    # shellcheck source=/dev/null
    . ./.SPF_DEV_CFG
fi

prettier --config .prettierrc -w .
markdownlint-cli2 "**/*.md" --fix

cargo clippy
cargo fmt

find . -name "*.bash" -exec shellcheck {} +
shfmt -w -i 4 -ci -sr .

if command -v ruff > /dev/null 2>&1; then
    RUFF_EXEC=ruff
elif [ -e "./.venv/bin/ruff" ]; then
    RUFF_EXEC=./.venv/bin/ruff
elif [ -v "$SPF_DEV_CFG" ]; then
    RUFF_EXEC="$SPF_DEV_CFG"
else
    read -p "> Ruff not found. Enter path for Ruff, or 'skip' to ignore this: " -r resp
    if [ -e "$resp" ]; then
        echo -e "RUFF_PATH=\"$resp\"" > ./.SPF_DEV_CFG

        RUFF_EXEC="$resp"
    else
        echo "> Skipping..."
        exit
    fi
fi

"$RUFF_EXEC" check
"$RUFF_EXEC" format
