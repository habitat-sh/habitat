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

ifneq (${CHROOT},)
	CHROOT := ${CHROOT}
else
	CHROOT := /mnt/chroot
endif
ifneq (${PLANS},)
	PLANS := ${PLANS}
else
	PLANS := `pwd`/../..
endif

IMAGE := chroot_build
TOOLS_TAR := lfs-tools/lfs-tools-20151209232702.tar.xz
TOOLS_EXTRA_TAR := lfs-tools/lfs-tools-extras-20151202003003.tar.xz
TARS := $(TOOLS_TAR) $(TOOLS_EXTRA_TAR)

ifneq (${use_bootstrap_tools},)
	TOOLS := `pwd`/$(TOOLS_TAR)
	TOOLS_EXTRA := `pwd`/$(TOOLS_EXTRA_TAR)
else
	TOOLS :=
	TOOLS_EXTRA :=
endif

image:
	if [ -n "${force}" -o -z "`docker images -q $(IMAGE)`" ]; then \
		make download-bootstrap-tools; \
		docker build $(build_args) \
			--build-arg use_bootstrap_tools=$(use_bootstrap_tools) \
			-t $(IMAGE):layers .; \
		id=`docker run -d $(IMAGE):layers sh`; \
		echo "==> Flattening $(IMAGE):layers to $(IMAGE):squash..."; \
		docker export $$id \
			| docker import -m "Squashed from $(IMAGE):layers" - $(IMAGE):squash; \
		docker rm -f $$id; \
		docker rmi -f $(IMAGE):layers; \
		docker build $(build_args) -t $(IMAGE) docker-final; \
		docker rmi -f $(IMAGE):squash; \
	fi

shell: image
	docker-compose run --rm $(run_args) build

clean-data:
	docker-compose rm -f -v data

vm:
	make download-bootstrap-tools
	time (env use_bootstrap_tools=${use_bootstrap_tools} vagrant up)

vm-shell: vm
	vagrant ssh

vm-destroy:
	vagrant destroy -f

chroot: ensure-linux
	make download-bootstrap-tools
	sudo -i env CHROOT=$(CHROOT) PLANS=$(PLANS) NO_MOUNT=${NO_MOUNT} \
		TOOLS=$(TOOLS) TOOLS_EXTRA=$(TOOLS_EXTRA) \
		`pwd`/scripts/create_chroot.sh

chroot-shell: chroot
	sudo -i env CHROOT=$(CHROOT) `pwd`/scripts/enter_chroot.sh

chroot-destroy: ensure-linux
	sudo -i env CHROOT=$(CHROOT) `pwd`/scripts/umount_filesystems.sh
	sudo -i rm -rfv $(CHROOT)

download-bootstrap-tools:
	mkdir -pv lfs-tools
	if [ -n "${use_bootstrap_tools}" ]; then make $(TARS); fi

$(TARS):
	mkdir -pv `dirname $@`
	http_proxy=${docker_http_proxy} curl -L -o "$@" \
		"http://s3-us-west-2.amazonaws.com/fnichol-lfs-tools/`basename $@`"

ensure-linux:
	@if [ `uname -s` != "Linux" ]; then \
		echo ">>> Only for running on Linux systems, aborting"; \
		exit 1; \
	fi
