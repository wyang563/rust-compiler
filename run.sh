#!/usr/bin/env bash
cargo build
RUST_BACKTRACE=1 target/debug/rust-compiler "$@"
CODE=$?
echo "Exit code: $CODE"