# Protea OS: South African App Ecosystem and Mmino Music Platform

**Document type:** Product vision and future implementation specification
**Status:** Proposed planning document. Listed apps and features are not claimed to be implemented.
**Repository:** https://github.com/saxs-14/Protea-OS
**Audience:** Future contributors

## 1. Purpose

This document records the product direction for Protea OS: a useful, distinctly South African cross-device ecosystem for PC/laptop, phone, TV, and watch. It also defines Mmino, the proposed music product.

South African identity must appear in product decisions, language support, local usefulness, design, music discovery, accessibility, and offline-first engineering—not merely flags, colours, or wallpapers. Products should feel rooted in South Africa while remaining polished and competitive internationally.

This is a long-term plan, not evidence that the listed apps or features already exist.

## 2. Non-negotiable engineering rules

1. **Audit first.** Inspect the current repository, branch, Git status, recent commits, source files, tests, workflows, and project instructions before editing.
2. **Do not rebuild blindly.** Extend existing code and architecture. Do not replace working code or restructure directories without evidence and a clear reason.
3. **Never claim false completion.** Distinguish proposed, in progress, implemented, tested, and released features.
4. **Real software, not mockups.** UI must connect to functioning behaviour. A static screen is not a completed app.
5. **Low budget by design.** Prefer free/open-source tools, local-first functionality, and self-hostable services. No paid cloud or API should be mandatory for core functionality.
6. **Low-end hardware matters.** Avoid heavyweight background services, unnecessarily large downloads, excessive RAM use, and expensive build requirements.
7. **Security and privacy first.** Use least privilege, secure storage, explicit consent, safe defaults, data minimisation, and POPIA-aware practices.
8. **Verify dependencies and licences.** Check maintenance, licence compatibility, platform support, security, and resource cost before adding dependencies.
9. **Test meaningful changes.** Run relevant formatting, linting, unit/integration tests, and builds. Report what ran and what did not.
10. **Preserve user work.** Do not delete files, discard changes, force-push, rewrite history, or perform destructive repository actions without explicit approval.
11. **Never commit secrets.** Do not commit tokens, private keys, credentials, personal data, or private listening/activity data.
12. **Keep status honest.** Update this document only when supported by repository evidence and test results.

## 3. South African product principles

- Design for urban, township, rural, and remote communities. Do not assume fast internet, expensive hardware, or uninterrupted electricity.
- Make offline and low-data use first-class: resumable downloads, adjustable quality, compact assets, clear data estimates, and graceful network failure.
- Plan localisation for all 12 official languages: isiZulu, isiXhosa, Afrikaans, Sepedi, Setswana, Sesotho, Xitsonga, siSwati, Tshivenda, isiNdebele, English, and South African Sign Language (SASL). Be honest about the difference between translated UI, translated content, speech recognition, text-to-speech, and sign-language resources.
- Support South African rand (ZAR), metric units, local date/time conventions, and South African public holidays using maintainable, verified data.
- Respect South Africa's cultural diversity. Do not treat one language, community, province, or visual tradition as representative of everyone.
- Use original, respectful design informed by South African landscapes, contemporary design, music, art, architecture, and community life. Avoid stereotypes and unlicensed cultural artwork.
- Use reliable, dated sources for government services, public holidays, transport, and other changing facts. Show when information was last verified.
- Design for accessibility: keyboard navigation, screen-reader labels, contrast, scalable text, captions, reduced motion, and touch/remote-friendly controls.
- Treat POPIA as an engineering and governance requirement, not a marketing claim. Seek appropriate legal review before commercial release.

## 4. Naming policy

All app names below are **working names**, not final linguistic or trademark approvals.

Before release:
- Ask fluent speakers to verify each name's meaning, spelling, pronunciation, cultural connotations, and suitability.
- Record the intended language and meaning in a naming register.
- Check trademark, domain, package identifier, and app-store availability.
- Do not claim a name is available until checked.
- Do not mix languages without intention or present uncertain translations as authoritative.
- Use neutral internal package names until public branding is approved.

Some names below are language-inspired suggestions and may need correction by fluent speakers. Treat them as creative candidates, not authoritative translations.

## 5. Proposed default apps

These are proposed products, not proof that apps exist in the repository.

### 5.1 PC / laptop

| Working name | Function | Minimum useful scope |
|---|---|---|
| Thabisa | Files | Browse files/folders, copy/move, rename, confirmed deletion, search, removable media, and file associations. Verify name. |
| Kgokagano | Browser | Safe web browsing, bookmarks, downloads, privacy controls, and documented browser-engine/licence decisions. Verify name. |
| Mmino | Music | Local audio library, playback queue, playlists, metadata, artwork, keyboard controls, and later authorised streaming. See Section 7. |
| Ponelopele | Photos | Browse albums, view images, rotate/crop, and export edited copies without silently overwriting originals. Verify name. |
| Ngwala | Notes | Fast offline notes, checklists, search, autosave, export, and optional sync. |
| Bala | Calculator | Basic/scientific modes, percentages, and unit conversions. Clearly label exchange rates as user-entered or sourced with timestamps. Verify name. |
| Matsatsi | Calendar | Calendar views, events, reminders, time zones, import/export, and verified South African public holidays. |
| Ditlhophiso | Settings | Display, language, accessibility, accounts, devices, permissions, network, privacy, updates, and diagnostics. Verify name. |
| Ditokomane | Documents | PDF/document viewing, print/export where supported, and scanning only where hardware/software permits. Verify name. |
| Moagi / Maker Studio | Developer tools | Optional development utilities, terminal/editor integrations, diagnostics, and learning resources. Verify name. |

### 5.2 TV

| Working name | Function | Minimum useful scope |
|---|---|---|
| Buka | TV home / Watch hub | Remote-first launcher for installed and supported media apps. Do not promise access to unsupported services. |
| Mmino TV | Music | TV-friendly music playback, large album art, visualiser, playlists, and authorised music-video sources. |
| Lefatshe | Ambient screens | Landscapes, artwork, photographs, and optional cultural stories with rights and attribution. Verify name. |
| Gae | Household home | Family profiles, shared reminders, and parental controls with consent and PIN-protected settings. Verify name. |

### 5.3 Phone

| Working name | Function | Minimum useful scope |
|---|---|---|
| Shayela | Phone | Dialler, contacts, call history, and emergency-call integration where supported by hardware, OS, and network. Verify name. |
| Khuluma | Messages | SMS and supported messaging integrations. Do not imply internet messaging unless implemented. Verify name. |
| Indlela | Maps | Maps, saved places, and routing from a licensed/open data provider; offline maps only when data terms permit. Verify name. |
| Isikhwama | Personal budget | Offline spending records, savings goals, and ZAR budgets. Not a bank or payment wallet by default. Verify name. |
| Datha | Data manager | Monitor data where platform permissions allow, show estimates, and provide configurable warnings. Never invent live tariffs. |
| Bontša | Camera | Camera/video capture, QR scanning, and controls supported by hardware and APIs. Verify name. |

### 5.4 Watch

| Working name | Function | Minimum useful scope |
|---|---|---|
| Nako | Watch face and time | Time, date, alarms, timers, and calendar reminders. Verify name. |
| Phela | Wellness | Activity and movement reminders; sensor measurements only on supported hardware with appropriate disclaimers. Verify name. |
| Ngivumele | Notifications | Notification mirroring and quick actions where supported by the watch and paired phone. Verify name. |
| Mmino Wrist | Music controls | Play/pause, skip, volume, favourites, and paired-device control. Standalone streaming is a later capability. |

### 5.5 Shared ecosystem apps/services

| Working name | Function |
|---|---|
| Bophelo | Wellbeing and daily tools; avoid medical claims unless validated. |
| Puo | Language selection, translation, and speech tools where reliable resources exist. |
| Kopano | Secure pairing, discovery, and connections between Protea devices. |
| Tšireletso | Security centre, permissions, privacy settings, and device protection. |
| Ntlafatsa | Signed system/app updates, status, and recovery/rollback where feasible. |
| Thuto | Learning resources, study tools, and downloadable educational content with proper rights. |
| Mmaraka | App discovery and installation with publisher identity and package-signature checks. |
| Backup & Restore | User-controlled backup and recovery for supported files/settings. Explain limits. |
| Find My Device | Consent-based location for supported devices, subject to permissions and connectivity. |
| Help Desk | Accessible tutorials, troubleshooting, system information, and support resources. |

## 6. Platform strategy

The repository's stated direction is:
- **PC/laptop:** Linux foundation with a Protea desktop shell.
- **Phone:** AOSP foundation with a Protea phone shell, later in the roadmap.
- **Watch:** Zephyr or another suitable low-power foundation depending on target hardware.
- **TV:** Android TV/AOSP or Linux foundation, chosen according to real device compatibility.
- **Shared layer:** identity, settings, permissions, device profile, and synchronisation contracts.

The first complete hardware path remains PC/laptop. Build a reliable PC experience first, stabilise shared contracts, then expand. For existing smart TVs and TV boxes, investigate an installable TV app/launcher on compatible Android TV devices before attempting custom firmware. Compatibility, vendor restrictions, certification, DRM, and streaming-app support vary by device. AOSP does not automatically include Google proprietary apps, Play services, or commercial streaming rights.

Use native platform capabilities where appropriate. Share data models, API contracts, design tokens, localisation, and service interfaces when useful, but do not force identical UI layouts across platforms.

## 7. Mmino — music product specification

### 7.1 Vision

**Name:** Mmino
**Working tagline:** “Feel the rhythm. Hear Africa.”
**Positioning:** A polished South African music experience that can grow from a local audio player into a cross-device music service.

Mmino should champion South African artists and music while welcoming music from the rest of Africa and the world. It should work on low-cost devices and weak connections, without requiring a paid AI API for core playback.

### 7.2 Product goals

- Excellent playback of user-owned/local audio files.
- Discovery by artists, albums, genres, language, region, and editorial collections.
- Support discovery of amapiano, maskandi, gospel, jazz, hip-hop, house, Afro-soul, and other artist/community-defined scenes.
- Playlists, queue, favourites, history, search, metadata, and artwork.
- Adjustable streaming quality and clear data-use estimates.
- Downloads for offline playback only where the service has the necessary rights and permissions.
- Optional account sync for playlists, favourites, and playback position.
- Captions/lyrics only where appropriately sourced and licensed.
- Artist profiles and submission tools when identity and rights can be verified.
- Full player on PC/phone, remote-friendly TV UI, and watch controls.
- Privacy controls for history, recommendations, telemetry, and account data.

### 7.3 Staged delivery

**Stage 1 — Local Mmino Player**
- Scan user-selected music folders only with permission.
- Support a tested set of audio formats using maintained libraries or platform APIs.
- Implement play/pause, previous/next, seek, volume, queue, shuffle/repeat, playlists, search, artist/album views, metadata, and artwork.
- Support background playback where the platform permits it.
- Provide keyboard, touch, TV remote, and accessibility support appropriate to each target.
- Handle missing/corrupt files, empty libraries, storage removal, and playback errors gracefully.
- No account or cloud backend is required for local playback.
- Do not bundle copyrighted music as demo content without permission.

**Stage 2 — Curated discovery catalogue**
- Build an API for artist profiles, releases, genres, editorial playlists, search, and catalogue metadata.
- Start with music explicitly authorised by independent artists/rights holders or content with verified distribution rights.
- Keep records of rights, permission, territories, permitted uses, duration, and takedown contact/process.
- Provide artist/content moderation and a way to report rights disputes.
- Label demo content and retain permission evidence.

**Stage 3 — Streaming service**
- Add authenticated playback and adjustable/adaptive streaming quality.
- Use expiring authorisation for protected media where appropriate.
- Add rate limits, monitoring, abuse protection, and reliable playback error handling.
- Add permitted offline downloads only after rights and licence terms are confirmed.
- Keep local playback usable if the streaming service is offline.

**Stage 4 — Accounts and cross-device sync**
- Sync playlists, favourites, and playback progress through documented APIs.
- Provide account export and deletion workflows.
- Secure sessions, protect tokens, support revocation, and avoid logging credentials/private listening data.
- Require explicit user approval for device pairing and provide revocation.
- Define simultaneous playback and handoff behaviour before implementing them.

**Stage 5 — Artist and commercial features**
- Consider artist onboarding, verification, catalogue submissions, analytics, royalty statements, subscriptions, or advertising only after rights, legal, accounting, privacy, and business reviews.
- Distinguish gross revenue, fees, contractual deductions, and amounts due to rights holders.
- Do not promise artist earnings or catalogue size without enforceable arrangements.

### 7.4 Music rights and legal constraints

**Music rights are a launch blocker for a commercial streaming catalogue.** Separate rights may apply to the composition, sound recording, performances, label/publisher interests, territories, and specific uses.

- Never scrape Spotify, YouTube, or another service and rehost its catalogue.
- A track being online, downloadable, or user-supplied does not automatically grant commercial streaming rights.
- Obtain and record appropriate licences/permissions for the intended use, territory, streaming, downloads, public performance, and monetisation.
- Investigate relevant South African rights organisations and seek qualified legal advice. Requirements depend on repertoire and service model; do not assume one licence covers every right.
- Respect takedown/dispute requests, attribution requirements, and licence expiry.
- Use royalty-free/openly licensed music only after checking the exact licence and attribution/redistribution conditions.
- Keep local-file playback separate from the commercial streaming catalogue.

This document is product/engineering guidance, not legal advice. A commercial launch requires professional rights and regulatory review.

### 7.5 Suggested technical components (subject to repository audit)

Do not implement this layout until the current architecture has been inspected. Adapt the existing stack where practical.

- **Player core:** playback state, queue, repeat/shuffle, track metadata, error model, and platform audio adapter.
- **Local library:** user-approved file indexing and metadata cache.
- **Catalogue API:** artists, releases, tracks, genres, editorial playlists, and rights status.
- **Streaming service:** authorisation, stream delivery, quality variants, and download entitlements.
- **Identity and sync:** accounts, devices, playlists, favourites, and progress.
- **Artist portal:** submissions, verification, rights declarations, and reporting.
- **Admin/moderation:** catalogue review, rights evidence, reports, and audit trail.
- **Observability:** privacy-conscious diagnostics and operational metrics.

Potential data entities to refine after architecture review and threat modelling: User, Device, Artist, Release/Album, Track, Recording/Asset, RightsGrant, Playlist, PlaylistItem, Favourite, PlaybackProgress, DownloadEntitlement, ContentReport, and ArtistSubmission.

Do not store audio binaries in a relational database. Choose storage/delivery/caching only after validating cost and rights. Keep secrets out of source control.

### 7.6 Music security and privacy

- Collect only data needed for playback, sync, billing, rights compliance, or explicitly enabled features.
- Make history and personalisation controls visible.
- Do not expose individual listening history by default.
- Protect accounts and artist/admin functions with appropriate authentication and authorisation.
- Apply rate limits, input validation, secure sessions, and audit logs for sensitive actions.
- Define retention/deletion rules for history, accounts, submissions, and logs.
- Use age-appropriate protections if children may use the service.
- Review applicable South African privacy, consumer, copyright, and electronic communications requirements before launch.

### 7.7 Monetisation

Potential future models include a free ad-supported tier, subscriptions, artist campaigns, or institutional/community partnerships. None is committed. Do not add ad trackers or payment integrations to the first local-player release. Validate rights, infrastructure cost, artist compensation, demand, and legal obligations first.

## 8. Design system and identity

- Build a refined, modern design with strong typography, clear hierarchy, contrast, and accessible focus states.
- A possible palette may draw on deep green, warm gold, earth tones, and neutral surfaces, but validate it in real UI and accessibility tests.
- Use original illustrations, photography, and audio artwork or assets with clear licences.
- Support small displays, low-resolution screens, TV viewing distance, remote controls, keyboard, and touch.
- Respect reduced-motion preferences and avoid distracting animation.
- Support dark/light modes where practical; share design tokens without forcing identical layouts on every platform.
- Do not use flags and decorative cultural motifs as substitutes for useful localisation and local content.

## 9. Shared cross-device foundations

Plan and document stable contracts for:
- Device identity and secure pairing.
- User accounts and local-only guest use.
- Language, region, date/time, and accessibility preferences.
- Permission requests and revocation.
- Notifications and user-controlled sync.
- Secure updates and recovery.
- App metadata, versioning, and trusted installation.
- Data export, backup, and deletion.
- Offline state and retry behaviour.

An account must not be mandatory for local files, basic playback, notes, calculator, or other core offline utilities unless a feature genuinely requires one.

## 10. Quality, validation, and release gates

Define acceptance criteria and tests for:
- Installation, startup, shutdown, upgrades, and recovery.
- Core workflows and common failure cases.
- Offline and poor-network operation.
- Low memory/storage conditions.
- Keyboard, touch, remote, screen-reader, and text-scaling usability as relevant.
- Security/privacy permissions and sensitive-data handling.
- Licence notices, attributions, and asset rights.
- Unit/integration/platform tests and reproducible CI builds.
- Real target hardware testing before claiming device compatibility.

For Mmino, test interrupted streams, missing files, invalid metadata, unsupported formats, audio focus, device removal, duplicate tracks, long playlists, account expiry, revoked downloads, and server outages.

Do not claim production readiness merely because an app compiles or a screenshot looks complete.

## 11. Recommended implementation order

1. Audit repository, architecture, working tree, tests, and open CI failures.
2. Record current state before code changes.
3. Establish a naming register and South African design/localisation guidelines.
4. Stabilise PC foundation and shared contracts, consistent with the existing roadmap.
5. Choose one small default PC utility and deliver it end-to-end with tests.
6. Build **Mmino Stage 1**, a real local music player, using the existing stack where practical.
7. Test on PC and document audio/platform decisions.
8. Design the discovery catalogue and rights model before commercial streaming.
9. Add mobile and TV experiences using supported platform foundations.
10. Add watch controls after pairing and playback contracts are stable.
11. Add streaming only after authorised content, rights, cost, security, and operational plans are in place.
12. Reassess based on tested outcomes, available hardware, budget, and feedback.

Do not attempt all device ports, a custom AOSP image, and a commercial streaming service in one change.

## 12. Claude Code execution instructions

When work begins:

1. Read this document and the root README, then read any CLAUDE.md, AGENTS.md, contribution guide, architecture documents, and relevant package manifests.
2. Inspect the actual repository tree and Git status. Identify the real default branch and do not assume branch names or paths.
3. Summarise existing components, implementation status, tests, constraints, and the smallest safe next task.
4. Follow repository-specific instructions unless they conflict with safety/scope rules here.
5. Implement only the explicitly approved phase. Do not silently broaden scope.
6. Reuse working architecture and dependencies unless a documented reason justifies change.
7. Keep changes small, reviewable, and reversible. Do not delete or overwrite user work.
8. Never invent credentials, services, API responses, licences, catalogue content, translations, or test results.
9. For each app, define acceptance criteria and a short plan, then implement the smallest end-to-end slice.
10. Add/update tests and documentation. Run relevant checks and report exact commands/results.
11. If platform requirements, licences, hardware, or an external service are uncertain, document uncertainty and ask for a decision instead of guessing.
12. Before finishing, report files changed, functionality implemented, tests/results, known limitations, security/licensing concerns, and the next recommended step.
13. Never claim code was committed, pushed, deployed, tested on hardware, or released unless the action actually happened and is verifiable.
14. Do not perform destructive Git actions, rewrite history, force-push, change repository visibility, or delete branches without explicit approval.

## 13. Definition of success

Protea OS should become a dependable, accessible, low-cost cross-device platform with useful products that South Africans recognise as designed for real needs. Mmino should first become an excellent local player, and only later a licensed music service if the legal, technical, and financial foundations support it.

**Guiding statement:** Built in South Africa, designed for South Africans, and made to compete globally.

## 14. Current status

This file records proposed product direction and future implementation requirements. It does not certify that any app or Mmino feature is implemented. Verify code and tests in the repository before planning or reporting implementation.
