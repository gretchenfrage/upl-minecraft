#!/usr/bin/env bash

# SERVER_DIR="gen-serv-${RANDOM}"
SERVER_DIR=target
mkdir $SERVER_DIR || exit 1
# if [ ! -f $SERVER_DIR ]; then mkdir $SERVER_DIR || exit 1; fi
cd $SERVER_DIR || exit 1
echo "creating server in ${SERVER_DIR}"

# download assets
echo "==== DOWNLOADING MISSING ASSETS ===="
./download-assets.sh

# unzip base server
echo "==== UNZIPPING BASE SERVER ===="
unzip ../Enigmatica2Server-1.62.zip > /dev/null || exit 1

# patch server with forge
echo "==== PATCHING SERVER WITH FORGE ===="
java -jar ../forge-1.12.2-14.23.5.2838-installer.jar --installServer

# install SpongeForge (serverside plugin loader)
cp ../spongeforge-1.12.2-2838-7.1.11-RC4007.jar mods/__aa_spongeforge.jar

# agree to minecraft's EULA
echo "eula=true" > eula.txt

# copy over our server settings
cp ../server.properties ./

# copy over our run script
cp ../run.sh ./

# sponge foam fix
cp ../foamfix.cfg config/

