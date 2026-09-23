#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BUILD_DIR="${ROOT}/output/pc"
KERNEL="${BUILD_DIR}/images/bzImage"
ROOTFS="${BUILD_DIR}/images/rootfs.ext2"

[[ -f "${KERNEL}" ]] || { echo "Missing ${KERNEL}" >&2; exit 1; }
[[ -f "${ROOTFS}" ]] || { echo "Missing ${ROOTFS}" >&2; exit 1; }

exec qemu-system-x86_64   -m 1024   -smp 2   -kernel "${KERNEL}"   -drive "file=${ROOTFS},format=raw,if=virtio"   -device virtio-gpu-pci   -append "root=/dev/vda console=ttyS0"   -nographic
