#!/bin/sh
set -xe

cargo test
cargo build --release
