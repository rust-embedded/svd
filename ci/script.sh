set -ex

main() {
    cross build --all-features
    cross test --release --all-features
}

if [ -z $TRAVIS_TAG ]; then
    main
fi
