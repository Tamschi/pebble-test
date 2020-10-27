#! /bin/bash

set -e
. ~/.bashrc_non_interactive

cd /mnt/workspace
cd $FOLDER

cargo clean
RUST_TARGET_PATH=/mnt/workspace \
	cargo build --target thumbv7m-pebble-eabi --release

rm -rf build
pebble build --debug
# pebble install --emulator basalt
# pebble gdb --emulator basalt

xeyes
