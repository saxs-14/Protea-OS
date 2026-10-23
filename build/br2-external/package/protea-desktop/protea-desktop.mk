################################################################################
#
# protea-desktop
#
################################################################################

PROTEA_DESKTOP_VERSION = 0.1.0
PROTEA_DESKTOP_SITE = $(BR2_EXTERNAL_PROTEA_PATH)/../..
PROTEA_DESKTOP_SITE_METHOD = local
PROTEA_DESKTOP_SUBDIR = pc/protea-shell
PROTEA_DESKTOP_CARGO_BUILD_OPTS = --bin protea-desktop
PROTEA_DESKTOP_OVERRIDE_SRCDIR_RSYNC_EXCLUSIONS = \
	--exclude output \
	--exclude .cache \
	--exclude target \
	--exclude .git \
	--exclude phone \
	--exclude watch \
	--exclude tv

$(eval $(cargo-package))
