#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OVERLAY="${ROOT}/build/pc-overlay"
INIT_DIR="${OVERLAY}/etc/init.d"

mkdir -p "${INIT_DIR}"

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
