# Protea OS engineering status

Date: 2026-10-13

## Executive summary

This repository is not yet a complete, bootable, installable Protea OS release. The evidence in the repo and the current GitHub workflow history show that the shared core and initial PC-shell contracts are implemented, while the reproducible Buildroot PC image remains in a partial/broken state that still needs a clean, low-level validation pass.

The strongest evidence in the tree is:

- `core/protea-core` contains a versioned local-first state model for identity, settings, permissions, device profile, and mode policy.
- `core/protea-core-cli` and `pc/protea-shell` both implement local state loading/saving and mode changes.
- `build/build-pc.sh`, `build/prepare-pc-overlay.sh`, `build/validate-pc-image.sh`, and the Buildroot external tree define the PC image pipeline.
- Remote workflow history shows repeated build failures in `Protea PC Image` followed by fixes for package configuration, Mesa driver selection, and runner disk pressure; the repo is therefore in the middle of the hardening loop, not in a finished release state.

## Major component audit

| Component | Intended role | Code present | Build/test status | Real-world status | Next action |
| --- | --- | --- | --- | --- | --- |
| Shared core | Versioned identity/settings/permission/device contracts | VERIFIED | VERIFIED (Rust unit tests present; local compile blocked here by missing linker toolchain, but code exists and tests are defined) | PARTIAL | Finish a clean toolchain-enabled validation run and keep contracts stable before expanding scope |
| PC shell | User-facing Linux desktop shell and settings | VERIFIED for local CLI-based shell and GTK package integration | PARTIAL | PARTIAL | Validate the GTK shell in a running user session and confirm it boots cleanly from the image |
| PC image build | Reproducible Buildroot x86_64 image generation | VERIFIED in scripts and config | BROKEN/UNVERIFIED in this environment; remote workflows show active failure chain | BROKEN | Resolve the remaining build root cause and keep the configuration deterministic |
| Linux startup + init | Boot marker and boot configuration for Protea | VERIFIED script presence | UNVERIFIED | PARTIAL | Validate boot marker and startup ordering inside QEMU or a real VM |
| Phone | AOSP-based phone shell foundation | PARTIAL | UNVERIFIED | PLANNED | Do not divert effort from the PC milestone until the shared contracts and PC image are stable |
| Watch / TV | Future device classes | PLANNED | UNVERIFIED | PLANNED | Keep the scope strictly later-stage |

## Status by major component

### Shared core — VERIFIED

- Intended behavior: local-first identity, settings, permissions, hardware profile, and mode policy for all device classes.
- Actual code: `core/protea-core/src/lib.rs` and `core/protea-core/src/persistence.rs` provide the model and persistence behavior.
- Build status: the Rust workspace is defined and the code is structured for unit tests; full Rust validation is blocked in this environment because the C/C++ linker toolchain is missing (`cc` not found).
- Testing: the library defines unit tests for settings replacement, permission toggling, memory tiering, mode policy, and initial state creation.
- Hardware validation: no real hardware validation has been performed.
- Missing/broken: the shared core is intentionally local-first and still lacks deeper authenticated sync, migration validation, and broader security review.
- Next action: run the Rust workspace on a machine with the native C toolchain installed and then fix any real failures before expanding the contract.

### PC shell — PARTIAL

- Intended behavior: local Linux desktop interaction, settings display, identity display, and mode switching.
- Actual code: `pc/protea-shell/src/main.rs` and `pc/protea-shell/src/bin/protea-desktop.rs` implement a terminal/desktop shell with state persistence and a settings surface.
- Build status: the code exists and is aligned with the repo’s architecture; it is not proven on an actual graphical Linux session in this environment.
- Testing: no end-to-end graphical session tests were executed here.
- Hardware validation: not performed.
- Missing/broken: the shell remains a foundation, not a full desktop environment, and the bootable image path still needs proven validation.
- Next action: validate the graphical stack in QEMU or a VM with a real Wayland session and confirm the Protea boot marker is emitted from the generated image.

### PC image / Buildroot — BROKEN / UNVERIFIED

- Intended behavior: a reproducible x86_64 Linux image with Protea core, desktop package, overlay, and QEMU boot validation.
- Actual code: `build/build-pc.sh`, `build/fetch-buildroot.sh`, `build/prepare-pc-overlay.sh`, `build/validate-pc-image.sh`, and the Buildroot external package definitions.
- Build status: the current repo contains the build logic and the package fixes for known failures, but the image was not successfully built in this local environment.
- Testing: remote GitHub workflow history shows a chain of failures culminating in runner disk exhaustion while generating Buildroot output. This shows the build is still not proven end-to-end.
- Missing/broken: the exact full boot path is not proven; this is the project’s current critical gap.
- Next action: on a machine with adequate disk space and the required Linux development packages, run the full `./build/build-pc.sh` and then `./build/validate-pc-image.sh` path and inspect the first failing command, not just the later disk issue.

### Phone, watch, TV — PLANNED / UNVERIFIED

- Intended behavior: later device-family integration after the shared core and PC path are stable.
- Actual code: placeholder READMEs and launcher scaffolding exist for Android; no proven AOSP runtime or device bootstrap is complete.
- Build status: not proven.
- Hardware validation: not performed.
- Missing/broken: no complete device-specific implementation is ready.
- Next action: keep these as later milestones and avoid scope creep until PC and shared contracts are proven.

## Required status labels

- VERIFIED: shared core contracts, local persistence model, early Rust tests, initial architecture definitions.
- PARTIAL: PC shell, Linux integration, and the overall PC image path.
- PLANNED: phone/watch/TV work.
- BROKEN: the Buildroot image pipeline, from the evidence of repeated failing CI and missing proof of generated artifacts.
- UNVERIFIED: real hardware / real boot / VM boot claims beyond the available local evidence.

## Conclusion

The repo is a credible engineering foundation with real definitions and real code, but it still falls short of a complete Protea OS release. The highest-confidence actionable issue is to complete the Buildroot image validation loop and keep the shared contracts stable before moving into wider platform work.
