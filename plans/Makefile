.PHONY: base gpg help new-plan
.DEFAULT_GOAL := base

# The root path of the Habitat file system. If the `$HAB_ROOT_PATH` environment
# variable is set, this value is overridden, otherwise it is set to its default
HAB_ROOT_PATH ?= /opt/bldr
BLDR_KEY_CACHE := $(HAB_ROOT_PATH)/cache/keys
BLDR_GPG_CACHE := $(HAB_ROOT_PATH)/cache/gpg

base: gpg ## builds all base packages in serial order
	sh ./build-base-plans.sh
	mkdir -pv $(BLDR_KEY_CACHE)
	cp ./chef-public.gpg $(BLDR_KEY_CACHE)/chef-public.asc

gpg: ## imports (temporary) package signing keys
	mkdir -pv $(BLDR_GPG_CACHE)
	- gpg --import chef-public.gpg
	- gpg --import chef-private.gpg
	- gpg --homedir $(BLDR_GPG_CACHE) --import chef-public.gpg
	- gpg --homedir $(BLDR_GPG_CACHE) --import chef-private.gpg

help:
	@perl -nle'print $& if m{^[a-zA-Z_-]+:.*?## .*$$}' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

new-plan: ## creates a new Plan from a template, plan-tmpl.sh
	mkdir -p $(plan)
	sed 's/PACKAGE/$(plan)/g' plan-tmpl.sh > $(plan)/plan.sh
