#!/bin/bash

# Linux
#cargo rustc -- -C link-arg=-nostartfiles
# Windows
#cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"
# macOS
cargo rustc -v --target "riscv64imac-unknown-none-elf"