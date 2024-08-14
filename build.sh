#!/bin/bash

set -e

apt-get update
apt-get install -y curl build-essential

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

cargo build --release