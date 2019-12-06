#!/bin/bash

PATH+=":$HOME/.cargo/bin"
if [[ -f RUSTFMT_VERSION ]]; then
	toolchain="$(< RUSTFMT_VERSION)"
else
	toolchain=stable
fi

{
    if ! rustup run "$toolchain" rustfmt --version; then
        rustup set profile minimal
        rustup toolchain install "$toolchain"
        rustup component add --toolchain "$toolchain" rustfmt
        rustup set profile default
    fi
} &> /dev/null

cargo +"${toolchain}" fmt -- "$@"
