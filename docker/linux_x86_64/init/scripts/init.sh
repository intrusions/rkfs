#!/bin/bash

set -eu 

readonly     HOME_DIR="/home/${USER_NAME}"

CreateDirectory() {
    local directory="$1"
    local permissions="$2"

    if [ ! -e "${directory}" ];
        then mkdir -p "${directory}" && echo "Created: ${directory}"
        else echo "Retrieved: ${directory}"
    fi

    chmod "${permissions}" "${directory}"
    chown "${USER_NAME}:${USER_NAME}" "${directory}"
}

chmod 711 "${HOME_DIR}"

while true; do sleep 1; done
