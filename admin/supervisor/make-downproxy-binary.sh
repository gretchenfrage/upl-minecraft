#!/usr/bin/env bash

echo "==== compiling downproxy ===="

SCRIPT_DIR=$(cd -P -- "$(dirname -- "$0")" && pwd -P)
cd "${SCRIPT_DIR}/../downproxy" || exit 1

cargo build --release || exit 1
cp target/release/downproxy "${SCRIPT_DIR}"/downproxy || exit 1

