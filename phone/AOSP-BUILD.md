# Protea AOSP Build Strategy

AOSP is a major build system. Protea will not pretend that compiling AOSP is the same thing as having a working Protea phone OS.

## Current platform reference

Android 17 is API level 37. The current AOSP latest-release manifest points to the Android 17 release branch.

The project should pin an exact AOSP tag when beginning a reproducible phone build instead of silently following a moving branch.

## Zero-budget strategy

1. Use the Android SDK and emulator for early launcher testing.
2. Build the smallest possible AOSP target on available Linux hardware.
3. Use Cuttlefish where appropriate for platform-level testing.
4. Only acquire a physical test phone when a supported device is selected.
5. Keep device-specific vendor blobs and proprietary components outside the shared Protea core.

## Integration rule

First prove:

AOSP -> boots -> launcher -> Protea shell

Then prove:

AOSP -> Protea identity/settings -> persistent system behavior

Only after that should deeper framework/system-service modifications be introduced.

## Security

A release phone build must preserve Android's verified boot, signing, permission and update mechanisms appropriate to the selected device.

Source references:

- https://source.android.com/
- https://developer.android.com/about/versions/17/
