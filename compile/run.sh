#!/bin/bash
#
#
rustc +nightly -Zunpretty=mir ./src/main.rs
# rustc +nightly -Zunpretty=hir ./src/main.rs
