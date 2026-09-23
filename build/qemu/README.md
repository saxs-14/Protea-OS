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

## Integration order

1. boot baseline Linux;
2. add Protea core;
3. add Protea session startup;
4. add graphical stack;
5. add Protea desktop shell;
6. add persistent state;
7. test reboot;
8. package the resulting image.
