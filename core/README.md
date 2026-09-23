# Protea Shared Core

This directory contains the shared software contracts and services used by Protea device shells.

The first executable implementation is a standard-library-only Rust library plus a small CLI harness.

Implemented now:

- device class
- hardware tier
- device profile
- local identity model
- settings model
- permission model
- Gaming/Office mode state
- aggregate Protea state
- unit tests

Not implemented yet:

- durable persistence
- IPC
- encrypted storage
- synchronization
- authentication
- OS service integration

Those are deliberately separate steps. The current code is real executable foundation code, not a UI mockup.
