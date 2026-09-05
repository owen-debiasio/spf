[[ $# -eq 1 ]] || {
    echo "Provide package version (MUST START WITH v)"
    exit 1
}
[[ "$1" == v* ]] || {
    echo "Version must start with 'v'"
    exit 1
}

git tag "$1"
git push origin "$1"

echo "Done"
