#!/usr/bin/env bash

SCRIPT_DIR=$(cd -P -- "$(dirname -- "$0")" && pwd -P)

## make directory
if [ -z "${1}" ]
then
    VISOR_DIR="${SCRIPT_DIR}/target"
else
    VISOR_DIR="${1}"
fi
echo "rebuilding backupjob in ${VISOR_DIR}"
if [ ! -d "${VISOR_DIR}" ]
then
    echo "[ERROR] hey wait, that directory doesn't exist"
    exit 1
fi

## rebuild docker image
sudo docker build -t backupjob "${SCRIPT_DIR}/backupjob" || exit 1

## create the backupjob script
BKUPSCRIPT="${VISOR_DIR}/start_backupper.sh"
rm "${BKUPSCRIPT}" || exit 1
echo '#!/usr/bin/env bash' >> "${BKUPSCRIPT}" || exit 1
chmod +x "${BKUPSCRIPT}" || exit 1

# go to script dir
echo 'SCRIPT_DIR=$(cd -P -- "$(dirname -- "$0")" && pwd -P)' >> "${BKUPSCRIPT}" || exit 1
echo 'cd "${SCRIPT_DIR}"' >> "${BKUPSCRIPT}" || exit 1

# run the container
echo 'sudo docker run --network host --mount "type=bind,src=${PWD}/resticdb,target=/resticdb" --mount "type=bind,src=${PWD}/resticpass,target=/resticpass" -it backupjob' >> "${BKUPSCRIPT}" || exit 1
