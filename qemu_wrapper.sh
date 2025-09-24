#!/usr/bin/env bash
qemu-system-i386 -serial stdio -device isa-debug-exit,iobase=0xf4,iosize=0x01 -m 512 -kernel "$1"

exit $(($? >> 1))