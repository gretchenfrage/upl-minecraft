#!/usr/bin/env bash
## backup job entrypoint

RESTIC_DB=/resticdb
RESTIC_PASS=/resticpass/password

# create the DB if it's never been created before
if ! restic stats -r "${RESTIC_DB}" > /dev/null
then
    echo "INFO: creating new restic database for backups"
    
    # generate password
    if [ ! -f "${RESTIC_PASS}" ]
    then
        (
        hexdump -n 16 -e '4/4 "%08X" 1 "\n"' /dev/random
        hexdump -n 16 -e '4/4 "%08X" 1 "\n"' /dev/random
        ) > "${RESTIC_PASS}"
    fi

    # init database
    restic init --repo "${RESTIC_DB}" --password-file "${RESTIC_PASS}" || exit 1
else
    
    # assert password exists
    if [ ! -f "${RESTIC_PASS}" ]
    then
        echo "ERROR: restic password not mounted at ${RESTIC_PASS}"
        exit 1
    fi
fi

# backup every 20 minutes
# seconds = 20 * 60

let "SECONDS = 20 * 60"
echo "making a backup every ${SECONDS} seconds"

# make periodic backups
while true
do    
    echo "==== MAKING A BACKUP ====";
    (
        ./mcrcon -p password save off || exit 1;
        echo "backing upar  !";
        ./mcrcon -p password save on || exit 1;
    ) || ./mcrcon -p password save on;
    sleep "${SECONDS}"
done
