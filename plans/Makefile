BLDR_PKGS := glibc libgcc patchelf zlib cacerts busybox libgpg-error libassuan gnupg gpgme openssl runit bldr
PKGS := $(BLDR_PKGS) redis ncurses libedit bzip2 pcre nginx haproxy libaio libltdl libxml2 numactl perl
REPO := http://159.203.235.47

.PHONY: bldr-deps world publish gpg clean baseimage_root $(PKGS) $(addprefix publish-,$(PKGS))

world: gpg $(PKGS)
	cp ./chef-public.gpg /opt/bldr/cache/keys/chef-public.asc

bldr-deps: gpg $(BLDR_PKGS)

publish:
	cargo build --release
	$(MAKE) $(addprefix publish-,$(PKGS))

$(PKGS):
	./bldr-build $@

$(addprefix publish-,$(PKGS)):
	../target/release/bldr upload chef/$(subst publish-,,$@) -u $(REPO)

gpg:
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
		/opt/bldr/pkgs/chef/cacerts \
		/opt/bldr/pkgs/chef/busybox \
		/opt/bldr/pkgs/chef/gnupg \
		/opt/bldr/pkgs/chef/openssl \
		/opt/bldr/pkgs/chef/bldr \
		&& \
	popd
