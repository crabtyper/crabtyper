#!/bin/sh

set -ex

# # install latest trunk version
# wget -qO- https://github.com/thedodd/trunk/releases/download/${VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-

cd crates/web/

npm install
npm run build
