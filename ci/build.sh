#!/bin/sh

set -ex

VERSION=0.15.0

# install latest trunk version
wget -qO- https://github.com/thedodd/trunk/releases/download/${VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-

cd web/

npm install
npm run prod
