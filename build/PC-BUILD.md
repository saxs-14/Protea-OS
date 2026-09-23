# Protea PC Build Plan

The PC system is built in two stages.

## Stage A — normal Linux development

Develop and test the Protea shell inside an existing Linux installation. This avoids making every UI change require a full OS image rebuild.

## Stage B — controlled bootable system

After the shell and core are stable, Buildroot will produce a controlled Linux system image.

Buildroot can generate a cross-compilation toolchain, root filesystem, Linux kernel image and bootloader for a target system. We will pin a tested release for reproducible builds.

## First target

The first boot target is x86_64 PC hardware.

Initial validation should use a virtual machine or spare machine before touching a primary computer.

## Safety

Never overwrite a user's primary disk during early testing.

Initial image validation:

1. QEMU virtual machine
2. spare USB drive
3. spare PC

## Acceptance

A PC image is not complete until it boots, initializes display/input, starts Protea, loads persistent state, survives reboot and has a documented recovery path.
