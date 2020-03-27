#!/usr/bin/env bash

SCRIPT_DIR=$(cd -P -- "$(dirname -- "$0")" && pwd -P)
cd "${SCRIPT_DIR}" || exit 1

echo "==== launching server ===="
if [ -f server.lock ]
then
    echo "[ERROR] server.lock file already exists!"
    exit 1
fi

touch server.lock || exit 1
if [ -f mhao.log ]
then
    rm mhao.log
fi

if [ -f mhao_pid ]
then
    rm mhao_pid
fi

echo "==== launching maintain host address object process ===="
nohup ./maintain_host_address_object.sh > ./mhao.log 2>&1 &

echo "==== launching minecraft server ===="
./start_supervisor.sh || exit 1

echo "==== launching backup job ===="
./start_backupper.sh || exit 1

