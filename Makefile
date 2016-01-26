build_args :=
run_args :=
ifneq (${docker_http_proxy},)
	_http_proxy := http_proxy="${docker_http_proxy}"
	build_args := $(build_args) --build-arg $(_http_proxy)
	run_args := $(run_args) -e $(_http_proxy)
endif
ifneq (${docker_https_proxy},)
	_https_proxy := https_proxy="${docker_https_proxy}"
	build_args := $(build_args) --build-arg $(_https_proxy)
	run_args := $(run_args) -e $(_https_proxy)
endif

run := docker-compose run --rm $(run_args)
IMAGE := chef/bldr
VOLUMES := installed cache_pkgs cache_src cache_keys cargo
CLEAN_VOLUMES := clean-installed clean-cache_pkgs clean-cache_src clean-cache_keys clean-cargo
NO_CACHE := false

.PHONY: image test run shell clean bldr-base clean-package package volumes clean-volumes all

all: package

atest:
ifeq ($(GITHUB_DEPLOY_KEY),)
	@echo "nothing to see here"
else
	@echo "$$GITHUB_DEPLOY_KEY"
	@echo "SHASUM: $(DELIVERY_GIT_SHASUM)"
endif

package: image
ifeq ($(GITHUB_DEPLOY_KEY),)
	$(run) package sh -c '(cd /src/plans && make world)'
else
	$(run) package sh -c "mkdir -p ~/.ssh; echo \"$${GITHUB_DEPLOY_KEY}\" > ~/.ssh/id_rsa_bldr_github; chmod 0600 ~/.ssh/id_rsa_bldr_github; chmod +x /usr/local/bin/ssh_wrapper.sh; ([ -d /src/plans ] && true || (cd / && GIT_SSH=/usr/local/bin/ssh_wrapper.sh git clone git@github.com:chef/bldr.git /src)) && git checkout $(DELIVERY_GIT_SHASUM) && (cd /src/plans && make world)"
endif

clean-package: image
	$(run) package sh -c 'rm -rf /opt/bldr/cache/pkgs/* /opt/bldr/pkgs/*'

volumes: $(VOLUMES)

$(VOLUMES):
	docker-compose up -d $@

clean-volumes: $(CLEAN_VOLUMES)

$(CLEAN_VOLUMES):
	docker-compose rm -f `echo $@ | sed 's/^clean-//'`

image:
	if [ -n "${force}" -o -z "`docker images -q $(IMAGE)`" ]; then docker build $(build_args) -t $(IMAGE) --no-cache=${NO_CACHE} .; fi

test: image
	$(run) package cargo test

unit: image
	$(run) package cargo test --lib

functional: image
	$(run) package cargo test --test functional

cargo-clean: image
	$(run) package cargo clean

docs: image
	$(run) package sh -c 'set -ex; \
		cargo doc; \
		rustdoc --crate-name bldr README.md -o ./target/doc/bldr; \
		docco -e .sh -o target/doc/bldr/bldr-build plans/bldr-build; \
		cp -r images ./target/doc/bldr; \
		echo "<meta http-equiv=refresh content=0;url=bldr/index.html>" > target/doc/index.html;'

doc-serve: image
	@echo "==> View the docs at:\n\n        http://`\
		echo ${DOCKER_HOST} | sed -e 's|^tcp://||' -e 's|:[0-9]\{1,\}$$||'`:9633/\n\n"
	$(run) -p 9633:9633 package sh -c 'set -e; cd ./target/doc; python -m SimpleHTTPServer 9633;'

repo-serve: image
	$(run) --service-ports repo cargo run -- repo

shell: image
	$(run) bldr bash

pkg-shell: image
	$(run) package bash

bldr-base: package

base-shell: image
	$(run) base

clean:
	docker-compose kill
	docker-compose rm -f -v
	(docker images -q -f dangling=true | xargs docker rmi -f) || true

gpg:
	mkdir -p /opt/bldr/cache/gpg
	- gpg --import /src/plans/chef-public.gpg
	- gpg --import /src/plans/chef-private.gpg
	- gpg --homedir /opt/bldr/cache/gpg --import /src/plans/chef-public.gpg
	- gpg --homedir /opt/bldr/cache/gpg --import /src/plans/chef-private.gpg

redis:
	$(run) bldr cargo run -- start chef/redis

publish:
	for x in `docker images | egrep '^bldr/base' | awk '{print $2}'`; do \
		docker tag -f bldr/base:$x quay.io/bldr/base:$x ; \
	done
	docker push quay.io/bldr/base
