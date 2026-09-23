# Protea Cross-Device Contract

The same Protea concepts must behave consistently across device classes even when their implementations differ.

## Identity

A device may have a local Protea identity, paired devices, and an optional network identity in a later phase. Offline use remains valid.

## Settings

Every synchronized setting must declare:

- key
- type
- owner
- scope
- default
- version
- merge behavior

A device may ignore settings it cannot support, but it must not silently reinterpret them.

## Permissions

Permissions are capabilities, not UI decorations.

Each platform adapter maps Protea permissions to the native security model.

## Modes

Common mode semantics begin with Gaming and Office.

A platform may implement the policy differently. PC may alter background workload and notifications; phone may alter notifications and power behavior; a watch may only change notification priority.

## Device capabilities

Each device publishes capabilities so the shared layer can make safe decisions.

Unknown capabilities are treated as unavailable.

## Synchronization

Future synchronization must be:

- encrypted
- authenticated
- offline-first
- conflict-aware
- versioned
- exportable

No cloud vendor is part of this contract.
