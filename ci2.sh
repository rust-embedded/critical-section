#!/bin/bash

set -euxo pipefail

cargo +nightly-2021-01-07 build -Zbuild-std=core --target avr-specs/avr-atmega328p.json
