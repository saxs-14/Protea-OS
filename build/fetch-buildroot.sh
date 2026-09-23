#!/usr/bin/env bash
set -euo pipefail

VERSION="2025.02.18"
ARCHIVE="buildroot-${VERSION}.tar.xz"
URL="https://buildroot.org/downloads/${ARCHIVE}"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CACHE="${ROOT}/.cache"
SRC="${CACHE}/buildroot-${VERSION}"

mkdir -p "${CACHE}"

if [ ! -d "${SRC}" ]; then
  if [ ! -f "${CACHE}/${ARCHIVE}" ]; then
    curl -fL "${URL}" -o "${CACHE}/${ARCHIVE}"
  fi
  tar -xf "${CACHE}/${ARCHIVE}" -C "${CACHE}"
fi

echo "${SRC}"
