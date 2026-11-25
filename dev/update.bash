#!/usr/bin/bash

[[ $# -eq 1 ]] || {
    echo "Provide package version (MUST START WITH v)"
    exit 1
}

[[ "$1" == v* ]] || {
    echo "Version must start with 'v'"
    exit 1
}

git commit --allow-empty -m "Update to $1"
git tag "$1"
git push origin HEAD --tags

echo "Done"
