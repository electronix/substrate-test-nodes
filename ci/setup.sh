#!/usr/bin/env bash

set -eux

# Install packages used for substrate
sudo apt-get -y update
sudo apt-get install -y cmake pkg-config libssl-dev

# Install wasm32 for nightly
rustup target add wasm32-unknown-unknown

# Install wasm-gc
if ! [ -x "$(command -v wasm-gc)" ]; then
    cargo install --git https://github.com/alexcrichton/wasm-gc
else
    echo "wasm-gc already installed"
fi

# Show rust information
rustc --version
rustup --version
cargo --version
