# Protea PC boot media

The PC build produces a bootable ISO in addition to the kernel/rootfs artifacts.

## Validation

Build the image with:

`./build/build-pc.sh`

Expected artifacts:

- `output/pc/images/bzImage`
- `output/pc/images/rootfs.ext2`
- `output/pc/images/rootfs.iso9660`

The ISO uses the Linux kernel plus a Buildroot-generated initramfs and GRUB 2. It is intended for QEMU validation first and later USB installation/media testing.

Example QEMU boot:

`qemu-system-x86_64 -m 1024 -smp 2 -cdrom output/pc/images/rootfs.iso9660 -device virtio-gpu-pci -nographic`

Physical installation remains a separate validation gate because storage layout, firmware mode, GPU, Wi-Fi, audio and recovery behavior must be tested on real supported hardware before release.
