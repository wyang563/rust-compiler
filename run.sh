#!/usr/bin/env bash
cargo build
target/debug/rust-compiler "$@"
# CODE=$?
# echo "Exit code: $CODE"