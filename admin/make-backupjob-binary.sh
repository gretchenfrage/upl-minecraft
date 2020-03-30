#!/usr/bin/env bash

echo "==== compiling backupjob-rs ===="

SCRIPT_DIR=$(cd -P -- "$(dirname -- "$0")" && pwd -P)
cd "${SCRIPT_DIR}/backupjob-rs" || exit 1

cargo build --release || exit 1
cp ./target/release/backupjob-rs ./bin || exit 1

