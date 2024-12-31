#!/usr/bin/env bash

set -e;

OUT_PATH="bin";
GAME_PATH="$OUT_PATH/bevy_pong";

exec $GAME_PATH;
