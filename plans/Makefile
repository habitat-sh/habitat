PKGS := glibc libgcc zlib cacerts busybox patchelf libgpg-error libassuan gnupg gpgme openssl runit bldr redis ncurses libedit bzip2 pcre nginx haproxy libaio libltdl libxml2 numactl perl
REPO := http://ec2-52-10-238-149.us-west-2.compute.amazonaws.com

bldr: gpg
	@for pkg in glibc libgcc zlib cacerts busybox patchelf libgpg-error libassuan gnupg gpgme openssl runit bldr; do \
		./bldr-build $$pkg; \
	done

world: gpg
	@for pkg in $(PKGS); do \
		./bldr-build $$pkg; \
	done
	cp ./chef-public.gpg /opt/bldr/cache/keys/chef-public.asc

publish:
	cargo build --release
	@for pkg in $(PKGS); do \
		../target/release/bldr upload chef/$$pkg -u $(REPO); \
	done

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
