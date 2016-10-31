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
	api_run := $(common_run) -p 9636:9636 shell
	docs_host := ${DOCKER_HOST}
	docs_run := $(common_run) -p 9633:9633 shell
else
	run :=
	docs_host := 127.0.0.1
	docs_run :=
endif

BIN = director hab sup
LIB = builder-dbcache builder-protocol common core depot-client http-client net
SRV = builder-api builder-jobsrv builder-sessionsrv builder-vault builder-worker depot
ALL = $(BIN) $(LIB) $(SRV)

.DEFAULT_GOAL := build-bin

build: build-bin build-lib build-srv ## builds all the components
build-all: build
.PHONY: build build-all

build-bin: $(addprefix build-,$(BIN)) ## builds the binary components
.PHONY: build-bin

build-lib: $(addprefix build-,$(LIB)) ## builds the library components
.PHONY: build-lib

build-srv: $(addprefix build-,$(SRV)) ## builds the service components
.PHONY: build-srv

unit: unit-bin unit-lib unit-srv ## executes all the components' unit test suites
unit-all: unit
.PHONY: unit unit-all

unit-bin: $(addprefix unit-,$(BIN)) ## executes the binary components' unit test suites
.PHONY: unit-bin

unit-lib: $(addprefix unit-,$(LIB)) ## executes the library components' unit test suites
.PHONY: unit-lib

unit-srv: $(addprefix unit-,$(SRV)) ## executes the service components' unit test suites
.PHONY: unit-srv

functional: functional-bin functional-lib functional-srv ## executes all the components' functional test suites
functional-all: functional
test: functional ## executes all components' test suites
.PHONY: functional functional-all test

functional-bin: $(addprefix unit-,$(BIN)) ## executes the binary components' unit functional suites
.PHONY: functional-bin

functional-lib: $(addprefix unit-,$(LIB)) ## executes the library components' unit functional suites
.PHONY: functional-lib

functional-srv: $(addprefix unit-,$(SRV)) ## executes the service components' unit functional suites
.PHONY: functional-srv

clean: clean-bin clean-lib clean-srv ## cleans all the components' clean test suites
clean-all: clean
.PHONY: clean clean-all

clean-bin: $(addprefix clean-,$(BIN)) ## cleans the binary components' project trees
.PHONY: clean-bin

clean-lib: $(addprefix clean-,$(LIB)) ## cleans the library components' project trees
.PHONY: clean-lib

clean-srv: $(addprefix clean-,$(SRV)) ## cleans the service components' project trees
.PHONY: clean-srv

help:
	@perl -nle'print $& if m{^[a-zA-Z_-]+:.*?## .*$$}' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
.PHONY: help

api-shell: image ## launches a development shell running the API
	$(api_run) sh -c 'cp /src/support/run-api.sh /usr/local/sbin/api; api start; bash'
.PHONY: api-shell

shell: image ## launches a development shell
	$(run)
.PHONY: shell

serve-docs: docs ## serves the project documentation from an HTTP server
	@echo "==> View the docs at:\n\n        http://`\
		echo $(docs_host) | sed -e 's|^tcp://||' -e 's|:[0-9]\{1,\}$$||'`:9633/\n\n"
	$(docs_run) sh -c 'set -e; cd ./components/sup/target/doc; python -m SimpleHTTPServer 9633;'
.PHONY: serve-docs

ifneq ($(IN_DOCKER),)
distclean: ## fully cleans up project tree and any associated Docker images and containers
	$(compose_cmd) stop
	$(compose_cmd) rm -f -v
	$(docker_cmd) rmi $(dimage) || true
	($(docker_cmd) images -q -f dangling=true | xargs $(docker_cmd) rmi -f) || true
.PHONY: distclean

image: ## create an image
	if [ -n "${force}" -o -n "${refresh}" -o -z "`$(docker_cmd) images -q $(dimage)`" ]; then \
		if [ -n "${force}" ]; then \
		  $(docker_cmd) build --no-cache $(build_args) -t $(dimage) .; \
		else \
		  $(docker_cmd) build $(build_args) -t $(dimage) .; \
		fi \
	fi
.PHONY: image
else
image: ## no-op
.PHONY: image

distclean: clean ## fully cleans up project tree
.PHONY: distclean
endif

docs: image ## build the docs
	$(run) sh -c 'set -ex; \
		cd components/sup && cargo doc && cd ../../ \
		rustdoc --crate-name habitat_sup README.md -o ./components/sup/target/doc/habitat_sup; \
		docco -e .sh -o components/sup/target/doc/habitat_sup/hab-plan-build components/plan-build/bin/hab-plan-build.sh; \
		cp -r images ./components/sup/target/doc/habitat_sup; \
		echo "<meta http-equiv=refresh content=0;url=habitat_sup/index.html>" > components/sup/target/doc/index.html;'

define BUILD
build-$1: image ## builds the $1 component
	$(run) sh -c 'cd components/$1 && cargo build'
.PHONY: build-$1

endef
$(foreach component,$(ALL),$(eval $(call BUILD,$(component))))

define UNIT
unit-$1: image ## executes the $1 component's unit test suite
	$(run) sh -c 'cd components/$1 && cargo test'
.PHONY: unit-$1

endef
$(foreach component,$(ALL),$(eval $(call UNIT,$(component))))

define FUNCTIONAL
functional-$1: image ## executes the $1 component's functional test suite
	$(run) sh -c 'cd components/$1 && cargo test --features functional'
.PHONY: functional-$1

endef
$(foreach component,$(ALL),$(eval $(call FUNCTIONAL,$(component))))

define CLEAN
clean-$1: image ## cleans the $1 component's project tree
	$(run) sh -c 'cd components/$1 && cargo clean'
.PHONY: clean-$1

endef
$(foreach component,$(ALL),$(eval $(call CLEAN,$(component))))
