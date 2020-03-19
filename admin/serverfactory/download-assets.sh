#!/usr/bin/env bash

for file in forge-1.12.2-14.23.5.2838-installer.jar spongeforge-1.12.2-2838-7.1.11-RC4007.jar Enigmatica2Server-1.62.zip
do
    #url="https://storage.cloud.google.com/uplmc-lfs/${file}?organizationId=0"
    url="https://storage.googleapis.com/storage/v1/b/mcupl-lfs/o/${file}?alt=media"    

    echo "checking file ${file}"
    if [ ! -f $file ]
    then
        echo "downloading ${file} from ${url}"
        curl $url --output $file
    fi
    echo ""
done
