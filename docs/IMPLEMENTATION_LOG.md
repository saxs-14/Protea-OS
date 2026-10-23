# Implementation log

## Baseline inspected

- Local branch: `main`
- Local HEAD: `5ad303f` (`Add periodic per-directory disk usage monitor during the build`)
- Remote branch: `origin/main`
- Remote HEAD at time of inspection: `5ad303f`
- Baseline session checkpoint recorded by the host tooling: `0a6f07a`

## Remote workflow history inspected

The workflow history for `saxs-14/Protea-OS` shows a chain of repeated failures in the `Protea PC Image` workflow, beginning with the framework-level Buildroot package/configuration issues and ending in runner disk exhaustion.

Key workflow sequence:

- `Fix installed binary name mismatch: protea-core-cli -> protea-core` — successful workflow for the core/PC path
- `Add missing Cargo.lock files required by Buildroot's --locked cargo build` — followed by image failures
- `Fix duplicate --release flag breaking Buildroot cargo packages` — image build failed thereafter
- `Harden append_config against stale config lines; add diagnostic` — config fix attempted
- `Expand build-pc.sh diagnostic to include toolchain capability flags` — more diagnostics added
- `Dump full resolved .config instead of guessing symbol names` — continued debugging
- `Enable C++ support and eudev: real root cause of missing graphical stack` — real config root cause identified
- `Fix invalid Mesa Gallium driver name: softpipe -> swrast` — package names corrected
- `Enable cairo PDF/PS/XML and PNG/SVG surfaces for GTK4's print backend` — more required graphics support added
- `Raise PC Image CI timeout to 120 minutes` — attempted to absorb Buildroot time
- `Free GitHub runner disk space before the Buildroot build` — addressed runner space issues
- `Add periodic per-directory disk usage monitor during the build` — diagnosis and reporting, still failing in the latest run

This shows a real and iterative engineering path: the project did not stop at an initial failure; it identified the actual Buildroot package dependencies and then addressed runner resource pressure. The remaining state is still `in progress`, not `complete`.

## Source evidence of the main build issues

The Buildroot configuration file `build/build-pc.sh` is the key evidence for the project’s known image breakage and fixes. It contains the package enablement for:

- `BR2_PACKAGE_PROTEA_CORE_CLI=y`
- `BR2_PACKAGE_PROTEA_DESKTOP=y`
- `BR2_PACKAGE_LIBGTK4=y`
- `BR2_PACKAGE_LIBGTK4_WAYLAND=y`
- `BR2_PACKAGE_WESTON=y`
- `BR2_PACKAGE_MESA3D=y`
- `BR2_PACKAGE_CAIRO_PNG=y`
- `BR2_PACKAGE_CAIRO_ZLIB=y`
- `BR2_ROOTFS_OVERLAY=...`

The relevant configuration and overlay files support a graphical stack for the Protea PC image, which was missing in earlier failing iterations.

## Current project state

- Shared contracts: implemented and documented.
- PC shell: partially implemented and documented.
- PC image path: still not proven bootable.
- Phone/watch/TV: still out of scope for the current milestone.

## Highest-priority remaining tasks

1. Prove the image build end-to-end on a machine with the supported Linux toolchain and enough disk space.
2. Validate `./build/validate-pc-image.sh` on the generated rootfs before considering the build stable.
3. Boot the image in QEMU and confirm the Protea readiness marker appears before claiming the PC milestone is complete.
