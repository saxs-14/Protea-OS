# Build and testing status

This document records the commands actually run and the evidence they produced during the repository audit.

## Baseline repository state

Commands executed:

```bash
git status --short --branch
git branch --show-current
git remote -v
git log --oneline -n 10
git diff --stat
```

Observed result:

- Repository: `main`
- Remote: `origin` points to `https://github.com/saxs-14/Protea-OS`
- Local checkout was aligned with `origin/main` at the time of inspection
- The worktree was not clean: untracked concept documents were present and the repo contained many modified tracked files that were preserved as-is

## Remote GitHub inspection

Commands executed:

```bash
gh auth status
gh run list --limit 10 -R saxs-14/Protea-OS
gh run view 36419841531 --log-failed -R saxs-14/Protea-OS
```

Observed result:

- GitHub CLI successfully authenticated.
- The most recent failing `Protea PC Image` run was `36419841531` (`Add periodic per-directory disk usage monitor during the build`).
- The workflow history shows repeated failures in the image build path and a final disk-pressure issue on the runner.
- The workflow sequence indicates the repo repeatedly hit Buildroot package/configuration issues before landing on the final disk-space problem.

## Local Rust validation attempt

Command executed:

```bash
cd /mnt/c/Users/mamag/Documents/Projects/ProteaOS/core && cargo test --quiet
```

Observed result:

```text
error: linker `cc` not found
  |
  = note: No such file or directory (os error 2)
```

Interpretation:

- The repo does contain a Rust workspace and tests, but the local environment lacked the C toolchain needed to link the binaries.
- This is an environment blocker, not proof that the Rust code is incorrect.

## Buildroot image attempt

Command executed:

```bash
cd /mnt/c/Users/mamag/Documents/Projects/ProteaOS && ./build/build-pc.sh
```

Observed result:

- No output tree was generated in this environment.
- There was no local proof that the full Buildroot image built successfully here.

This means the repository is not fully validated in the current environment; the status remains `PARTIAL`/`BROKEN` for the image pipeline.

## Environment limitations encountered

- `cargo` cannot run without a linker toolchain (`cc` missing).
- Installing the build dependencies in this container was constrained by missing or delayed apt privileges and the observed environment restrictions.
- The repo therefore has evidence from source inspection and GitHub CI, but not a full local successful image build in this session.

## Recommended validation sequence for a complete machine

1. Install the Linux build dependencies required by `build/build-pc.sh`.
2. Run:

```bash
chmod +x build/fetch-buildroot.sh build/build-pc.sh build/prepare-pc-overlay.sh build/qemu/run-console.sh build/validate-pc-image.sh
./build/build-pc.sh
```

3. Run:

```bash
./build/validate-pc-image.sh
```

4. Boot the generated image in QEMU:

```bash
build/qemu/run-console.sh
```

5. If QEMU reaches a boot marker, validate the expected `PROTEA_BOOT_OK` output.

## Conclusion

The repo has documented and partially tested build logic, but the PC image is not proven complete in this environment. The current evidence is in the GitHub workflow history and repo source code, not in a successful local end-to-end build.
