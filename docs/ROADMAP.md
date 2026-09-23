# Protea OS Engineering Roadmap

This roadmap describes the path from an empty repository to a real operating system. It is intentionally staged so the project can be developed with free tools and limited hardware.

## Phase 0 — Repository foundation

- [x] Establish repository structure and engineering principles
- [x] Define shared architecture
- [ ] Add approved production logo assets
- [ ] Add project-wide license
- [ ] Establish CI
- [ ] Establish issue/milestone tracking

## Phase 1 — Shared core

- [ ] Define stable identity model
- [ ] Define settings model
- [ ] Define permission model
- [ ] Define device profile
- [ ] Implement local persistence
- [ ] Add unit/integration tests
- [ ] Define versioned core API
- [ ] Document compatibility rules

## Phase 2 — Real PC shell

- [ ] Linux session bootstrap
- [ ] Desktop shell process
- [ ] Taskbar
- [ ] Application/start surface
- [ ] Window/application launching
- [ ] Protea settings surface
- [ ] Shared-core integration
- [ ] Basic accessibility
- [ ] Keyboard and mouse navigation

## Phase 3 — Real operating modes

### Gaming

- [ ] Mode state
- [ ] Notification policy
- [ ] Background-work policy
- [ ] Performance profile integration where supported
- [ ] Safe rollback when a performance change fails

### Office

- [ ] Mode state
- [ ] Quiet/battery policy
- [ ] Productivity shortcuts
- [ ] Notification policy

Mode changes must remain safe and reversible.

## Phase 4 — Bootable Protea PC system

- [ ] Reproducible Linux image
- [ ] Bootloader
- [ ] Kernel configuration
- [ ] Filesystem
- [ ] Display stack
- [ ] Input stack
- [ ] Protea session auto-start
- [ ] Recovery environment
- [ ] Installation documentation

## Phase 5 — Low-end hardware validation

At minimum, test on:

- older dual-core PC/laptop
- 2–4 GB RAM machine
- SSD and HDD where available
- integrated graphics
- common USB keyboard/mouse
- wired and Wi-Fi networking where available

Record boot time, idle memory, CPU usage, storage usage, failures and recovery behavior.

## Phase 6 — Phone foundation

- [ ] Select supported AOSP release
- [ ] Establish reproducible AOSP environment
- [ ] Select an affordable/openly documented test device
- [ ] Build unmodified AOSP first
- [ ] Boot stock AOSP build on supported hardware
- [ ] Add Protea launcher
- [ ] Add Protea shared-core integration
- [ ] Add Protea settings integration
- [ ] Validate update and recovery behavior

## Phase 7 — Cross-device identity and sync

- [ ] Device pairing
- [ ] Secure authentication
- [ ] Encrypted synchronization
- [ ] Conflict handling
- [ ] Offline-first behavior
- [ ] Data export/import
- [ ] Account recovery

## Phase 8 — Additional device classes

Only after PC and phone are reliable:

- watch
- TV
- other constrained devices

## Phase 9 — Production engineering

- [ ] Signed release artifacts
- [ ] Reproducible builds where practical
- [ ] Security review
- [ ] Update infrastructure
- [ ] Hardware compatibility matrix
- [ ] Installer/recovery tooling
- [ ] Documentation
- [ ] Public release process

## Budget rule

No stage should assume paid infrastructure.

Prefer:

- Linux
- Git
- GitHub Free
- GitHub Actions within available free limits
- Buildroot
- AOSP
- Rust/C/C++/Python where justified
- local test hardware
- free documentation and open standards

If a later stage requires paid hardware or hosting, document the requirement before spending money.
