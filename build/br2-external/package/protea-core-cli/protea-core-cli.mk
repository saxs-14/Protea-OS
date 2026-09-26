################################################################################
#
# protea-core-cli
#
################################################################################

PROTEA_CORE_CLI_VERSION = 0.1.0
PROTEA_CORE_CLI_SITE = $(BR2_EXTERNAL_PROTEA_PATH)/../../core
PROTEA_CORE_CLI_SITE_METHOD = local
PROTEA_CORE_CLI_SUBDIR = protea-core-cli

$(eval $(cargo-package))
