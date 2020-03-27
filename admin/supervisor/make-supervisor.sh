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
echo 'sudo docker run --env COMMAND="cd /mcserver && ./run.sh" -p 25565:25565 -p 127.0.0.1:25575:25575 --mount "type=bind,src=${PWD}/mcserver,target=/mcserver" --rm -d --name upl-minecraft-server javacontainer' >> "${RUNSCRIPT}" || exit 1

## create the backupjob script
BKUPSCRIPT="${VISOR_DIR}/start_backupper.sh"
echo '#!/usr/bin/env bash' >> "${BKUPSCRIPT}" || exit 1
chmod +x "${BKUPSCRIPT}" || exit 1

# go to script dir
echo 'SCRIPT_DIR=$(cd -P -- "$(dirname -- "$0")" && pwd -P)' >> "${BKUPSCRIPT}" || exit 1
echo 'cd "${SCRIPT_DIR}"' >> "${BKUPSCRIPT}" || exit 1

mkdir "${VISOR_DIR}/resticdb" || exit 1
mkdir "${VISOR_DIR}/resticpass" || exit 1
echo 'sudo docker run --network host --mount "type=bind,src=${PWD}/mcserver,target=/mcserver" --mount "type=bind,src=${PWD}/resticdb,target=/resticdb" --mount "type=bind,src=${PWD}/resticpass,target=/resticpass" --rm -d --name upl-minecraft-backup backupjob' >> "${BKUPSCRIPT}" || exit 1



# build the downproxy binary
$SCRIPT_DIR/make-downproxy-binary.sh || exit 1

# copy it into the supervisor target
cp "${SCRIPT_DIR}/downproxy" "${VISOR_DIR}/downproxy" || exit 1

# create the script to maintain the host address object
MHAOSCRIPT="${VISOR_DIR}/maintain_host_address_object.sh"
echo '#!/usr/bin/env bash' >> "${MHAOSCRIPT}" || exit 1
chmod +x "${MHAOSCRIPT}" || exit 1

## set the log level
echo 'export RUST_LOG=info,downproxy=trace' >> "${MHAOSCRIPT}" || exit 1

## set the path to the secret (this is hard-coded for phoenix's computer)
echo 'export TOKEN_PATH=/home/phoenix/secret/mcupl-host-address-editor-service-account/minecraftserver-271604-84eaae3bc93b.json' >> "${MHAOSCRIPT}" || exit 1

## go to script dir
echo 'SCRIPT_DIR=$(cd -P -- "$(dirname -- "$0")" && pwd -P)' >> "${MHAOSCRIPT}" || exit 1
echo 'cd "${SCRIPT_DIR}" || exit 1' >> "${MHAOSCRIPT}" || exit 1

## tell the mhao process to write its own PID
echo 'export WRITE_OWN_PID_TO=./mhao_pid' >> "${MHAOSCRIPT}" || exit 1

## run the downproxy in maintain_host_address_object mode
echo './downproxy maintain_host_address_object || (echo "ERROR: maintain_host_address_object script failed"; exit 1)' >> "${MHAOSCRIPT}" || exit 1


# copy in the boilerplate scripts
for f in $(cd "${SCRIPT_DIR}/boilerplate-scripts" && ls)
do
    cp "${SCRIPT_DIR}/boilerplate-scripts/${f}" "${VISOR_DIR}/${f}" || exit 1
    chmod +x "${VISOR_DIR}/${f}" || exit 1
done
