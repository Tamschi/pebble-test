#! /bin/bash

set -e
. ~/.bashrc_non_interactive

cd /mnt/workspace
cargo clean

cd $FOLDER

xargo build --release --target arm-none-eabi
pebble build
