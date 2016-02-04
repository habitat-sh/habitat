build_args := --build-arg BLDR_REPO=$(BLDR_REPO)
run_args := -e BLDR_REPO=$(BLDR_REPO)
ifneq (${http_proxy},)
	_http_proxy := http_proxy="${http_proxy}"
	build_args := $(build_args) --build-arg $(_http_proxy)
	run_args := $(run_args) -e $(_http_proxy)
endif
ifneq (${https_proxy},)
	_https_proxy := https_proxy="${https_proxy}"
	build_args := $(build_args) --build-arg $(_https_proxy)
	run_args := $(run_args) -e $(_https_proxy)
endif

ifneq (${STUDIO_TYPE},)
	build_args := $(build_args) --build-arg STUDIO_TYPE=$(STUDIO_TYPE)
	dimage := bldr/studio:$(STUDIO_TYPE)
	image_tag := :$(STUDIO_TYPE)
else
	dimage := bldr/studio:bldr
	image_tag := :bldr
endif

docker_cmd := env http_proxy= https_proxy= docker
compose_cmd := env http_proxy= https_proxy= IMAGE_TAG=$(image_tag) docker-compose

docker-studio: docker-studio-image
	$(compose_cmd) run --rm $(run_args) instance

docker-studio-clean:
	$(compose_cmd) stop
	$(compose_cmd) rm -f -v
	$(docker_cmd) rmi $(dimage) || true

docker-studio-image:
	if [ -n "${force}" -o -z "`$(docker_cmd) images -q $(dimage)`" ]; then \
		$(docker_cmd) build $(build_args) -t $(dimage) .; \
	fi

vagrant-studio: vagrant-studio-vm
	vagrant ssh

vagrant-studio-clean:
	vagrant destroy -f

vagrant-studio-vm:
	vagrant up
