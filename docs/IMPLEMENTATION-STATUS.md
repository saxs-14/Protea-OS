# Protea OS — Implementation Status

This document records what is actually implemented in the repository. It deliberately separates working software from work that still requires a real machine, emulator, device, signing keys, or production infrastructure.

## Implemented

### Shared core
- Rust workspace with a standard-library-only Protea core.
- Device classes: PC, Phone, Watch, TV.
- Hardware tiers: Minimum, Recommended, Full.
- Local identity model.
- Settings and local permissions.
- Gaming and Office mode contracts.
- Versioned local state format.
- Atomic state-file replacement.
- Unix state-file permissions set to 0600 when saved.
- Unit tests for state, persistence, permissions, hardware tiers, and mode policies.

### PC shell
- GTK4 desktop executable.
- Real local state loading and saving.
- Hardware detection for RAM, root filesystem storage, DRM GPU presence, and battery presence.
- Gaming/Office mode controls.
- Best-effort Linux power-profile switching through powerprofilesctl.
- Application discovery from standard .desktop application directories.
- Application launch through discovered desktop entries.
- Taskbar/start surface foundation.
- Settings window with local identity/device/mode information.
- Restart and shutdown controls using the underlying Linux power commands.
- QEMU virtio networking with BusyBox DHCP startup support.

### Phone launcher
- Real Android Activity configured as a HOME/launcher application.
- Android API 36 build target in CI (the launcher is designed for newer Android/AOSP integration later).
- Gaming/Office mode selection.
- Local mode persistence using Android SharedPreferences.
- Discovery of installed launcher applications through PackageManager.
- Launching installed applications.

### Engineering
- GitHub Actions for core, PC, and phone builds.
- Formatting, unit tests, and Clippy enforcement for the Rust core.
- Android debug-build automation.
- Buildroot bootstrap/build scripts pinned to a reproducible LTS release.
- QEMU-first PC validation plan.
- Explicit Protea boot-readiness marker checked by CI.
- Buildroot package for the Protea GTK4 desktop shell.
- Wayland/Weston and Mesa software-rendering integration for the PC image.
- QEMU virtio-GPU and DRM kernel configuration fragment.
- Cross-device contract and architecture documentation.

## Not yet a finished operating system

The repository does **not** yet contain a complete installable Protea OS release.

The following still require implementation and/or real-world validation:

1. Verified bootable Protea PC image with the Protea session integrated.
2. Real graphical login/session startup.
3. Wayland compositor/window-management integration.
4. Full desktop shell: settings, notifications, networking, file manager, system tray, lock screen and shutdown/reboot controls.
5. Hardware-specific GPU acceleration and graphics testing.
6. Deeper Gaming/Office performance controls beyond power-profile selection.
7. Secure update and rollback system.
8. Recovery environment.
9. Verified boot/signing strategy.
10. Cross-device authenticated synchronization.
11. Real AOSP build and Protea system integration on supported phone hardware.
12. Watch implementation and hardware validation.
13. TV implementation and hardware validation.
14. Accessibility, localization, telemetry/privacy controls, crash reporting and release QA.
15. Installer/recovery media and release documentation.

## Phase order

### Phase 1 — Foundation
**Status: implemented**

Core contracts, local state, CI, initial PC shell and Android launcher exist.

### Phase 2 — Real PC system
**Status: in progress**

The repository now contains a Buildroot external tree for the Protea Rust core and GTK4 desktop shell, a Weston/Wayland graphics stack, a root filesystem init hook, a QEMU DRM kernel fragment, and an automated QEMU build/boot workflow. CI still has to prove the current image builds and boots; after that, real display/input/session validation remains.

### Phase 3 — PC hardware adaptation
**Status: in progress**

Hardware discovery exists for the PC shell. More detailed CPU/GPU/storage capability detection and real low-spec testing are still required.

### Phase 4 — Phone system integration
**Status: in progress**

The launcher is a real Android component, but it is not yet a complete Protea phone OS. AOSP integration and device testing remain.

### Phase 5 — Cross-device identity and sync
**Status: architecture defined**

The shared contract exists. Secure authenticated synchronization, conflict handling and offline-first replication are not implemented yet.

### Phase 6 — Watch and TV
**Status: planned**

The device architecture is documented. Platform-specific implementations should follow after PC and phone foundations are stable.

### Phase 7 — Security, recovery and updates
**Status: planned**

This must be completed before calling a build a production operating-system release.

## No-budget development strategy

Protea OS should continue using free/open-source development and testing wherever possible:

- Linux as the PC development base.
- QEMU before physical installation.
- Buildroot for controlled PC images.
- Android emulator/Cuttlefish before physical phone hardware.
- Existing Linux/Android graphics and driver stacks instead of writing a new kernel or compositor.
- Existing open-source system components where they reduce maintenance without changing the Protea design.
- Real low-spec hardware only when hardware validation becomes necessary.

## Definition of “done”

Protea OS should only be called a completed release after a release candidate can:

1. Boot from installation media or a supported disk image.
2. Start the Protea session automatically.
3. Persist identity and settings across reboot.
4. Launch applications.
5. Switch Gaming/Office modes and apply documented system policies.
6. Handle networking, audio, display, storage and power correctly on supported hardware.
7. Update and recover safely.
8. Protect local user data and privileged operations.
9. Run the supported phone build on a real device.
10. Pass automated tests plus documented QEMU and physical-hardware validation.

Until then, the repository is a working development foundation toward the full Protea OS.
