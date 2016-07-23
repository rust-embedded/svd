set -ex

if [[ "$TRAVIS_PULL_REQUEST" = 'false' ]]; then
    ./ghp-import/ghp_import.py target/doc
    set +x
    git push -fq "https://${GH_TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git" gh-pages && echo OK
    set -x
fi
