#!/usr/bin/env bash
## backup job entrypoint

RESTIC_DB=/resticdb
RESTIC_PASS=/resticpass/password

# create the DB if it's never been created before
if ! ([ -f "${RESTIC_PASS}" ] && restic stats --password-file "${RESTIC_PASS}" -r "${RESTIC_DB}" > /dev/null)
then
    echo "INFO: creating new restic database for backups"
    
    # generate password
    if [ ! -f "${RESTIC_PASS}" ]
    then
        (
        hexdump -n 16 -e '4/4 "%08X" 1 "\n"' /dev/random
        hexdump -n 16 -e '4/4 "%08X" 1 "\n"' /dev/random
        ) | tr -d '\n' > "${RESTIC_PASS}"
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

if [ -z "${BACKUP_PERIOD}" ]
then
    let "SECONDS = 20 * 60"
else
    SECONDS="${BACKUP_PERIOD}"
fi
echo "making a backup every ${SECONDS} seconds"

# make periodic backups
while true
do    
    echo "==== MAKING A BACKUP ===="
    (
        ./mcrcon -p password 'say now creating server backup.' || exit 1;
        ./mcrcon -p password 'save-off' || exit 1;

        echo "backing up server";
        restic -r "${RESTIC_DB}" --password-file "${RESTIC_PASS}" backup --exclude /mcserver/logs /mcserver || exit 1;

        ./mcrcon -p password 'save-on' || exit 1;
        ./mcrcon -p password 'say server backup complete.' || exit 1;

    ) || (
        ./mcrcon -p password 'say [WARN] server backup error.';
        ./mcrcon -p password 'save-on';
    )
    sleep "${SECONDS}"
done
