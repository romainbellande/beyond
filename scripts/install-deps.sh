#!/usr/bin/env bash

echo "installing mkcert...."

sudo pacman -Syu mkcert

sudo bash -c "$(curl --location https://taskfile.dev/install.sh)" -- -d -b /usr/local/bin;

echo "installation finished!"