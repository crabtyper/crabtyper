#!/bin/sh

set -ex

CI=true
VERSION=v0.15.0
web="$(pwd)/web"

# Build the API (using the Rust toolchain + wasm-pack)
cd $web
npm install
wget -qO- https://github.com/thedodd/trunk/releases/download/${VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-

npm run prod
