#!/bin/bash
#
#
# rustc +nightly -Zunpretty=mir ./src/main.rs
# rustc +nightly -Zunpretty=hir ./src/main.rs

# rustc --emit=mir ./src/main.rs


rustc --emit=mir -C opt-level=1 ./src/main.rs

