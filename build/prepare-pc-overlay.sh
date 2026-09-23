#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OVERLAY="${ROOT}/build/pc-overlay"
BIN_DIR="${OVERLAY}/usr/bin"
INIT_DIR="${OVERLAY}/etc/init.d"

mkdir -p "${BIN_DIR}" "${INIT_DIR}"

cargo build --manifest-path "${ROOT}/core/Cargo.toml" --release -p protea-core-cli
install -m 0755 "${ROOT}/target/release/protea-core-cli" "${BIN_DIR}/protea-core"

cat > "${INIT_DIR}/S99protea" <<'EOF'
#!/bin/sh
case "$1" in
  start)
    echo "Starting Protea core..."
    mkdir -p /var/lib/protea
    export PROTEA_STATE_FILE=/var/lib/protea/state
    /usr/bin/protea-core status || true
    ;;
  stop)
    ;;
esac
exit 0
EOF

chmod 0755 "${INIT_DIR}/S99protea"
