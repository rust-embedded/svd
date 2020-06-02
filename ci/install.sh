set -ex

main() {
    if [ -x "$(command -v cross)" ]; then
        cross -V
    else
        rustup install stable
        cargo +stable install cross
    fi
}

main
