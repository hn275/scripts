#!/usr/bin/env bash

find /home/haln -type d \( \
    -name node_modules \
    -o -name .git \
    -o -name .npm \
    -o -name .nvm \
    -o -name .rustup \
    -o -name .yarn \
    -o -name .local \
    -o -name .cache \
    -o -name .npm-global \
    -o -name .docker \
    -o -name .mozilla \
    -o -name .pki \
    -o -name .ssh \
    -o -name .cargo \) \
    -prune \
    -o -name '*' -type d -print | fzf
