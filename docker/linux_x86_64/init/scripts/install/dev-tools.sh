#!/bin/bash

set -e

sudo apt update -y              \
    && sudo apt-get install -y  \
        build-essential         \
        lldb                    \
        valgrind                \
        qemu-system-i386        \
        grub                    \
        nasm                    \
        xorriso                 \
        grub-pc-bin             \
