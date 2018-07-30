UNAME_S := $(shell uname -s)
HAS_DOCKER := $(shell command -v docker 2> /dev/null)
ifneq (${IN_DOCKER},)
	IN_DOCKER := ${IN_DOCKER}
else ifeq ($(UNAME_S),Darwin)
	IN_DOCKER := true
endif

ifeq ($(IN_DOCKER),true)
	build_args := --build-arg HAB_BLDR_URL=$(HAB_BLDR_URL)
	run_args := -e HAB_BLDR_URL=$(HAB_BLDR_URL)
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
	docs_run := $(common_run) -p 9633:9633 shell
else
	run :=
	docs_run :=
endif
ifneq ($(DOCKER_HOST),)
	docs_host := ${DOCKER_HOST}
else
	docs_host := 127.0.0.1
endif
ifeq (${CI},true)
	CARGO_FLAGS := --no-default-features
else
	CARGO_FLAGS :=
endif

# launcher is intentionally omitted from the standard build process
# see https://github.com/habitat-sh/habitat/blob/master/components/launcher/README.md
BIN = hab pkg-export-docker pkg-export-kubernetes sup
LIB = butterfly common builder-depot-client sup-protocol sup-client
ALL = $(BIN) $(LIB)
VERSION := $(shell cat VERSION)

.DEFAULT_GOAL := build-bin

build: build-bin build-lib
build-all: build
.PHONY: build build-all

build-bin: $(addprefix build-,$(BIN)) ## builds the binary components
.PHONY: build-bin

build-lib: $(addprefix build-,$(LIB)) ## builds the library components
.PHONY: build-lib

unit: unit-bin unit-lib
unit-all: unit
.PHONY: unit unit-all

unit-bin: $(addprefix unit-,$(BIN)) ## executes the binary components' unit test suites
.PHONY: unit-bin

unit-lib: $(addprefix unit-,$(LIB)) ## executes the library components' unit test suites
.PHONY: unit-lib

lint: lint-bin lint-lib
lint-all: lint
.PHONY: lint lint-all

lint-bin: $(addprefix lint-,$(BIN))
.PHONY: lint-bin

lint-lib: $(addprefix lint-,$(LIB))
.PHONY: lint-lib

functional: functional-bin functional-lib
functional-all: functional
test: functional ## executes all components' test suites
.PHONY: functional functional-all test

functional-bin: $(addprefix unit-,$(BIN)) ## executes the binary components' unit functional suites
.PHONY: functional-bin

functional-lib: $(addprefix unit-,$(LIB)) ## executes the library components' unit functional suites
.PHONY: functional-lib

clean: clean-bin clean-lib
clean-all: clean
.PHONY: clean clean-all

clean-bin: $(addprefix clean-,$(BIN)) ## cleans the binary components' project trees
.PHONY: clean-bin

clean-lib: $(addprefix clean-,$(LIB)) ## cleans the library components' project trees
.PHONY: clean-lib

fmt: fmt-bin fmt-lib
fmt-all: fmt
.PHONY: fmt fmt-all

fmt-bin: $(addprefix fmt-,$(BIN)) ## formats the binary components' codebases
.PHONY: clean-bin

fmt-lib: $(addprefix fmt-,$(LIB)) ## formats the library components' codebases
.PHONY: clean-lib

help:
	@perl -nle'print $& if m{^[a-zA-Z_-]+:.*?## .*$$}' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
.PHONY: help

shell: image ## launches a development shell
	$(run)
.PHONY: shell

serve-docs: docs ## serves the project documentation from an HTTP server
	@echo "==> View the docs at:\n\n        http://`\
		echo $(docs_host) | sed -e 's|^tcp://||' -e 's|:[0-9]\{1,\}$$||'`:9633/\n\n"
	$(docs_run) sh -c 'set -e; cd ./target/doc; python -m SimpleHTTPServer 9633;'
.PHONY: serve-docs

ifeq ($(IN_DOCKER),true)
distclean: ## fully cleans up project tree and any associated Docker images and containers
	$(compose_cmd) stop
	$(compose_cmd) rm -f -v
	$(docker_cmd) rmi $(dimage) || true
	($(docker_cmd) images -q -f dangling=true | xargs $(docker_cmd) rmi -f) || true
.PHONY: distclean

image: ## create an image
ifeq ($(HAS_DOCKER),)
	$(error "Docker does not seem installed, please install docker first.")
endif
	@if [ -n "${force}" -o -n "${refresh}" -o -z "`$(docker_cmd) images -q $(dimage)`" ]; then \
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

changelog: image
	@$(run) sh -c 'hab pkg install core/github_changelog_generator && \
		hab pkg binlink core/git git --force && \
		hab pkg binlink core/github_changelog_generator github_changelog_generator --force && \
		github_changelog_generator --future-release $(VERSION) --token $(GITHUB_TOKEN) --max-issues=1000'

docs: image ## build the docs
	$(run) sh -c 'set -ex; \
		cd components/sup && cargo doc && cd ../../ \
		rustdoc --crate-name habitat_sup README.md -o ./target/doc/habitat_sup; \
		docco -e .sh -o target/doc/habitat_sup/hab-plan-build components/plan-build/bin/hab-plan-build.sh; \
		cp -r images ./target/doc/habitat_sup; \
		echo "<meta http-equiv=refresh content=0;url=habitat_sup/index.html>" > target/doc/index.html;'

tag-release:
	sh -c 'git tag $(VERSION)'

define BUILD
build-$1: image ## builds the $1 component
	$(run) sh -c 'cd components/$1 && cargo build $(CARGO_FLAGS)'
.PHONY: build-$1

endef
$(foreach component,$(ALL),$(eval $(call BUILD,$(component))))

define UNIT
unit-$1: image ## executes the $1 component's unit test suite
	$(run) sh -c 'cd components/$1 && cargo test $(CARGO_FLAGS)'
.PHONY: unit-$1
endef
$(foreach component,$(ALL),$(eval $(call UNIT,$(component))))

# Here we just add a dependency on the hab-launch binary for the
# Supervisor (integration) tests
build-launcher-for-supervisor-tests:
	$(run) sh -c 'cd components/launcher && cargo build --bin=hab-launch $(CARGO_FLAGS)'
unit-sup: build-launcher-for-supervisor-tests
.PHONY: build-launcher-for-supervisor-tests

define LINT
lint-$1: image ## executes the $1 component's linter checks
	$(run) sh -c 'cd components/$1 && cargo build --features clippy $(CARGO_FLAGS)'
.PHONY: lint-$1
endef
$(foreach component,$(ALL),$(eval $(call LINT,$(component))))

define FUNCTIONAL
functional-$1: image ## executes the $1 component's functional test suite
	$(run) sh -c 'cd components/$1 && cargo test --features functional $(CARGO_FLAGS)'
.PHONY: functional-$1

endef
$(foreach component,$(ALL),$(eval $(call FUNCTIONAL,$(component))))

define CLEAN
clean-$1: image ## cleans the $1 component's project tree
	$(run) sh -c 'cd components/$1 && cargo clean'
.PHONY: clean-$1

endef
$(foreach component,$(ALL),$(eval $(call CLEAN,$(component))))

define FMT
fmt-$1: image ## formats the $1 component
	$(run) sh -c 'cd components/$1 && cargo fmt'
.PHONY: fmt-$1

endef
$(foreach component,$(ALL),$(eval $(call FMT,$(component))))

# Run BATS integration tests in a Docker "cleanroom" container.
bats: build-hab build-sup build-launcher-for-supervisor-tests
	./run-bats.sh
