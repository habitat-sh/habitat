UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
	IN_DOCKER := true
endif

ifneq ($(IN_DOCKER),)
	build_args := --build-arg HAB_DEPOT_URL=$(HAB_DEPOT_URL)
	run_args := -e HAB_DEPOT_URL=$(HAB_DEPOT_URL)
	run_args := $(run_args) -e HAB_ORIGIN=$(HAB_ORIGIN)
	ifneq (${http_proxy},)
		build_args := $(build_args) --build-arg http_proxy="${http_proxy}"
		run_args := $(run_args) -e http_proxy="${http_proxy}"
	endif
	ifneq (${https_proxy},)
		build_args := $(build_args) --build-arg https_proxy="${https_proxy}"
		run_args := $(run_args) -e https_proxy="${https_proxy}"
	endif

	dimage := habitat/devshell
	docker_cmd := env http_proxy= https_proxy= docker
	compose_cmd := env http_proxy= https_proxy= docker-compose
	common_run := $(compose_cmd) run --rm $(run_args)
	run := $(common_run) shell
	docs_host := ${DOCKER_HOST}
	docs_run := $(common_run) -p 9633:9633 shell
else
	run :=
	docs_host := 127.0.0.1
	docs_run :=
endif

.PHONY: help all bin shell serve-docs test unit functional clean image docs gpg
.DEFAULT_GOAL := bin

bin: image ## builds the project's main binaries
	$(run) sh -c 'cd components/hab && cargo build'
	$(run) sh -c 'cd components/sup && cargo build'
	$(run) sh -c 'cd components/depot && cargo build'

all: image ## builds all the project's Rust components
	$(run) sh -c 'cd components/builder-api && cargo build'
	$(run) sh -c 'cd components/builder-sessionsrv && cargo build'
	$(run) sh -c 'cd components/builder-vault && cargo build'
	$(run) sh -c 'cd components/common && cargo build'
	$(run) sh -c 'cd components/core && cargo build'
	$(run) sh -c 'cd components/depot-core && cargo build'
	$(run) sh -c 'cd components/depot-client && cargo build'
	$(run) sh -c 'cd components/director && cargo build'
	$(MAKE) bin

test: image ## tests the project's Rust components
	$(run) sh -c 'cd components/builder-api && cargo test'
	$(run) sh -c 'cd components/builder-dbcache && cargo test'
	$(run) sh -c 'cd components/builder-protocol && cargo test'
	$(run) sh -c 'cd components/builder-sessionsrv && cargo test'
	$(run) sh -c 'cd components/builder-vault && cargo test'
	$(run) sh -c 'cd components/core && cargo test'
	$(run) sh -c 'cd components/depot-core && cargo test'
	$(run) sh -c 'cd components/depot-client && cargo test'
	$(run) sh -c 'cd components/common && cargo test'
	$(run) sh -c 'cd components/sup && cargo test '
	$(run) sh -c 'cd components/depot && cargo test'
	$(run) sh -c 'cd components/director && cargo test'

unit: image ## executes the components' unit test suites
	$(run) sh -c 'cd components/builder-api && cargo test --lib'
	$(run) sh -c 'cd components/builder-dbcache && cargo test --lib'
	$(run) sh -c 'cd components/builder-protocol && cargo test --lib'
	$(run) sh -c 'cd components/builder-sessionsrv && cargo test --lib'
	$(run) sh -c 'cd components/builder-vault && cargo test --lib'
	$(run) sh -c 'cd components/core && cargo test'
	$(run) sh -c 'cd components/depot-core && cargo test --lib'
	$(run) sh -c 'cd components/depot-client && cargo test --lib'
	$(run) sh -c 'cd components/director && cargo test --lib'
	$(run) sh -c 'cd components/common && cargo test --lib'
	$(run) sh -c 'cd components/sup && cargo test --lib'
	$(run) sh -c 'cd components/depot && cargo test --lib'

functional: image ## executes the components' functional test suites
	$(run) sh -c 'cd components/core && cargo test --features functional'
	$(run) sh -c 'cd components/sup && cargo test --test functional'
	$(run) sh -c 'cd components/depot && cargo test --test server'

clean: ## cleans up the project tree
	$(run) sh -c 'cd components/builder-api && cargo clean'
	$(run) sh -c 'cd components/builder-dbcache && cargo clean'
	$(run) sh -c 'cd components/builder-protocol && cargo clean'
	$(run) sh -c 'cd components/builder-sessionsrv && cargo clean'
	$(run) sh -c 'cd components/builder-vault && cargo clean'
	$(run) sh -c 'cd components/common && cargo clean'
	$(run) sh -c 'cd components/core && cargo clean'
	$(run) sh -c 'cd components/depot-client && cargo clean'
	$(run) sh -c 'cd components/depot-core && cargo clean'
	$(run) sh -c 'cd components/depot && cargo clean'
	$(run) sh -c 'cd components/director && cargo clean'
	$(run) sh -c 'cd components/hab && cargo clean'
	$(run) sh -c 'cd components/net && cargo clean'
	$(run) sh -c 'cd components/sodiumoxide && cargo clean'
	$(run) sh -c 'cd components/sup && cargo clean'

help:
	@perl -nle'print $& if m{^[a-zA-Z_-]+:.*?## .*$$}' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

shell: image ## launches a development shell
	$(run)

serve-docs: docs ## serves the project documentation from an HTTP server
	@echo "==> View the docs at:\n\n        http://`\
		echo $(docs_host) | sed -e 's|^tcp://||' -e 's|:[0-9]\{1,\}$$||'`:9633/\n\n"
	$(docs_run) sh -c 'set -e; cd ./components/sup/target/doc; python -m SimpleHTTPServer 9633;'

ifneq ($(IN_DOCKER),)
distclean: ## fully cleans up project tree and any associated Docker images and containers
	$(compose_cmd) stop
	$(compose_cmd) rm -f -v
	$(docker_cmd) rmi $(dimage) || true
	($(docker_cmd) images -q -f dangling=true | xargs $(docker_cmd) rmi -f) || true

image: ## create an image
	if [ -n "${force}" -o -n "${refresh}" -o -z "`$(docker_cmd) images -q $(dimage)`" ]; then \
		if [ -n "${force}" ]; then \
		  $(docker_cmd) build --no-cache $(build_args) -t $(dimage) .; \
		else \
		  $(docker_cmd) build $(build_args) -t $(dimage) .; \
		fi \
	fi
else
image: ## no-op

distclean: clean ## fully cleans up project tree
endif

docs: image ## build the docs
	$(run) sh -c 'set -ex; \
		cd components/sup && cargo doc && cd ../../ \
		rustdoc --crate-name habitat_sup README.md -o ./components/sup/target/doc/habitat_sup; \
		docco -e .sh -o components/sup/target/doc/habitat_sup/hab-plan-build components/plan-build/bin/hab-plan-build.sh; \
		cp -r images ./components/sup/target/doc/habitat_sup; \
		echo "<meta http-equiv=refresh content=0;url=habitat_sup/index.html>" > components/sup/target/doc/index.html;'
