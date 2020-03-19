 #!/usr/bin/env bash

SCRIPT_DIR=$(cd -P -- "$(dirname -- "$0")" && pwd -P)

## make directory

if [ -z "${1}" ]
then
    VISOR_DIR=target
else
    VISOR_DIR="${1}"
fi
echo "building supervisor to ${VISOR_DIR}"
mkdir "${VISOR_DIR}" || exit 1


## build the base server, if it's not already built

MAKESERVER_TRGT="${SCRIPT_DIR}/../serverfactory/target"

if [ ! -d "${MAKESERVER_TRGT}" ]
then
    (
        cd $SCRIPT_DIR/../serverfactory/
        ./make-server.sh || exit 1
    ) || exit 1
fi

cp -r "${MAKESERVER_TRGT}" "${VISOR_DIR}/mcserver" || exit 1
