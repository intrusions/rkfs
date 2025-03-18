#!/bin/bash

set -e

sudo apt update -y              \
    && sudo apt-get install -y  \
        tmux                    \
        procps                  \
        iproute2                \
        curl                    \
        git                     \
        tree                    \
        vim                     \
        wget                    \
