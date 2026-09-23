# Protea Shared Core

This directory contains the shared software contracts and services used by Protea device shells.

The first implementation is local-first and deliberately dependency-light.

Planned modules:

- identity
- settings
- permissions
- device profile
- synchronization

The shared core must not contain PC-specific UI code or phone-specific UI code.

The first implementation target is a small Rust library with a command-line test harness. The command-line harness is an engineering tool, not the final user interface.
