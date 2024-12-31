#!/usr/bin/env bash

set -e;

OUT_PATH="bin";
GAME_PATH="$OUT_PATH/bevy_pong";

BUILD_BIN_PATH="target/release/bevy_pong";

cargo build --release --no-default-features $@;

rm -rf $GAME_PATH;
cp $BUILD_BIN_PATH $GAME_PATH;
