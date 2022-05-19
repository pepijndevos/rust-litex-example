#!/usr/bin/env bash
set -ex
riscv64-elf-objcopy $1 -O binary $1.bin
litex_term --kernel $1.bin /dev/ttyUSB1