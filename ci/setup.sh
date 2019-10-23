#!/usr/bin/env bash

set -eux

# Install packages used for substrate
sudo apt-get -y update
sudo apt-get install -y cmake pkg-config libssl-dev

# Show rust information
rustc --version
rustup --version
cargo --version
