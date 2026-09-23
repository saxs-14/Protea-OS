#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="${ROOT}/output/pc"
BUILDROOT="$("${ROOT}/build/fetch-buildroot.sh")"

mkdir -p "${BUILD_DIR}"
"${ROOT}/build/prepare-pc-overlay.sh"
make -C "${BUILDROOT}" O="${BUILD_DIR}" qemu_x86_64_defconfig
make -C "${BUILDROOT}" O="${BUILD_DIR}" BR2_ROOTFS_OVERLAY="${ROOT}/build/pc-overlay"

echo "Build completed."
echo "Images: ${BUILD_DIR}/images"
