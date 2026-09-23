# Protea System Builds

This directory will contain reproducible system-build definitions.

For the PC target, Buildroot is a candidate for generating a controlled Linux system image because it can generate a toolchain, root filesystem, Linux kernel and bootloader configuration from one build system.

Buildroot is not the PC shell itself. It is the system-image construction layer.

The project will first prove the shell on normal Linux hardware, then integrate it into a controlled image.

Build definitions must remain reproducible and must not contain personal credentials or secrets.
