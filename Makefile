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

docker_cmd := env http_proxy= https_proxy= docker
compose_cmd := env http_proxy= https_proxy= docker-compose
run := $(compose_cmd) run --rm $(run_args)
dimage := bldr/devshell

.PHONY: build shell docs-serve test unit functional clean image docs help
.DEFAULT_GOAL := help

help:
	@perl -nle'print $& if m{^[a-zA-Z_-]+:.*?## .*$$}' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

build: image ## run cargo build
	$(run) shell cargo build

shell: image ## start a shell for building packages
	$(run) shell

docs-serve: docs ## serve up the documentation
	@echo "==> View the docs at:\n\n        http://`\
		echo ${DOCKER_HOST} | sed -e 's|^tcp://||' -e 's|:[0-9]\{1,\}$$||'`:9633/\n\n"
	$(run) -p 9633:9633 shell sh -c 'set -e; cd ./target/doc; python -m SimpleHTTPServer 9633;'

test: image ## run `cargo test`
	$(run) shell cargo test

unit: image ## run unit tests with cargo
	$(run) shell cargo test --lib

functional: image ## run the functional tests
	$(run) shell cargo test --test functional

clean: ## clean up our docker environment
	rm -rf target/debug target/release
	$(compose_cmd) stop
	$(compose_cmd) rm -f -v
	$(docker_cmd) rmi $(dimage) || true
	($(docker_cmd) images -q -f dangling=true | xargs $(docker_cmd) rmi -f) || true

image: ## create an image
	if [ -n "${force}" -o -z "`$(docker_cmd) images -q $(dimage)`" ]; then \
		if [ -n "${force}" ]; then \
		  $(docker_cmd) build --no-cache $(build_args) -t $(dimage) .; \
		else \
		  $(docker_cmd) build $(build_args) -t $(dimage) .; \
		fi \
	fi

docs: image ## build the docs
	$(run) shell sh -c 'set -ex; \
		cargo doc; \
		rustdoc --crate-name bldr README.md -o ./target/doc/bldr; \
		docco -e .sh -o target/doc/bldr/bldr-build plans/bldr-build; \
		cp -r images ./target/doc/bldr; \
		echo "<meta http-equiv=refresh content=0;url=bldr/index.html>" > target/doc/index.html;'

pkg-shell: shell ## Alias to `make shell` for the "old fingers" crowd
