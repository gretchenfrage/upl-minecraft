#!/usr/bin/env bash

SCRIPT_DIR=$(cd -P -- "$(dirname -- "$0")" && pwd -P)
cd "${SCRIPT_DIR}" || exit 1

echo "==== shutting down server ===="
if [ ! -f server.lock ]
then
    echo "[ERROR] server.lock file doesn't exist!"
    exit 1
fi

# remove the lock file
rm server.lock

# kill the maintain host address object process
kill $(cat mhao_pid)

# kill the docker containers
docker kill upl-minecraft-server
docker kill upl-minecraft-backup

