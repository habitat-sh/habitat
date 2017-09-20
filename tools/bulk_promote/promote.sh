#!/bin/bash

# This script takes as input a path to a file of idents, and a channel
# and promotes all the packages listed in the file to the specified
# channel. The file should have one fully qualified ident on each line.

if [ "$#" -ne 2 ] || ! [ -e "$1" ]; then
  echo "Usage: $0 FILE CHANNEL" >&2
  exit 1
fi

echo "Promoting from: $1"
echo "Promoting to  : $2"
echo "Environment   : $HAB_BLDR_URL"
echo "Are you sure?"

select yn in "Yes" "No"; do
    case $yn in
        Yes ) break;;
        No ) exit;;
    esac
done

list=$(cat $1)
for ident in $list; do
  hab pkg promote $ident $2
done
