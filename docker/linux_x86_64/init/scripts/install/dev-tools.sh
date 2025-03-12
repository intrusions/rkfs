#!/bin/bash

set -e

sudo apt update -y              \
    && sudo apt-get install -y  \
        build-essential         \
        lldb                    \
        python3                 \
        python3-pip             \
        valgrind                \
        sparse                  \
        strace                  \
        readline-doc            \
        libreadline-dev         \
        kmod                    \
        dwarves                 \
        clang                   \
        clang-format            \
        cmake                   \
        libssl-dev              \
        qemu-system-i386        \
