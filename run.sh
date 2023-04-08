#! /bin/bash
RUST_BACKTRACE=1 RUST_LOG="warn,bevy-pixel-tanks=debug"  cargo watch -q -c -x 'run --features bevy/dynamic_linking'