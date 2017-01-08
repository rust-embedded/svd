set -ex

main() {
    cross build
    cross test --release
}

if [ -z $TRAVIS_TAG ]; then
    main
fi
