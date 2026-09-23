#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="${ROOT}/output/pc"
BUILDROOT="$("${ROOT}/build/fetch-buildroot.sh")"

mkdir -p "${BUILD_DIR}"
"${ROOT}/build/prepare-pc-overlay.sh"
make -C "${BUILDROOT}" BR2_EXTERNAL="${ROOT}/build/br2-external" O="${BUILD_DIR}" qemu_x86_64_defconfig
printf "\nBR2_PACKAGE_PROTEA_CORE_CLI=y\nBR2_ROOTFS_OVERLAY=\"${ROOT}/build/pc-overlay\"\n" >> "${BUILD_DIR}/.config"
make -C "${BUILDROOT}" BR2_EXTERNAL="${ROOT}/build/br2-external" O="${BUILD_DIR}" olddefconfig
make -C "${BUILDROOT}" BR2_EXTERNAL="${ROOT}/build/br2-external" O="${BUILD_DIR}"

echo "Build completed."
echo "Images: ${BUILD_DIR}/images"
