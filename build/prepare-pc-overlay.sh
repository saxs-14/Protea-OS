#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OVERLAY="${ROOT}/build/pc-overlay"

mkdir -p "${OVERLAY}/etc/init.d"
mkdir -p "${OVERLAY}/etc/xdg/weston"

# Weston's desktop-shell plugin draws its own default top panel with
# generic placeholder launcher icons and a built-in idle-lock screen -
# both are separate from protea-desktop and were showing through as a
# stray, unbranded bar with broken-looking icons. protea-desktop is the
# only shell UI Protea should show, and it requests fullscreen itself,
# so disable Weston's own panel/lock entirely.
cat > "${OVERLAY}/etc/xdg/weston/weston.ini" <<'EOF'
[shell]
panel-position=none
locking=false
background-color=0xFF17120F
EOF

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
      weston --backend=drm-backend.so --log=/var/log/weston.log &
      # Do not assume the compositor binds "wayland-0": libseat's embedded
      # seatd launcher (used here since there is no systemd-logind or
      # standalone seatd) forks weston internally, and the resulting socket
      # has been observed to land on "wayland-1" instead. Detect whichever
      # wayland-<N> socket actually appears rather than waiting on a fixed name.
      i=0
      WAYLAND_DISPLAY=""
      while [ -z "$WAYLAND_DISPLAY" ] && [ "$i" -lt 300 ]; do
        for candidate in "$XDG_RUNTIME_DIR"/wayland-*; do
          case "$candidate" in
            *.lock) continue ;;
          esac
          if [ -S "$candidate" ]; then
            WAYLAND_DISPLAY="${candidate##*/}"
            break
          fi
        done
        [ -n "$WAYLAND_DISPLAY" ] && break
        sleep 0.1
        i=$((i + 1))
      done
      if [ -n "$WAYLAND_DISPLAY" ]; then
        export WAYLAND_DISPLAY
        # weston runs as root (it needs DRM/input device access via
        # libseat), so the socket it creates is root:root 0755. The
        # unprivileged protea user has only r-x ("other") on that file,
        # and connect(2) on a Unix domain socket requires write
        # permission - without this, protea-desktop's wl_display_connect
        # fails with EACCES and GTK silently falls back to a (nonexistent)
        # X11 display. Open up the socket so the protea-user client can
        # actually connect to the root-owned compositor.
        chmod 777 "$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY" 2>/dev/null || true
        # GSK_RENDERER=cairo: this image has no EGL configuration at all
        # (no GPU driver, no software GL), so GSK's default behaviour -
        # try the NGL renderer, fail, try the GL renderer, fail, then fall
        # back to Cairo - leaves the renderer in a broken half-initialised
        # state. That is the actual cause of the repeated "g_object_ref:
        # assertion G_IS_OBJECT (object) failed" criticals and the fatal
        # Wayland shared-memory-buffer crash a few hundred ms after the
        # window first presents. Forcing Cairo directly skips the failed
        # GL attempts and initialises it cleanly.
        su -s /bin/sh protea -c "HOME='$PROTEA_HOME' XDG_RUNTIME_DIR='$XDG_RUNTIME_DIR' WAYLAND_DISPLAY='$WAYLAND_DISPLAY' PROTEA_STATE_FILE='$PROTEA_STATE_FILE' GDK_BACKEND=wayland GSK_RENDERER=cairo /usr/bin/protea-desktop" &
        echo "PROTEA_GRAPHICS_OK (wayland display: $WAYLAND_DISPLAY)"
      else
        echo "Protea: Wayland compositor did not start; continuing headless."
        echo "---- weston.log ----"
        cat /var/log/weston.log 2>&1 || echo "(no weston.log)"
        echo "---------------------"
        echo "---- diagnostics ----"
        echo "free:"
        free -m 2>&1 || echo "(no free)"
        echo "ps:"
        ps aux 2>&1 || echo "(no ps)"
        echo "runtime dir listing ($XDG_RUNTIME_DIR):"
        ls -la "$XDG_RUNTIME_DIR" 2>&1 || echo "(cannot list runtime dir)"
        echo "dmesg tail:"
        dmesg 2>&1 | tail -60 || echo "(no dmesg)"
        echo "----------------------"
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
