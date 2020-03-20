#!/usr/bin/env bash

if [ -f mcrcon ]
then
    exit 0
fi

echo "==== building mcrcon ===="
git clone git@github.com:Tiiffi/mcrcon.git || exit 1
cd mcrcon || exit 1
make || exit 1
cd .. || exit 1
cp mcrcon/mcrcon backupjob/mcrcon || exit 1
