#!/bin/bash

PATH+=":$HOME/.cargo/bin"
if [[ -f RUSTFMT_VERSION ]]; then
	toolchain="$(< RUSTFMT_VERSION)"
else
	toolchain=stable
fi

{
    if ! rustup run "$toolchain" rustfmt --version; then
        rustup toolchain install "$toolchain"
        rustup component add --toolchain "$toolchain" rustfmt
    fi
} &> /dev/null

rustup run "$toolchain" rustfmt "$@"
