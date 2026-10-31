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
# CORRECTION: an earlier version of this comment claimed
# BR2_PACKAGE_MESA3D_GALLIUM_DRIVER_SWRAST does not exist and tried
# BR2_PACKAGE_MESA3D_GALLIUM_DRIVER_LLVMPIPE instead - that was checked
# against buildroot/buildroot's bleeding-edge "master" branch, not the
# actual pinned release this build uses (see build/fetch-buildroot.sh:
# VERSION="2025.02.18"). Re-verified directly against the real
# buildroot/buildroot tag 2025.02.18 package/mesa3d/Config.in: SWRAST
# does exist there and correctly selects BR2_PACKAGE_MESA3D_GALLIUM_
# DRIVER; LLVMPIPE does not exist at all in this release (only in a
# later one), so that line was itself exactly the kind of silently-
# dropped-by-olddefconfig no-op it was trying to fix - and worse, since
# it left NO Gallium driver selected at all, which is what actually
# broke the build this time (protea-desktop/libgtk4 never got built:
# BR2_PACKAGE_LIBGTK4 depends on BR2_PACKAGE_HAS_LIBEGL, which Mesa only
# provides once a real Gallium driver is selected).
#
# What's still correct and kept: BR2_PACKAGE_MESA3D_OPENGL_ES (verified
# present in 2025.02.18 too) and an explicit BR2_PACKAGE_WAYLAND, since
# Weston's own gl-renderer is only compiled in when Mesa provides
# EGL+GBM+GLES together (weston.mk) - without gl-renderer, Weston's
# drm-backend silently falls back to pixman, which has no dma-buf/GL
# buffer-sharing support at all, so even a working GL client would have
# had nothing to hand buffers to. weston.ini and the weston launch line
# below now force renderer=gl explicitly for the same reason: so any
# future failure here is a loud, diagnosable Weston error instead of a
# silent fallback that looks fine but breaks every GL client.
append_config "BR2_PACKAGE_MESA3D" "y"
append_config "BR2_PACKAGE_MESA3D_GALLIUM_DRIVER_SWRAST" "y"
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
# ROOT CAUSE of the protea-desktop segfault, finally resolved via a CI
# diagnostic step that disassembled the real built libcairo.so.2.11804.4
# directly (nm against three independent copies - host sysroot, the
# per-package build dir, and the stripped target copy's dynamic symbol
# table - all agreed): the crash is inside cairo_scaled_font_get_font_
# options(), called by Pango/PangoCairo when laying out TEXT. This image
# had zero font packages at all - no BR2_PACKAGE_DEJAVU, no explicit
# fontconfig - so Pango had no real font to shape text with. That
# explains every observation this session: content-independent (every
# version of the desktop shows some text label), timing-consistent
# (Pango lazily resolves/queries fonts on the first real text layout
# pass, not at window-present time), and unaffected by every renderer/
# wallpaper/decoration change tried, because none of those touch font
# loading at all.
append_config "BR2_PACKAGE_DEJAVU" "y"
append_config "BR2_PACKAGE_FONTCONFIG" "y"
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
