# Protea QEMU Validation

The first boot pipeline is validated in QEMU before physical hardware.

## Why QEMU first

- no hardware purchase;
- no risk to the user's disk;
- repeatable;
- easy to capture boot logs;
- suitable for later CI.

The current build entrypoint starts from Buildroot's maintained x86_64 QEMU configuration and builds a baseline Linux image.

## Commands

From a Linux host:

```bash
chmod +x build/fetch-buildroot.sh build/build-pc.sh
./build/build-pc.sh
```

The resulting image must be tested before it is called a Protea OS release.

The PC-image GitHub Actions workflow boots the built kernel and root filesystem
in headless QEMU. In test mode, GTK activates the same lock-screen Sign In
button handler used by the UI, then the test requires the desktop event loop to
emit two post-sign-in heartbeat markers. This verifies the sign-in transition
and that the event loop remains active; it does not validate physical keyboard
or mouse input and does not replace human-driven visual, accessibility, or
hardware testing. Test mode is enabled only when the QEMU kernel command line
contains `protea.qemu_test=1`.

To run the interactive check from the repository root on Linux after building:

```bash
python3 build/qemu/test-interactive.py \
  --kernel output/pc/images/bzImage \
  --rootfs output/pc/images/rootfs.ext2
```

## Integration order

1. boot baseline Linux;
2. add Protea core;
3. add Protea session startup;
4. add graphical stack;
5. add Protea desktop shell;
6. add persistent state;
7. automate and verify the desktop sign-in flow in QEMU;
8. test reboot;
9. package the resulting image.
