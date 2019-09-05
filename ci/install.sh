set -ex

main() {
    if [ -x "$(command -v cross)" ]; then
        cross -V
    else
        cargo install cross
    fi
}

main
