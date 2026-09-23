# Protea OS Test Plan

## Core

- settings read/write
- permissions grant/revoke
- identity creation
- state serialization
- state restoration
- corrupted state rejection
- unsupported format rejection
- restart persistence

## PC

- shell starts
- keyboard navigation
- mouse navigation
- application launch
- settings persistence
- mode switching
- low-memory behavior

## Boot image

- boot
- filesystem mount
- display initialization
- input initialization
- Protea session start
- reboot
- shutdown
- recovery

## Hardware

Record every test machine and repeat tests after major kernel, graphics, shell or core changes.

No benchmark result should be invented or copied from another machine.
