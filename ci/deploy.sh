#!/bin/bash

set -o errexit -o nounset

echo "Running $0"

if [ $# -eq 1 ]
then
	if [ $1 = "manual" ]
	then
		echo "manually run"
		CONTENT_TESTS=1
		UPSTREAM_URL="git@github.com:jamesgraves/rust-cookbook.git"
		TRAVIS_REPO_SLUG="manual_build"
	fi
else
	if [ -z "${TRAVIS_BRANCH:-}" ]; then
	    echo "This script may only be run from Travis!"
	    exit 1
	fi

	if [[ "$TRAVIS_BRANCH" != "main" || "$TRAVIS_RUST_VERSION" != "stable" || "$TRAVIS_OS_NAME" != "linux" ]]; then
	    echo "This commit was made against '$TRAVIS_BRANCH' with '$TRAVIS_RUST_VERSION' on '$TRAVIS_OS_NAME'."
	    echo "Instead of 'main' branch with 'stable' on 'linux'!"
	    echo "Not deploying!"
	    exit 0
	fi

	UPSTREAM_URL="https://$GH_TOKEN@github.com/jamesgraves/rust-cookbook.git"
fi

if [ -z "${CONTENT_TESTS:-}" ]; then
    # check for outdated dependencies on nightly builds
    if [ "${TRAVIS_EVENT_TYPE:-}" == "cron" ]; then
        echo "This is cron build. Checking for outdated dependencies!"
        rm ./Cargo.lock
        cargo clean
        # replace all [dependencies] versions with "*"
        sed -i -e "/^\[dependencies\]/,/^\[.*\]/ s|^\(.*=[ \t]*\).*$|\1\"\*\"|" ./Cargo.toml

        cargo test || { echo "Cron build failed! Dependencies outdated!"; exit 1; }
        echo "Cron build success! Dependencies are up to date!"
        exit 0
    fi

    echo "We deploy only after we also test the markup and descriptions!"
    exit 0
fi

# Returns 1 if program is installed and 0 otherwise
program_installed() {
    local return_=1

    type $1 >/dev/null 2>&1 || { local return_=0; }

    echo "$return_"
}

# Ensure required programs are installed
if [ $(program_installed git) == 0 ]; then
    echo "Please install Git."
    exit 1
elif [ $(program_installed mdbook) == 0 ]; then
    echo "Please install mdbook: cargo install mdbook."
    exit 1
fi

echo "Building site to book/"
mdbook build

echo "Copying assets/* to book/"
cp -r assets/ book/

echo "Committing book directory to gh-pages branch"
REV=$(git rev-parse --short HEAD)
cd book

git init -b gh-pages
git remote add upstream $UPSTREAM_URL
git config user.name "Rust Cookbook"
git config user.email "cookbook@rust-lang.org"
git add -A .
git commit -qm "Build cookbook at ${TRAVIS_REPO_SLUG}@${REV}"

echo "Pushing gh-pages to GitHub"
git push -q upstream HEAD:refs/heads/gh-pages --force
