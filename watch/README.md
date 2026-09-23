# Protea Watch

The watch target is deliberately deferred until PC and phone foundations are stable.

Candidate foundations from the architecture are Zephyr or seL4, selected according to the actual hardware and licensing/support constraints.

The watch shell must be glanceable and resource-aware.

Required capabilities:

- time
- notifications
- basic controls
- device status
- Protea identity/pairing
- safe firmware/update path

The watch must not pull the full PC/phone UI stack onto constrained hardware.
