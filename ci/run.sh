set -ex

main() {
    cargo build
    cargo test --release
}

main $1
