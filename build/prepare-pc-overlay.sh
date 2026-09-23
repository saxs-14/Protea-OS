#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OVERLAY="${ROOT}/build/pc-overlay"

mkdir -p "${OVERLAY}/etc/init.d"

cat > "${OVERLAY}/etc/init.d/S99protea" <<'EOF'
#!/bin/sh
case "$1" in
  start)
    echo "Starting Protea core..."
    mkdir -p /var/lib/protea
    chmod 700 /var/lib/protea
    export PROTEA_STATE_FILE=/var/lib/protea/state
    if [ -x /usr/bin/protea-core ]; then
      /usr/bin/protea-core status || true
    fi

    if [ -x /usr/bin/weston ] && [ -x /usr/bin/protea-desktop ] && [ -e /dev/dri/card0 ]; then
      echo "Starting Protea graphical session..."
      mkdir -p /run/user/0
      chmod 700 /run/user/0
      export XDG_RUNTIME_DIR=/run/user/0
      export WAYLAND_DISPLAY=wayland-0
      weston --backend=drm-backend.so --tty=1 --log=/var/log/weston.log &
      i=0
      while [ ! -S "$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY" ] && [ "$i" -lt 50 ]; do
        sleep 0.1
        i=$((i + 1))
      done
      if [ -S "$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY" ]; then
        /usr/bin/protea-desktop &
      else
        echo "Protea: Wayland compositor did not start; continuing headless."
      fi
    else
      echo "Protea: graphical session unavailable; continuing with core services."
    fi
    touch /var/run/protea-boot-ok
    echo "PROTEA_BOOT_OK"
    ;;
  stop)
    killall protea-desktop 2>/dev/null || true
    killall weston 2>/dev/null || true
    ;;
esac
exit 0
EOF

chmod 0755 "${OVERLAY}/etc/init.d/S99protea"
