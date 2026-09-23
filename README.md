# Protea OS

Protea OS is a long-term cross-device operating system project.

The goal is a real operating system and software platform that provides a consistent Protea identity, settings, permissions, and adaptive user experience across PC/laptop, phone, watch, and TV form factors.

This repository is being built as a real engineering project, not a visual mockup. The project will use proven open-source foundations rather than attempting to replace every kernel, driver, bootloader, and hardware stack from scratch.

## Current engineering direction

- **PC/laptop:** Linux foundation + Protea desktop shell
- **Phone:** AOSP foundation + Protea phone shell
- **Future watch:** Zephyr or another suitable low-power foundation
- **Future TV:** Android TV/Linux foundation
- **Shared layer:** Protea identity, settings, permissions, device profile, synchronization

## First real hardware target

The first complete hardware path is the PC/laptop platform.

We will first make the Protea software run reliably on ordinary Linux hardware, then turn that software into a bootable Protea Linux system. Android/AOSP work follows once the shared contracts are stable.

## Design principles

1. Free and open-source tools first.
2. No paid cloud service is required for the core system.
3. Low-end hardware is a first-class requirement.
4. Dependencies must have a clear reason to exist.
5. Security and privacy are designed in from the beginning.
6. Shared contracts must remain stable across device shells.
7. Every major component must be testable independently.
8. No claim of "complete OS" is made until it actually boots, installs, runs, updates, and survives real hardware testing.

## Repository structure

```text
Protea-OS/
├── core/                 # Shared Protea system contracts and services
├── pc/                   # PC/laptop shell and integration
├── phone/                # AOSP integration and phone shell
├── build/                # Reproducible system-image/build definitions
├── branding/             # Approved Protea visual assets
├── docs/                 # Architecture, requirements and engineering records
├── tests/                # Cross-component and hardware validation
└── tools/                # Developer utilities
```

## Project status

**Foundation started.**

The repository intentionally starts small. We will add executable components only when their contracts and tests are defined. This keeps the project realistic for a student developer with no budget while preserving the long-term OS goal.

## Licensing

A project-wide license will be selected before external distribution. Individual upstream components retain their own licenses.
