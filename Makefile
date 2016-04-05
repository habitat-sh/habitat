UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
	IN_DOCKER := true
endif

ifneq ($(IN_DOCKER),)
	build_args := --build-arg BLDR_REPO=$(BLDR_REPO)
	run_args := -e BLDR_REPO=$(BLDR_REPO)
	ifneq (${http_proxy},)
		build_args := $(build_args) --build-arg http_proxy="${http_proxy}"
		run_args := $(run_args) -e http_proxy="${http_proxy}"
	endif
	ifneq (${https_proxy},)
		build_args := $(build_args) --build-arg https_proxy="${https_proxy}"
		run_args := $(run_args) -e https_proxy="${https_proxy}"
	endif

	dimage := bldr/devshell
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

.PHONY: help all shell serve-docs test unit functional clean image docs gpg
.DEFAULT_GOAL := all

all: image ## builds the project's Rust components
	$(run) cargo build --manifest-path components/hab/Cargo.toml
	$(run) cargo build --manifest-path components/sup/Cargo.toml
	$(run) cargo build --manifest-path components/depot/Cargo.toml

test: image ## tests the project's Rust components
	$(run) cargo test --manifest-path components/core/Cargo.toml
	$(run) cargo test --manifest-path components/depot-core/Cargo.toml
	$(run) cargo test --manifest-path components/depot-client/Cargo.toml
	$(run) cargo test --manifest-path components/sup/Cargo.toml
	$(run) cargo test --manifest-path components/depot/Cargo.toml

unit: image ## executes the components' unit test suites
	$(run) cargo test --lib --manifest-path components/core/Cargo.toml
	$(run) cargo test --lib --manifest-path components/depot-core/Cargo.toml
	$(run) cargo test --lib --manifest-path components/depot-client/Cargo.toml
	$(run) cargo test --lib --manifest-path components/sup/Cargo.toml
	$(run) cargo test --lib --manifest-path components/depot/Cargo.toml

functional: image ## executes the components' functional test suites
	$(run) cargo test --test functional --manifest-path components/sup/Cargo.toml
	$(run) cargo test --test server --manifest-path components/depot/Cargo.toml

clean: ## cleans up the project tree
	$(run) cargo clean --manifest-path components/core/Cargo.toml
	$(run) cargo clean --manifest-path components/depot-core/Cargo.toml
	$(run) cargo clean --manifest-path components/depot-client/Cargo.toml
	$(run) cargo clean --manifest-path components/sup/Cargo.toml
	$(run) cargo clean --manifest-path components/depot/Cargo.toml

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
		cargo doc --manifest-path components/sup/Cargo.toml; \
		rustdoc --crate-name habitat_sup README.md -o ./components/sup/target/doc/habitat_sup; \
		docco -e .sh -o components/sup/target/doc/habitat_sup/bldr-build plans/bldr-build; \
		cp -r images ./components/sup/target/doc/habitat_sup; \
		echo "<meta http-equiv=refresh content=0;url=habitat_sup/index.html>" > components/sup/target/doc/index.html;'

gpg: ## installs gpg signing keys, only run this in a Studio
	(cd plans && make gpg)
