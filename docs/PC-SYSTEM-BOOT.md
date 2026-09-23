# Protea OS PC System Boot

This document defines the real PC boot path currently being assembled.

## Boot chain

1. Firmware/bootloader loads the Linux kernel.
2. Linux initializes DRM, virtio-GPU, input and device management.
3. Buildroot starts the base userspace.
4. `S99protea` creates protected Protea state storage.
5. `protea-core` loads or creates the local Protea state.
6. If DRM and the graphical components are available, Weston starts as the Wayland compositor.
7. `protea-desktop` starts as the Protea desktop shell.
8. The shell reads the shared Protea state and exposes Gaming/Office mode and application launching.

## QEMU target

The automated target uses:

- x86_64 Buildroot
- 1024 MB RAM
- 2 virtual CPUs
- virtio block storage
- virtio-GPU
- Linux DRM/KMS
- Mesa software rendering
- Weston
- GTK4

QEMU is a validation environment, not a claim that all physical GPUs are supported.

## Current limitation

The GitHub Actions build must prove the complete image builds and reaches userspace. A successful userspace boot does not by itself prove graphical rendering, keyboard/mouse input, audio, networking, power management, or physical-hardware compatibility.

## Next validation gates

- Confirm the image builds with the Protea core and desktop packages.
- Confirm QEMU reaches Buildroot userspace.
- Confirm Weston creates a Wayland socket.
- Confirm the Protea desktop process starts.
- Add automated checks for the Protea state file.
- Validate keyboard/mouse and display on QEMU with a graphical run.
- Validate on a genuinely low-spec physical PC.
