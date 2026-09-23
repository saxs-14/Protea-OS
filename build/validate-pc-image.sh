#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="${ROOT}/output/pc"
IMAGES="${BUILD_DIR}/images"

fail() { echo "VALIDATION FAILED: $*" >&2; exit 1; }
ok() { echo "VALIDATION OK: $*"; }

for f in "${IMAGES}/bzImage" "${IMAGES}/rootfs.ext2" "${IMAGES}/rootfs.iso9660"; do
  [[ -s "$f" ]] || fail "missing or empty $f"
done
ok "all expected boot artifacts exist"

command -v file >/dev/null || fail "file command is required"
file "${IMAGES}/bzImage" | grep -Eq 'Linux kernel|boot executable' || fail "bzImage is not recognised as a kernel image"
file "${IMAGES}/rootfs.ext2" | grep -qi 'filesystem' || fail "rootfs.ext2 is not recognised as a filesystem"
file "${IMAGES}/rootfs.iso9660" | grep -qi 'ISO 9660' || fail "rootfs.iso9660 is not recognised as an ISO image"
ok "image formats are recognised"

if command -v debugfs >/dev/null; then
  for path in /usr/bin/protea-core /usr/bin/protea-desktop /etc/init.d/S99protea; do
    debugfs -R "stat $path" "${IMAGES}/rootfs.ext2" 2>/dev/null | grep -q 'Inode:' || fail "rootfs.ext2 does not contain $path"
  done
  debugfs -R "cat /etc/passwd" "${IMAGES}/rootfs.ext2" 2>/dev/null | grep -q '^protea:' || fail "rootfs.ext2 does not contain the unprivileged protea account"
  debugfs -R "cat /etc/init.d/S99protea" "${IMAGES}/rootfs.ext2" 2>/dev/null | grep -q 'su -s /bin/sh protea' || fail "graphical session is not configured to drop privileges"
  ok "core, desktop, boot init and unprivileged session configuration are present in rootfs"
else
  echo "VALIDATION WARNING: debugfs unavailable; rootfs file-presence validation skipped"
fi

grep -q 'BR2_PACKAGE_PROTEA_CORE_CLI=y' "${BUILD_DIR}/.config" || fail "core package is not enabled"
grep -q 'BR2_PACKAGE_PROTEA_DESKTOP=y' "${BUILD_DIR}/.config" || fail "desktop package is not enabled"
grep -q 'BR2_TARGET_ROOTFS_ISO9660=y' "${BUILD_DIR}/.config" || fail "ISO output is not enabled"
ok "Buildroot configuration contains required Protea targets"

echo "Protea PC artifact validation passed."
