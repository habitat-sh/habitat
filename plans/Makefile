.PHONY: base gpg help new-plan
.DEFAULT_GOAL := base

base: gpg ## builds all base packages in serial order
	sh ./build-base-plans.sh
	mkdir -pv /opt/bldr/cache/keys
	cp ./chef-public.gpg /opt/bldr/cache/keys/chef-public.asc

gpg: ## imports (temporary) package signing keys
	mkdir -pv /opt/bldr/cache/gpg
	- gpg --import chef-public.gpg
	- gpg --import chef-private.gpg
	- gpg --homedir /opt/bldr/cache/gpg --import chef-public.gpg
	- gpg --homedir /opt/bldr/cache/gpg --import chef-private.gpg

help:
	@perl -nle'print $& if m{^[a-zA-Z_-]+:.*?## .*$$}' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

new-plan: ## creates a new Plan from a template, plan-tmpl.sh
	mkdir -p $(plan)
	sed 's/PACKAGE/$(plan)/g' plan-tmpl.sh > $(plan)/plan.sh
