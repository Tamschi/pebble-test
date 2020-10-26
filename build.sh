#! /bin/bash

set -e
. ~/.bashrc_non_interactive

cd /mnt/workspace
cd $FOLDER

# cargo clean
RUST_TARGET_PATH=/mnt/workspace \
	cargo build --release --target thumbv7m-pebble-eabi

rm -rf build
pebble build
