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

.PHONY: build shell docs-serve test unit functional clean image docs

build: image
	$(run) shell cargo build

shell: image
	$(run) shell

docs-serve: docs
	@echo "==> View the docs at:\n\n        http://`\
		echo ${DOCKER_HOST} | sed -e 's|^tcp://||' -e 's|:[0-9]\{1,\}$$||'`:9633/\n\n"
	$(run) -p 9633:9633 shell sh -c 'set -e; cd ./target/doc; python -m SimpleHTTPServer 9633;'

test: image
	$(run) shell cargo test

unit: image
	$(run) shell cargo test --lib

functional: image
	$(run) shell cargo test --test functional

clean:
	rm -rf target/debug target/release
	$(compose_cmd) stop
	$(compose_cmd) rm -f -v
	$(docker_cmd) rmi $(dimage) || true
	($(docker_cmd) images -q -f dangling=true | xargs $(docker_cmd) rmi -f) || true

image:
	if [ -n "${force}" -o -z "`$(docker_cmd) images -q $(dimage)`" ]; then \
		$(docker_cmd) build $(build_args) -t $(dimage) .; \
	fi

docs: image
	$(run) shell sh -c 'set -ex; \
		cargo doc; \
		rustdoc --crate-name bldr README.md -o ./target/doc/bldr; \
		docco -e .sh -o target/doc/bldr/bldr-build plans/bldr-build; \
		cp -r images ./target/doc/bldr; \
		echo "<meta http-equiv=refresh content=0;url=bldr/index.html>" > target/doc/index.html;'

# Alias to `make shell` for the "old fingers" crowd
pkg-shell: shell
