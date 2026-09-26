# Protea Phone

The phone path now contains a real Android launcher project that can be built and installed independently while the full AOSP integration is developed.

The current launcher project targets API 36 (compileSdk/targetSdk 36, minSdk 26), matching `phone/launcher/app/build.gradle` and the `Protea Phone` CI workflow. Android 17 (API level 37) is a future upgrade target once the AOSP integration below tracks that release.

This launcher is an integration-stage component, not a claim that Protea is already an AOSP system image.

AOSP integration remains:

1. build unmodified AOSP;
2. boot a supported target;
3. install/replace the launcher in a controlled development build;
4. integrate Protea identity/settings;
5. add system-level services only where justified;
6. validate security, recovery and updates.
