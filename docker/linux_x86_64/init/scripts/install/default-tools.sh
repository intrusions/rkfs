#!/bin/bash

set -e

sudo apt update -y              \
    && sudo apt-get install -y  \
        man-db                  \
        manpages                \
        tmux                    \
        procps                  \
        iproute2                \
        curl                    \
        git                     \
        tree                    \
        vim                     \
        xclip                   \
        net-tools               \
        libtool                 \
        iputils-ping            \
        bsdmainutils            \
        netcat-traditional      \
        systemd                 \
        openssh-server          \
        wget                    \
