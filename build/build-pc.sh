#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="${ROOT}/output/pc"
BUILDROOT="$("${ROOT}/build/fetch-buildroot.sh")"

mkdir -p "${BUILD_DIR}"
"${ROOT}/build/prepare-pc-overlay.sh"
make -C "${BUILDROOT}" BR2_EXTERNAL="${ROOT}/build/br2-external" O="${BUILD_DIR}" qemu_x86_64_defconfig

append_config() {
  local key="$1"
  local value="$2"
  # Remove any existing assignment or "is not set" line for this symbol so a
  # stale/conflicting entry from the base defconfig can never shadow the
  # value we are requesting here.
  sed -i -E "/^${key}=|^# ${key} is not set\$/d" "${BUILD_DIR}/.config"
  echo "${key}=${value}" >> "${BUILD_DIR}/.config"
}

append_config "BR2_TOOLCHAIN_BUILDROOT_CXX" "y"
append_config "BR2_ROOTFS_DEVICE_CREATION_DYNAMIC_EUDEV" "y"
append_config "BR2_PACKAGE_PROTEA_CORE_CLI" "y"
append_config "BR2_PACKAGE_PROTEA_DESKTOP" "y"
append_config "BR2_PACKAGE_LIBGTK4" "y"
append_config "BR2_PACKAGE_LIBGTK4_WAYLAND" "y"
append_config "BR2_PACKAGE_WESTON" "y"
append_config "BR2_PACKAGE_WESTON_DEFAULT_DRM" "y"
append_config "BR2_PACKAGE_WESTON_SIMPLE_CLIENTS" "y"
append_config "BR2_PACKAGE_MESA3D" "y"
append_config "BR2_PACKAGE_MESA3D_GALLIUM_DRIVER_SWRAST" "y"
append_config "BR2_PACKAGE_MESA3D_OPENGL_EGL" "y"
append_config "BR2_PACKAGE_MESA3D_GBM" "y"
append_config "BR2_PACKAGE_CAIRO_PNG" "y"
append_config "BR2_PACKAGE_CAIRO_ZLIB" "y"
append_config "BR2_TARGET_ROOTFS_ISO9660" "y"
append_config "BR2_TARGET_ROOTFS_ISO9660_GRUB2" "y"
append_config "BR2_TARGET_ROOTFS_ISO9660_INITRD" "y"
append_config "BR2_TARGET_GRUB2" "y"
append_config "BR2_TARGET_GRUB2_I386_PC" "y"
append_config "BR2_TARGET_GRUB2_BOOT_PARTITION" "\"cd\""
append_config "BR2_TARGET_GRUB2_BUILTIN_MODULES_PC" "boot linux iso9660 ext2 normal biosdisk"
append_config "BR2_TARGET_ROOTFS_ISO9660_BOOT_MENU" "\"${ROOT}/build/qemu/grub.cfg\""
append_config "BR2_ROOTFS_OVERLAY" "\"${ROOT}/build/pc-overlay\""
append_config "BR2_ROOTFS_USERS_TABLES" "\"${ROOT}/build/protea-users-table.txt\""
append_config "BR2_LINUX_KERNEL_CONFIG_FRAGMENT_FILES" "\"${ROOT}/build/qemu/protea-linux.fragment\""

make -C "${BUILDROOT}" BR2_EXTERNAL="${ROOT}/build/br2-external" O="${BUILD_DIR}" olddefconfig

echo "Disk usage before full build:"
df -h / "${BUILD_DIR}" 2>/dev/null || true

# The last two CI attempts exhausted the runner's entire disk (145G total,
# 120G free going in) before the build finished, with no clean error - the
# runner process itself was killed by ENOSPC. Sample disk usage per
# directory throughout the build so we can see exactly what is growing and
# when, instead of only knowing the disk was full after the fact.
(
  while true; do
    sleep 90
    echo "=== disk snapshot $(date -u +%H:%M:%S) ==="
    df -h / 2>/dev/null
    du -sh "${BUILD_DIR}"/build "${BUILD_DIR}"/host "${BUILD_DIR}"/target \
      "${BUILD_DIR}"/images "${ROOT}/.cache" 2>/dev/null
  done
) &
DISK_MONITOR_PID=$!
trap 'kill "${DISK_MONITOR_PID}" 2>/dev/null || true' EXIT

make -C "${BUILDROOT}" BR2_EXTERNAL="${ROOT}/build/br2-external" O="${BUILD_DIR}"

kill "${DISK_MONITOR_PID}" 2>/dev/null || true
trap - EXIT

echo "Build completed."
echo "Images: ${BUILD_DIR}/images"
