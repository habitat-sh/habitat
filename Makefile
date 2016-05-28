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

.PHONY: all test unit functional clean bin unit-bin functional-bin clean-bin lib unit-lib functional-lib clean-lib srv unit-srv functional-srv clean-srv help shell serve-docs distclean image docs
.DEFAULT_GOAL := bin

all: image ## builds all the project's Rust components
	$(MAKE) bin
	$(MAKE) lib
	$(MAKE) srv

test: image ## executes the Rust components' test suites
	$(MAKE) functional

unit: image ## executes the components' unit test suites
	$(MAKE) unit-bin
	$(MAKE) unit-lib
	$(MAKE) unit-srv

functional: image ## executes the components' functional test suites
	$(MAKE) functional-bin
	$(MAKE) functional-lib
	$(MAKE) functional-srv

clean: ## cleans up the project tree
	$(MAKE) clean-bin
	$(MAKE) clean-lib
	$(MAKE) clean-srv

bin: image ## builds the project's main binaries
	$(run) sh -c 'cd components/director && cargo build'
	$(run) sh -c 'cd components/hab && cargo build'
	$(run) sh -c 'cd components/sup && cargo build'

unit-bin: ## executes binary components' unit test suites
	$(run) sh -c 'cd components/director && cargo test'
	$(run) sh -c 'cd components/hab && cargo test'
	$(run) sh -c 'cd components/sup && cargo test'

functional-bin: image ## executes binary component's function test suites
	$(run) sh -c 'cd components/director && cargo test --features functional'
	$(run) sh -c 'cd components/hab && cargo test --features functional'
	$(run) sh -c 'cd components/sup && cargo test --features functional'

clean-bin: ## cleans binary components' project trees
	$(run) sh -c 'cd components/director && cargo clean'
	$(run) sh -c 'cd components/hab && cargo clean'
	$(run) sh -c 'cd components/sup && cargo clean'

lib: image ## builds the project's library components
	$(run) sh -c 'cd components/builder-dbcache && cargo build'
	$(run) sh -c 'cd components/builder-protocol && cargo build'
	$(run) sh -c 'cd components/common && cargo build'
	$(run) sh -c 'cd components/core && cargo build'
	$(run) sh -c 'cd components/depot-client && cargo build'
	$(run) sh -c 'cd components/depot-core && cargo build'
	$(run) sh -c 'cd components/net && cargo build'

unit-lib: ## executes library components' unit test suites
	$(run) sh -c 'cd components/builder-dbcache && cargo test'
	$(run) sh -c 'cd components/builder-protocol && cargo test'
	$(run) sh -c 'cd components/common && cargo test'
	$(run) sh -c 'cd components/core && cargo test'
	$(run) sh -c 'cd components/depot-client && cargo test'
	$(run) sh -c 'cd components/depot-core && cargo test'
	$(run) sh -c 'cd components/net && cargo test'

functional-lib: image ## executes library component's function test suites
	$(run) sh -c 'cd components/builder-dbcache && cargo test --features functional'
	$(run) sh -c 'cd components/builder-protocol && cargo test --features functional'
	$(run) sh -c 'cd components/common && cargo test --features functional'
	$(run) sh -c 'cd components/core && cargo test --features functional'
	$(run) sh -c 'cd components/depot-client && cargo test --features functional'
	$(run) sh -c 'cd components/depot-core && cargo test --features functional'
	$(run) sh -c 'cd components/net && cargo test --features functional'

clean-lib: ## cleans library components' project trees
	$(run) sh -c 'cd components/builder-dbcache && cargo clean'
	$(run) sh -c 'cd components/builder-protocol && cargo clean'
	$(run) sh -c 'cd components/common && cargo clean'
	$(run) sh -c 'cd components/core && cargo clean'
	$(run) sh -c 'cd components/depot-client && cargo clean'
	$(run) sh -c 'cd components/depot-core && cargo clean'
	$(run) sh -c 'cd components/net && cargo clean'

srv: image ## builds the project's service components
	$(run) sh -c 'cd components/builder-api && cargo build'
	$(run) sh -c 'cd components/builder-jobsrv && cargo build'
	$(run) sh -c 'cd components/builder-sessionsrv && cargo build'
	$(run) sh -c 'cd components/builder-vault && cargo build'
	$(run) sh -c 'cd components/builder-worker && cargo build'
	$(run) sh -c 'cd components/depot && cargo build'

unit-srv: image ## executes service components' unit test suites
	$(run) sh -c 'cd components/builder-api && cargo test'
	$(run) sh -c 'cd components/builder-jobsrv && cargo test'
	$(run) sh -c 'cd components/builder-sessionsrv && cargo test'
	$(run) sh -c 'cd components/builder-vault && cargo test'
	$(run) sh -c 'cd components/builder-worker && cargo test'
	$(run) sh -c 'cd components/depot && cargo test'

functional-srv: image ## executes service component's function test suites
	$(run) sh -c 'cd components/builder-api && cargo test --features functional'
	$(run) sh -c 'cd components/builder-jobsrv && cargo test --features functional'
	$(run) sh -c 'cd components/builder-sessionsrv && cargo test --features functional'
	$(run) sh -c 'cd components/builder-vault && cargo test --features functional'
	$(run) sh -c 'cd components/builder-worker && cargo test --features functional'
	$(run) sh -c 'cd components/depot && cargo test --features functional'

clean-srv: ## cleans service components' project trees
	$(run) sh -c 'cd components/builder-api && cargo clean'
	$(run) sh -c 'cd components/builder-jobsrv && cargo clean'
	$(run) sh -c 'cd components/builder-sessionsrv && cargo clean'
	$(run) sh -c 'cd components/builder-vault && cargo clean'
	$(run) sh -c 'cd components/builder-worker && cargo clean'
	$(run) sh -c 'cd components/depot && cargo clean'

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
