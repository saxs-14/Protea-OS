# Protea OS — agent notes

Cross-device OS. Rust cargo workspaces (`core/` protea-core + protea-core-cli; `pc/` GTK4 shell), Android launcher (Gradle, API 36), Buildroot PC image. No Node.js — do not introduce JS tooling.

## Layout
- `core/` shared Rust contracts/services · `pc/` GTK4 shell · `phone/launcher` Android launcher (+ AOSP docs)
- `build/` Buildroot scripts, QEMU, validation · `docs/` · `tests/` (test plan only) · `tv/`, `watch/` placeholders

## Commands
- Core: from `core/` — `cargo fmt --all`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`
- CLI smoke: `PROTEA_STATE_FILE=<file> cargo run -q -p protea-core-cli -- status` (from `core/`)
- PC shell: from `pc/` — `cargo check --workspace` (needs GTK4 dev libs; Linux/CI)
- Phone: from `phone/launcher` — `gradle assembleDebug` (JDK 17, Gradle 8.11.1)
- PC image: `./build/build-pc.sh` then `./build/validate-pc-image.sh` (Linux only, long)

## CI (`.github/workflows/`)
core.yml (fmt, test, clippy, CLI smoke) · pc.yml (fmt, check) · phone.yml (assembleDebug) · pc-image.yml (Buildroot image + QEMU boot expecting `PROTEA_BOOT_OK`, 120 min).
Note: CI runs `cargo fmt --all` without `--check`, so formatting drift is not caught — run fmt locally.

## Local environment
Rust is not installed on the Windows dev PC; rely on CI or install rustup. PC image/GTK builds need Linux.
