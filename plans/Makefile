BLDR_PKGS := linux-headers glibc zlib file binutils m4 gmp mpfr libmpc
BLDR_WEB_PKGS := node ncurses libedit pcre nginx bldr-web
PKGS := $(BLDR_PKGS) $(BLDR_WEB_PKGS) redis haproxy libaio libltdl libxml2 numactl perl erlang libyaml libiconv libtool libffi ruby
REPO := http://159.203.235.47

.PHONY: bldr-deps bldr-webui world publish gpg clean baseimage_root $(PKGS) $(addprefix publish-,$(PKGS)) new-plan

new-plan:
	mkdir -p $(plan)
	sed 's/PACKAGE/$(plan)/g' plan-tmpl.sh > $(plan)/plan.sh

world: gpg $(PKGS)
	mkdir -pv /opt/bldr/cache/keys
	cp ./chef-public.gpg /opt/bldr/cache/keys/chef-public.asc

bldr-deps: gpg $(BLDR_PKGS)

bldr-webui: gpg $(BLDR_WEB_PKGS)

publish:
	cargo build --release
	$(MAKE) $(addprefix publish-,$(PKGS))

$(PKGS):
	./bldr-build $@

$(addprefix publish-,$(PKGS)):
	../target/release/bldr upload chef/$(subst publish-,,$@) -u $(REPO)

gpg:
	mkdir -pv /opt/bldr/cache/gpg
	- gpg --import chef-public.gpg
	- gpg --import chef-private.gpg
	- gpg --homedir /opt/bldr/cache/gpg --import chef-public.gpg
	- gpg --homedir /opt/bldr/cache/gpg --import chef-private.gpg

clean:
	rm -rf /opt/bldr/pkgs/*

baseimage_root:
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
