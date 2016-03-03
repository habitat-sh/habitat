ALL_PKGS := linux-headers glibc zlib file binutils m4 gmp mpfr libmpc gcc patchelf gcc-libs bzip2 pkg-config ncurses attr acl libcap sed shadow psmisc procps-ng coreutils bison flex pcre grep readline bash bc tar gawk libtool gdbm expat db inetutils iana-etc less perl diffutils autoconf automake findutils xz gettext gzip make patch texinfo util-linux tcl expect dejagnu check libidn cacerts openssl wget gnupg bash-static coreutils-static linux-headers-musl musl busybox-static gnupg-static jq-static bpm build vim libbsd clens mg backline bldr-studio
BOOTSTRAP_TOOLCHAIN_PKGS := linux-headers glibc zlib file binutils m4 gmp mpfr libmpc gcc patchelf-using-bootstrap-tools gcc-libs patchelf bzip2 pkg-config ncurses attr acl libcap sed shadow psmisc procps-ng coreutils bison flex pcre grep readline bash bc tar gawk libtool gdbm expat db inetutils iana-etc less perl diffutils autoconf automake findutils xz gettext gzip make patch texinfo util-linux tcl expect dejagnu check libidn cacerts openssl wget gnupg bash-static coreutils-static linux-headers-musl musl busybox-static gnupg-static jq-static bpm build vim libbsd clens mg backline bldr-studio
BLDR_PKGS := $(BOOTSTRAP_TOOLCHAIN_PKGS)
BLDR_WEB_PKGS := node ncurses libedit pcre nginx bldr-web
PKGS := $(BLDR_PKGS) $(BLDR_WEB_PKGS) redis haproxy libaio libltdl libxml2 numactl perl erlang libyaml libiconv libtool libffi ruby
REPO := http://52.11.158.96:32768

.PHONY: bldr-deps bldr-webui world publish gpg clean baseimage_root new-plan bootstrap-toolchain $(ALL_PKGS) $(addprefix publish-,$(ALL_PKGS)) $(addsuffix -using-bootstrap-tools,$(ALL_PKGS)) help
.DEFAULT_GOAL := help

help:
	@perl -nle'print $& if m{^[a-zA-Z_-]+:.*?## .*$$}' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

new-plan: ## create a new plan from the template, plan-tmpl.sh
	mkdir -p $(plan)
	sed 's/PACKAGE/$(plan)/g' plan-tmpl.sh > $(plan)/plan.sh

bootstrap-toolchain: gpg $(BOOTSTRAP_TOOLCHAIN_PKGS)
	mkdir -pv /opt/bldr/cache/keys
	cp ./chef-public.gpg /opt/bldr/cache/keys/chef-public.asc

world: gpg $(ALL_PKGS)
	mkdir -pv /opt/bldr/cache/keys
	cp ./chef-public.gpg /opt/bldr/cache/keys/chef-public.asc

bldr-deps: gpg $(BLDR_PKGS) ## build all of bldr's deps

bldr-webui: gpg $(BLDR_WEB_PKGS) ## build all of bldr-web's deps

publish: ## publish with cargo
	cargo build --release
	$(MAKE) $(addprefix publish-,$(PKGS))

$(ALL_PKGS): ## tell make how to build each package with bldr-buld
	./bldr-build $@

$(addsuffix -using-bootstrap-tools,$(ALL_PKGS)):
	env BOOTSTRAP_TOOLS=/tools $(MAKE) $(subst -using-bootstrap-tools,,$@)

$(addprefix publish-,$(ALL_PKGS)):
	../target/release/bldr upload chef/$(subst publish-,,$@) -u $(REPO)

gpg: ## setup the GPG keys
	mkdir -pv /opt/bldr/cache/gpg
	- gpg --import chef-public.gpg
	- gpg --import chef-private.gpg
	- gpg --homedir /opt/bldr/cache/gpg --import chef-public.gpg
	- gpg --homedir /opt/bldr/cache/gpg --import chef-private.gpg

clean: ## clean all the packages from the build environment
	rm -rf /opt/bldr/pkgs/*

baseimage_root: ## create a base image root filesystem of bldr packages
	pushd / && \
	tar jcvf /src/bldr-base/bldr-base.tar.bz2 \
		/opt/bldr/pkgs/chef/glibc \
		/opt/bldr/pkgs/chef/libgcc \
		/opt/bldr/pkgs/chef/zlib \
		/opt/bldr/pkgs/chef/xz \
		/opt/bldr/pkgs/chef/cacerts \
		/opt/bldr/pkgs/chef/busybox \
		/opt/bldr/pkgs/chef/gnupg \
		/opt/bldr/pkgs/chef/openssl \
		/opt/bldr/pkgs/chef/rngd \
		/opt/bldr/pkgs/chef/bldr \
		&& \
	popd
