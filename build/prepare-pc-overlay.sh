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
    PROTEA_HOME=/home/protea
    PROTEA_STATE_DIR="$PROTEA_HOME/.local/share/protea"
    PROTEA_STATE_FILE="$PROTEA_STATE_DIR/state"

    if ! id protea >/dev/null 2>&1; then
      echo "Protea validation error: unprivileged protea user is missing."
      exit 1
    fi

    mkdir -p "$PROTEA_STATE_DIR"
    chown -R protea:protea "$PROTEA_HOME"
    chmod 700 "$PROTEA_HOME" "$PROTEA_HOME/.local" "$PROTEA_STATE_DIR"

    if [ ! -x /usr/bin/protea-core ]; then
      echo "Protea validation error: /usr/bin/protea-core is missing."
      exit 1
    fi

    if ! su -s /bin/sh protea -c "PROTEA_STATE_FILE='$PROTEA_STATE_FILE' /usr/bin/protea-core status"; then
      echo "Protea validation error: core status failed."
      exit 1
    fi

    if [ ! -s "$PROTEA_STATE_FILE" ]; then
      echo "Protea validation error: persistent state was not created."
      exit 1
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
        chmod 666 "$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY"
        su -s /bin/sh protea -c "HOME='$PROTEA_HOME' XDG_RUNTIME_DIR='$XDG_RUNTIME_DIR' WAYLAND_DISPLAY='$WAYLAND_DISPLAY' PROTEA_STATE_FILE='$PROTEA_STATE_FILE' /usr/bin/protea-desktop" &
        echo "PROTEA_GRAPHICS_OK"
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
