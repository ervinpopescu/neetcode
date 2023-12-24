#!/bin/bash
cargo -q clippy
echo
RUST_BACKTRACE=1 cargo build -q --release
./target/release/neetcode
