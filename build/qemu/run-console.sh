#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
IMAGE_DIR="${ROOT}/output/pc/images"
KERNEL="${IMAGE_DIR}/bzImage"
ROOTFS="${IMAGE_DIR}/rootfs.ext2"

command -v qemu-system-x86_64 >/dev/null || {
  echo "qemu-system-x86_64 is required." >&2
  exit 1
}

test -f "${KERNEL}" || {
  echo "Missing ${KERNEL}. Run ./build/build-pc.sh first." >&2
  exit 1
}

test -f "${ROOTFS}" || {
  echo "Missing ${ROOTFS}. Rebuild with an ext2 root filesystem." >&2
  exit 1
}

exec qemu-system-x86_64 \
  -M pc \
  -m 1024 \
  -smp 2 \
  -kernel "${KERNEL}" \
  -drive "file=${ROOTFS},format=raw,if=virtio" \
  -append "root=/dev/vda rw console=ttyS0" \
  -serial mon:stdio \
  -no-reboot
