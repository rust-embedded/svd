# drops the cmsis-svd submodule, which we don't want in our crates.io release, and publishes this
# crate

set -ex

main() {
    local td=$(mktemp -d)

    git clone . $td
    cd $td
    rm .gitmodules
    rmdir cmsis-svd
    git add .
    git commit -m 'clean up'

    cargo publish
}

main
