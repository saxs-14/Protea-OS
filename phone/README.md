# Protea Phone

The phone path now contains a real Android launcher project that can be built and installed independently while the full AOSP integration is developed.

Android 17 is API level 37. The current launcher project targets API 37 and uses Android Gradle Plugin 8.9.0 or newer, matching the Android 17 SDK setup guidance.

This launcher is an integration-stage component, not a claim that Protea is already an AOSP system image.

AOSP integration remains:

1. build unmodified AOSP;
2. boot a supported target;
3. install/replace the launcher in a controlled development build;
4. integrate Protea identity/settings;
5. add system-level services only where justified;
6. validate security, recovery and updates.
