# Protea OS: Multi-Device Expansion Vision

**Document type:** Product vision and future implementation specification
**Status:** Proposed planning document. Nothing in this document is claimed to be implemented. It records direction so future work (human or AI-assisted) builds toward one agreed shape instead of guessing.
**Repository:** https://github.com/saxs-14/Protea-OS

## 1. Purpose

This document expands the original PC/phone/watch/TV plan in `ARCHITECTURE.md` with a sixth target class - server/cloud - and sets a concrete reference point for each shell's look and feel. It does not replace `ARCHITECTURE.md`'s phase order or `DEVELOPMENT.md`'s engineering rules; it adds detail inside them.

## 2. Non-negotiable rules (restated, these govern everything below)

1. Audit the existing repository before writing anything new. Extend real code; do not replace working code without evidence.
2. No fake implementations. A screen that doesn't connect to real behaviour is not a feature.
3. Never claim something is complete, "like Windows," or "like Android" unless a human has actually used it and it is backed by passing tests/CI.
4. One target at a time. Each of the six device classes below is its own multi-month effort at minimum. Do not start parallel half-built implementations across all six - that produces six broken things instead of one working thing.
5. Low-budget, low-end-hardware, security-and-privacy-first still apply to every target, including the server flavor.

## 3. Target reference points

| Target | Reference point | What that means concretely | What it does NOT mean |
|---|---|---|---|
| PC / laptop / desktop | Windows-like | Start menu, taskbar with running-app icons and system tray, draggable/resizable windows, familiar keyboard shortcuts | Running actual Windows binaries, cloning Windows' codebase, or any Microsoft trademark/UI asset reuse |
| Server / WSL-like / cloud-core | A general-purpose Linux server flavor | Headless Buildroot (or Debian-based) image, container runtime, package manager, SSH, a WSL-style "Linux inside Protea PC" mode | A competitor to every SUSE/Ubuntu/RHEL product line on day one |
| Phone / tablet | Android-like but lean | Familiar gestures/app drawer/notification shade, but minimal background services and small storage footprint | A full Android/AOSP replacement before the existing launcher work (Phase 4) is finished |
| Watch | Three capability tiers | Tier 1: time/notifications only. Tier 2: "smart watch" (health sensors, apps). Tier 3: Apple Watch-class (cellular, payments, deeper health) | Promising Tier 3 hardware support before Tier 1 exists on any real device |
| TV | Smart-TV-plus | Launcher, media/casting, voice/remote nav - "plus" means Protea identity/sync, not reinventing codecs or DRM | A new streaming platform or content licensing business |

## 4. Server/cloud workload categories

The server flavor should eventually serve, in order of realistic difficulty (easiest first):

1. **Infrastructure** - basic headless boot, SSH, package management, container runtime (Podman/containerd, no paid tooling required).
2. **Cloud native development** - a dev box: git, compilers, Docker/Podman, local Kubernetes (k3s) for testing.
3. **IoT and devices** - lightweight runtime for edge/IoT workloads, reusing the same Protea core contracts as PC.
4. **Open source security** - hardening baseline (least privilege, signed packages, audit logging) built in from the start, not bolted on.
5. **Data** - self-hostable data tooling (Postgres, object storage) with no mandatory paid service.
6. **Managed Infrastructure** - orchestration/fleet-management features. Realistically a multi-year goal; do not block earlier phases on it.
7. **AI** - local/self-hosted inference tooling. Also realistically a later-stage goal once the base server image is solid.

## 5. Desktop imagery

Desktop wallpapers/backgrounds should showcase real South African heritage sites and landscapes, sourced legally (see implementation note below on licensing) rather than scraped from arbitrary commercial/tourism sites without rights to redistribute.

## 6. Suggested build order (recommendation, not yet decided)

1. Evolve the existing GTK4 PC shell toward the Windows-like reference point - this extends code that already boots and is CI-validated today, so it is the lowest-risk, highest-confidence next step.
2. Define a separate, headless server Buildroot target (new `build/build-server.sh`, not a modification of the desktop image) once the desktop shell work is underway.
3. Mobile, watch, and TV remain Phase 4/6 per `ARCHITECTURE.md` - pick up only one at a time, after PC is further along.

This order is a recommendation. It is written here so the actual decision is made explicitly rather than implied by whichever task happens to run first.
