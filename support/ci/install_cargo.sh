#!/bin/bash
set -eu

curl -s https://static.rust-lang.org/cargo-dist/cargo-nightly-x86_64-unknown-linux-gnu.tar.gz | tar xzm
cargo-nightly-x86_64-unknown-linux-gnu/install.sh --prefix=$HOME
