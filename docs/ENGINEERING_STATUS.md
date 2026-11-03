# Protea OS engineering status

Date: 2026-11-03

## Executive summary

Protea OS is an active development foundation, not a finished operating-system release. The latest `main` commit (`6fde747`, 2026-11-01) has a successful PC image workflow: Buildroot produced the expected images, image-content validation passed, the PC image booted in QEMU through the graphical-window-presented marker, and the installation ISO reached the system-readiness marker. This verifies the automated CI paths, not human interaction, physical hardware compatibility, installation to disk, or recovery.

The previous status that called the Buildroot image pipeline broken is obsolete. The earlier failures remain useful history, but the latest successful run supersedes that assessment.

## Component status

| Component | Evidence found | Current status | Remaining work | Dependencies | Next action |
| --- | --- | --- | --- | --- | --- |
| Shared Rust core | Core crate, versioned state and persistence; Core CI | Working and verified in CI | Broader migration and permission/security validation | Rust toolchain | Keep contracts stable; extend tests with concrete use cases |
| PC shell | GTK4 shell; PC CI; QEMU CI detects graphical readiness and window presentation | Implemented; automated startup verified | Human interaction, accessibility, lifecycle and app-launch validation | Linux graphical runtime; QEMU or hardware | Run an interactive desktop session and exercise controls |
| PC image | Buildroot external tree and latest successful image workflow | Working and verified in CI | Repeatable local download/runtime testing; installation and recovery | Linux Buildroot toolchain for local rebuild | Boot the published ISO in QEMU and VirtualBox; record results |
| Boot and session startup | Init/overlay configuration; CI QEMU boot markers | Working and verified in CI | Input and interactive session behavior | QEMU or supported hardware | Validate keyboard/mouse interaction and session persistence |
| Android launcher | Launcher implementation and successful Phone CI build | Implemented; build verified | AOSP integration and device/emulator runtime verification | Android/AOSP environment | Defer until PC interactive validation is complete |
| Watch and TV | Project direction and placeholder documentation | Planned, not implemented | Platform-specific designs and implementations | Stable PC/phone foundation | Keep deferred |
| CHPC environment | No cluster inventory or authorised connection evidence inspected | Blocked / not assessed | Establish approved access and verify candidates before use | Authorised SSH/config and cluster policy | Do not connect or deploy until access is provided and verified |

## Verified evidence

- Latest `Protea PC Image` run: GitHub Actions run `41356380128`, commit `6fde747`, completed successfully on 2026-11-01.
- CI steps passed: Buildroot image build, expected artifact checks, image validation, QEMU PC-image boot, and QEMU installation-ISO boot.
- The PC-image QEMU check requires `PROTEA_BOOT_OK`, `PROTEA_GRAPHICS_OK`, and `PROTEA_DESKTOP_WINDOW_PRESENTED`, and rejects known desktop crash signatures.
- The ISO QEMU check requires `PROTEA_BOOT_OK`; it rejects graphics crash signatures when graphics readiness is reported, but does not require the desktop-window-presented marker.
- The latest recorded Core, PC-shell, and Phone CI runs passed on 2026-10-23, 2026-10-31, and 2026-10-23 respectively.
- No physical-device test or human-driven desktop interaction is evidenced by these checks.

## Highest-priority remaining work

1. Perform a human-driven desktop session test in QEMU: inspect the visible UI, interact with keyboard and mouse, launch an app, exercise settings/mode controls, and check the shell remains stable.
2. Verify whether settings and identity persist across a reboot using the intended writable storage path.
3. Validate input, networking, accessibility, shutdown/restart, and failure recovery in the guest.
4. Plan installation and recovery workflows only after the interactive runtime path is reliable.

## Limitations

- CI boot markers demonstrate automated startup, not a complete or production-ready OS.
- No physical PC, phone, watch, or TV compatibility has been established.
- The Android launcher is not a complete AOSP-based Protea phone system.
- Secure updates, verified boot, recovery, cross-device synchronization, and a release process remain future work.
- CHPC access and candidate occupancy have not been checked.
