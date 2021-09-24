#!/bin/bash

set -euxo pipefail

cargo build
cargo build --target thumbv6m-none-eabi
cargo build --target thumbv7em-none-eabi
cargo build --target riscv32imc-unknown-none-elf
cargo build --target riscv32imac-unknown-none-elf
