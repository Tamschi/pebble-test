#! /bin/bash

cd /mnt/workspace

RUN . ~/.bashrc_non_interactive; xargo build --$PACKAGE --release --target arm-none-eabi
RUN . ~/.bashrc_non_interactive; pebble build
