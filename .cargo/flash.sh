#!/usr/bin/env bash
set -ex
# Create bin file
riscv64-elf-objcopy $1 -O binary $1.bin
# Program iCEBreaker
litex_term --kernel litex-example.bin /dev/ttyUSB0