#!/bin/bash
cargo clippy
echo
RUST_BACKTRACE=1 cargo build -q --release
./target/release/neetcode
