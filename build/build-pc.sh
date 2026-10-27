#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="${ROOT}/output/pc"
BUILDROOT="$("${ROOT}/build/fetch-buildroot.sh")"

mkdir -p "${BUILD_DIR}"
rm -rf "${BUILD_DIR}/build" "${BUILD_DIR}/host" "${BUILD_DIR}/target"
"${ROOT}/build/prepare-pc-overlay.sh"

# Pre-vendor Cargo dependencies for offline Buildroot package builds
if command -v cargo >/dev/null 2>&1; then
  echo "Vendoring Cargo dependencies for protea-desktop..."
  (
    cd "${ROOT}/pc"
    mkdir -p .cargo protea-shell/.cargo
    cp -f Cargo.lock protea-shell/Cargo.lock 2>/dev/null || true
    cargo vendor vendor > .cargo/config.toml
    cat > protea-shell/.cargo/config.toml <<'EOF'
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "../vendor"
EOF
  )
fi

make -C "${BUILDROOT}" BR2_EXTERNAL="${ROOT}/build/br2-external" O="${BUILD_DIR}" BR2_DL_DIR="${ROOT}/.cache/dl" qemu_x86_64_defconfig

append_config() {
  local key="$1"
  local value="$2"
  # Remove any existing assignment or "is not set" line for this symbol so a
  # stale/conflicting entry from the base defconfig can never shadow the
  # value we are requesting here.
  sed -i -E "/^${key}=|^# ${key} is not set\$/d" "${BUILD_DIR}/.config"
  echo "${key}=${value}" >> "${BUILD_DIR}/.config"
}

append_config "BR2_TOOLCHAIN_EXTERNAL" "y"
append_config "BR2_TOOLCHAIN_EXTERNAL_BOOTLIN" "y"
append_config "BR2_TOOLCHAIN_EXTERNAL_BOOTLIN_X86_64_GLIBC_STABLE" "y"
append_config "BR2_ROOTFS_DEVICE_CREATION_DYNAMIC_EUDEV" "y"
append_config "BR2_PACKAGE_PROTEA_CORE_CLI" "y"
append_config "BR2_PACKAGE_PROTEA_DESKTOP" "y"
append_config "BR2_PACKAGE_LIBGTK4" "y"
append_config "BR2_PACKAGE_LIBGTK4_WAYLAND" "y"
append_config "BR2_PACKAGE_WESTON" "y"
append_config "BR2_PACKAGE_WESTON_DEFAULT_DRM" "y"
append_config "BR2_PACKAGE_WESTON_SIMPLE_CLIENTS" "y"
# BR2_PACKAGE_MESA3D_GALLIUM_DRIVER_SWRAST does not exist in current
# Buildroot (verified against buildroot/buildroot master package/mesa3d/
# Config.in - the real Gallium driver symbols are CROCUS/ETNAVIV/.../
# LLVMPIPE/.../SOFTPIPE/..., no "SWRAST" entry anywhere). Kconfig silently
# drops an unrecognized symbol during olddefconfig with no error, so this
# line has been a complete no-op this whole time: no Gallium driver was
# ever enabled, which forced BR2_PACKAGE_MESA3D_OPENGL_EGL and
# BR2_PACKAGE_MESA3D_GBM off too (both "depends on" the Gallium-driver
# bool, which only a real per-driver symbol can select) - exactly matching
# the observed "could not open zink: zink_dri.so" / "No EGL configuration
# available" messages and GTK 4.14's removed no-GL Cairo fallback crashing
# on an unchecked oversized buffer. The real llvmpipe (software Gallium)
# driver symbol also needs its own LLVM toggle turned on explicitly
# (LLVMPIPE "depends on" MESA3D_LLVM, it does not select it).
#
# This is the client-side half of the fix; Weston itself also needs the
# same EGL/GBM/GLES trio present (see weston.mk: gl-renderer is compiled
# in only when BR2_PACKAGE_HAS_LIBEGL + HAS_LIBGBM + HAS_LIBGLES are ALL
# provided) or its own drm-backend silently falls back to the pixman
# renderer, which has zero dma-buf/GL buffer-sharing support - so a
# working client renderer would still have nothing to hand buffers to.
append_config "BR2_PACKAGE_MESA3D" "y"
append_config "BR2_PACKAGE_MESA3D_LLVM" "y"
append_config "BR2_PACKAGE_MESA3D_GALLIUM_DRIVER_LLVMPIPE" "y"
append_config "BR2_PACKAGE_MESA3D_OPENGL_EGL" "y"
append_config "BR2_PACKAGE_MESA3D_OPENGL_ES" "y"
append_config "BR2_PACKAGE_MESA3D_GBM" "y"
append_config "BR2_PACKAGE_WAYLAND" "y"
append_config "BR2_PACKAGE_CAIRO_PNG" "y"
append_config "BR2_PACKAGE_CAIRO_ZLIB" "y"
# GTK4's CSS "background-image: url(...)" (used for the desktop wallpaper)
# decodes images via gdk-pixbuf, whose JPEG/PNG loaders are each gated
# behind a separate codec package (confirmed in gdk-pixbuf.mk: jpeg support
# is "-Djpeg=enabled" only if BR2_PACKAGE_JPEG=y, same for libpng). Neither
# was enabled, even though most bundled wallpapers are .jpg - a failed/
# garbage image decode in that path is a strong candidate for the "too big"
# Cairo surface crash protea-desktop has been hitting on first paint.
append_config "BR2_PACKAGE_GDK_PIXBUF" "y"
append_config "BR2_PACKAGE_JPEG" "y"
append_config "BR2_PACKAGE_LIBPNG" "y"
append_config "BR2_TARGET_ROOTFS_ISO9660" "y"
append_config "BR2_TARGET_ROOTFS_ISO9660_GRUB2" "y"
append_config "BR2_TARGET_ROOTFS_ISO9660_INITRD" "y"
# GTK4 + Weston + Mesa push the rootfs well past the 60M default; use 512M
append_config "BR2_TARGET_ROOTFS_EXT2_SIZE" "\"512M\""
append_config "BR2_TARGET_GRUB2" "y"
append_config "BR2_TARGET_GRUB2_I386_PC" "y"
append_config "BR2_TARGET_GRUB2_BOOT_PARTITION" "\"cd\""
append_config "BR2_TARGET_GRUB2_BUILTIN_MODULES_PC" "\"boot linux iso9660 ext2 fat squash4 part_msdos part_gpt normal biosdisk\""
append_config "BR2_TARGET_ROOTFS_ISO9660_BOOT_MENU" "\"${ROOT}/build/qemu/grub.cfg\""
append_config "BR2_ROOTFS_OVERLAY" "\"${ROOT}/build/pc-overlay\""
append_config "BR2_ROOTFS_USERS_TABLES" "\"${ROOT}/build/protea-users-table.txt\""
append_config "BR2_LINUX_KERNEL_CONFIG_FRAGMENT_FILES" "\"${ROOT}/build/qemu/protea-linux.fragment\""

make -C "${BUILDROOT}" BR2_EXTERNAL="${ROOT}/build/br2-external" O="${BUILD_DIR}" BR2_DL_DIR="${ROOT}/.cache/dl" olddefconfig

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

make -C "${BUILDROOT}" BR2_EXTERNAL="${ROOT}/build/br2-external" O="${BUILD_DIR}" BR2_DL_DIR="${ROOT}/.cache/dl"

kill "${DISK_MONITOR_PID}" 2>/dev/null || true
trap - EXIT

echo "Build completed."
echo "Images: ${BUILD_DIR}/images"
