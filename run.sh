#!/usr/bin/env bash
cargo build
target/debug/rust-compiler "$@"