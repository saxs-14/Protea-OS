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

append_config "BR2_PACKAGE_PROTEA_CORE_CLI" "y"
append_config "BR2_PACKAGE_PROTEA_DESKTOP" "y"
append_config "BR2_PACKAGE_LIBGTK4" "y"
append_config "BR2_PACKAGE_LIBGTK4_WAYLAND" "y"
append_config "BR2_PACKAGE_WESTON" "y"
append_config "BR2_PACKAGE_WESTON_DEFAULT_DRM" "y"
append_config "BR2_PACKAGE_WESTON_SIMPLE_CLIENTS" "y"
append_config "BR2_PACKAGE_MESA3D" "y"
append_config "BR2_PACKAGE_MESA3D_GALLIUM_DRIVER_SOFTPIPE" "y"
append_config "BR2_PACKAGE_MESA3D_OPENGL_EGL" "y"
append_config "BR2_PACKAGE_MESA3D_GBM" "y"
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

echo "--- Protea graphical stack config check (post-olddefconfig) ---"
for sym in \
  BR2_PACKAGE_MESA3D BR2_PACKAGE_WESTON BR2_PACKAGE_LIBGTK4 \
  BR2_PACKAGE_LIBGTK4_WAYLAND BR2_PACKAGE_PROTEA_DESKTOP \
  BR2_PACKAGE_PROTEA_CORE_CLI BR2_PACKAGE_HAS_LIBEGL \
  BR2_PACKAGE_HAS_UDEV BR2_INSTALL_LIBSTDCPP BR2_STATIC_LIBS \
  BR2_TOOLCHAIN_HAS_THREADS BR2_TOOLCHAIN_HAS_THREADS_NPTL \
  BR2_TOOLCHAIN_HAS_SYNC_1 BR2_TOOLCHAIN_HAS_SYNC_4 \
  BR2_TOOLCHAIN_GCC_AT_LEAST_8 BR2_USE_WCHAR \
  BR2_TOOLCHAIN_BUILDROOT_GLIBC BR2_TOOLCHAIN_BUILDROOT_MUSL \
  BR2_TOOLCHAIN_BUILDROOT_UCLIBC; do
  grep -E "^${sym}=|^# ${sym} is not set" "${BUILD_DIR}/.config" || echo "${sym}: ABSENT FROM CONFIG (invisible)"
done
echo "--- full BR2_TOOLCHAIN_* dump ---"
grep -E "^BR2_TOOLCHAIN|^# BR2_TOOLCHAIN" "${BUILD_DIR}/.config" || true
echo "-----------------------------------------------------------------"

make -C "${BUILDROOT}" BR2_EXTERNAL="${ROOT}/build/br2-external" O="${BUILD_DIR}"

echo "Build completed."
echo "Images: ${BUILD_DIR}/images"
