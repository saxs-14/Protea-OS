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
      PROTEA_UID="$(id -u protea)"
      PROTEA_RUNTIME_DIR="/run/user/$PROTEA_UID"
      mkdir -p "$PROTEA_RUNTIME_DIR"
      chown protea:protea "$PROTEA_RUNTIME_DIR"
      chmod 700 "$PROTEA_RUNTIME_DIR"
      export XDG_RUNTIME_DIR="$PROTEA_RUNTIME_DIR"
      export WAYLAND_DISPLAY=wayland-0
      weston --backend=drm-backend.so --log=/var/log/weston.log &
      i=0
      while [ ! -S "$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY" ] && [ "$i" -lt 300 ]; do
        sleep 0.1
        i=$((i + 1))
      done
      if [ -S "$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY" ]; then
        su -s /bin/sh protea -c "HOME='$PROTEA_HOME' XDG_RUNTIME_DIR='$XDG_RUNTIME_DIR' WAYLAND_DISPLAY='$WAYLAND_DISPLAY' PROTEA_STATE_FILE='$PROTEA_STATE_FILE' /usr/bin/protea-desktop" &
        echo "PROTEA_GRAPHICS_OK"
      else
        echo "Protea: Wayland compositor did not start; continuing headless."
        echo "---- weston.log ----"
        cat /var/log/weston.log 2>&1 || echo "(no weston.log)"
        echo "---------------------"
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

APPS_DIR="${OVERLAY}/usr/share/applications"
mkdir -p "${APPS_DIR}"

cat > "${APPS_DIR}/mmino.desktop" <<'EOF'
[Desktop Entry]
Type=Application
Name=Mmino
GenericName=Music Player
Comment=South African local music player for Protea OS
Exec=protea-desktop --app mmino
Icon=audio-player
Categories=AudioVideo;Audio;Player;
Terminal=false
EOF

cat > "${APPS_DIR}/ditlhophiso.desktop" <<'EOF'
[Desktop Entry]
Type=Application
Name=Ditlhophiso
GenericName=Settings
Comment=System settings and preferences for Protea OS
Exec=protea-desktop --app settings
Icon=preferences-system
Categories=Settings;System;
Terminal=false
EOF

cat > "${APPS_DIR}/ngwala.desktop" <<'EOF'
[Desktop Entry]
Type=Application
Name=Ngwala
GenericName=Notes
Comment=Fast offline notes and checklists
Exec=protea-desktop --app notes
Icon=accessories-text-editor
Categories=Office;Utility;
Terminal=false
EOF

cat > "${APPS_DIR}/bala.desktop" <<'EOF'
[Desktop Entry]
Type=Application
Name=Bala
GenericName=Calculator
Comment=System calculator for Protea OS
Exec=protea-desktop --app calculator
Icon=accessories-calculator
Categories=Utility;Calculator;
Terminal=false
EOF
