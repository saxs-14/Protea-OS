# Protea PC Build Plan

The PC system is built in two stages.

## Stage A — normal Linux development

Develop and test the Protea shell inside an existing Linux installation. This avoids making every UI change require a full OS image rebuild.

## Stage B — controlled bootable system

After the shell and core are stable, Buildroot will produce a controlled Linux system image.

Buildroot generates the root filesystem, Linux kernel image and bootloader for a target system. The PC image uses Buildroot 2025.02.18's pinned Bootlin x86_64/glibc stable external toolchain instead of compiling a second toolchain from source; this reduces the image build's time and peak disk use on GitHub-hosted runners. The resolved Buildroot configuration and image validator check that this profile remains selected.

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
