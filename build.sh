#! /bin/bash

set -e
. ~/.bashrc_non_interactive

cd /mnt/workspace
cd $FOLDER

cargo clean

RUST_TARGET_PATH=/mnt/workspace \
	cargo build --release --target thumbv7m-none-eabi

pebble build
