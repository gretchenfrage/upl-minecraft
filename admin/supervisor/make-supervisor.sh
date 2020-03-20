 #!/usr/bin/env bash

SCRIPT_DIR=$(cd -P -- "$(dirname -- "$0")" && pwd -P)

## make directory
if [ -z "${1}" ]
then
    VISOR_DIR="${SCRIPT_DIR}/target"
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

## build the docker images
sudo docker build -t javacontainer "${SCRIPT_DIR}/javacontainer" || exit 1
sudo docker build -t backupjob "${SCRIPT_DIR}/backupjob" || exit 1

## create the run script
RUNSCRIPT="${VISOR_DIR}/start_supervisor.sh"
echo '#!/usr/bin/env bash' >> "${RUNSCRIPT}" || exit 1
chmod +x "${RUNSCRIPT}" || exit 1

# go to run script dir
echo 'SCRIPT_DIR=$(cd -P -- "$(dirname -- "$0")" && pwd -P)' >> "${RUNSCRIPT}" || exit 1
echo 'cd "${SCRIPT_DIR}"' >> "${RUNSCRIPT}" || exit 1

# 1. run the mcserver entrypoint
# 2. expose minecraft port (25565) to internet
# 3. expose minecraft RCON port (25575) to localhost
echo 'sudo docker run --env COMMAND="cd /mcserver && ./run.sh" -p 25565:25565 -p 127.0.0.1:25575:25575 --mount "type=bind,src=${PWD}/mcserver,target=/mcserver" -it javacontainer' >> "${RUNSCRIPT}" || exit 1

## create the backupjob script
BKUPSCRIPT="${VISOR_DIR}/start_backupper.sh"
echo '#!/usr/bin/env bash' >> "${BKUPSCRIPT}" || exit 1
chmod +x "${BKUPSCRIPT}" || exit 1

# go to script dir
echo 'SCRIPT_DIR=$(cd -P -- "$(dirname -- "$0")" && pwd -P)' >> "${BKUPSCRIPT}" || exit 1
echo 'cd "${SCRIPT_DIR}"' >> "${BKUPSCRIPT}" || exit 1

mkdir "${VISOR_DIR}/resticdb" || exit 1
mkdir "${VISOR_DIR}/resticpass" || exit 1
echo 'sudo docker run --mount "type=bind,src=${PWD}/resticdb,target=/resticdb" --mount "type=bind,src=${PWD}/resticpass,target=/resticpass" -it backupjob' >> "${BKUPSCRIPT}" || exit 1

