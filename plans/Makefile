.PHONY: base help new-plan
.DEFAULT_GOAL := base

base: ## builds all base packages in serial order
	bash ./build-base-plans.sh

help:
	@perl -nle'print $& if m{^[a-zA-Z_-]+:.*?## .*$$}' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

new-plan: ## creates a new Plan from a template, plan-tmpl.sh
	mkdir -p $(plan)
	sed 's/PACKAGE/$(plan)/g' plan-tmpl.sh > $(plan)/plan.sh
