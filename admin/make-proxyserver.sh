#!/usr/bin/env bash

echo "==== building image for proxy server ===="

SCRIPT_DIR=$(cd -P -- "$(dirname -- "$0")" && pwd -P)
cd "${SCRIPT_DIR}" || exit 1

./make-downproxy-binary.sh || exit 1
cp ./downproxy/bin ./proxyserver/downproxy || exit 1


cd ./proxyserver || exit 1
sudo docker build . || exit 1
