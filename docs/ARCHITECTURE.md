# Protea OS Architecture

## 1. Purpose

Protea OS is a cross-device operating experience built on proven foundations.

The central Protea technology is not a new kernel. It is the shared system layer and adaptive shells that make different device classes feel like one platform.

## 2. Layer model

```text
┌───────────────────────────────────────────────┐
│                 Protea Shells                  │
│ PC │ Phone │ Watch │ TV                        │
├───────────────────────────────────────────────┤
│            Protea Shared Services              │
│ Identity │ Settings │ Permissions │ Profiles   │
│ Sync │ Device capabilities │ Policy             │
├───────────────────────────────────────────────┤
│             Platform Integration               │
│ Linux │ AOSP │ Zephyr/seL4 │ Android TV/Linux │
├───────────────────────────────────────────────┤
│                 Hardware                      │
│ CPU │ GPU │ RAM │ Storage │ Network │ Sensors │
└───────────────────────────────────────────────┘
```

## 3. Shared services

### Identity

A Protea identity represents the local user/device relationship. The first implementation is local-first. Network identity and synchronization are added only after local behavior is reliable.

### Settings

Settings are typed values with explicit ownership and scope.

Initial scopes:

- device
- user
- shell
- mode

### Permissions

Permissions are explicit capabilities. Components must not silently receive access to resources.

Initial permission concepts:

- storage
- network
- notifications
- device information
- account data

### Device profile

The profile describes available resources and capabilities so the shell can adapt.

Important signals include:

- CPU class
- memory class
- storage capacity
- GPU availability
- battery presence
- display characteristics
- input capabilities

## 4. Hardware tiers

Protea will use resource tiers rather than pretending every device can provide identical performance.

### Minimum

Old or constrained hardware. Effects, background work and synchronization are reduced.

### Recommended

Normal modern hardware. Full standard experience.

### Full

High-performance hardware. Additional visual effects and advanced workloads may be enabled.

The tier is an implementation policy, not a marketing label.

## 5. PC architecture

The first PC implementation runs as a Linux user session.

Initial progression:

1. Protea shared core library
2. Protea settings/permission service
3. Desktop shell process
4. Session startup integration
5. Window/application integration
6. Gaming/Office mode controller
7. Bootable Linux image
8. Hardware validation

The first shell does not replace the Linux kernel or GPU drivers.

## 6. Phone architecture

The phone platform will be based on AOSP.

Protea will initially provide:

- launcher/shell
- settings integration
- identity integration
- device profile
- Protea visual/system conventions

The project will track current AOSP compatibility requirements instead of inventing a parallel Android application format.

## 7. Synchronization

Synchronization is intentionally local-first.

A device must remain useful without an Internet connection.

Future sync architecture will support:

- conflict detection
- explicit data ownership
- encrypted transport
- encrypted sensitive data at rest
- offline changes
- deterministic merge rules

No paid cloud dependency is required for the initial implementation.

## 8. Security model

Security is a system requirement, not a later feature.

The project will progressively add:

- least privilege
- signed builds
- verified update strategy
- secure storage
- audit logging
- dependency tracking
- reproducible builds where practical
- threat modelling for shared services

## 9. What Protea OS is not

Protea OS is not initially:

- a new CPU architecture
- a replacement for Linux
- a replacement for Android
- a new GPU driver ecosystem
- a Windows compatibility layer written from scratch
- a promise of universal hardware support

Those would be separate, much larger projects.

## 10. Long-term definition of done

Protea OS becomes a real distributable operating system when a supported hardware target can:

1. boot Protea software from power-on;
2. initialize hardware;
3. present the Protea shell;
4. create/use a local identity;
5. persist settings;
6. enforce permissions;
7. switch operating modes;
8. install/run supported applications;
9. recover from normal failures;
10. update through a verified mechanism;
11. preserve user data through supported updates;
12. pass documented hardware and security tests.

Until then, components are considered engineering stages toward that definition.
