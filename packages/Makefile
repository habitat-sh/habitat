world: gpg
	./bldr-build glibc
	./bldr-build libgcc
	./bldr-build zlib
	./bldr-build cacerts
	./bldr-build busybox
	./bldr-build gnupg
	./bldr-build openssl
	./bldr-build runit
	./bldr-build bldr
	./bldr-build redis
	./bldr-build ncurses
	./bldr-build libedit
	./bldr-build bzip2
	./bldr-build pcre
	./bldr-build nginx
	./bldr-build haproxy
	./bldr-build libaio
	./bldr-build libltdl
	./bldr-build libxml2
	./bldr-build numactl
	./bldr-build perl
	cp ./chef-public.gpg /opt/bldr/cache/keys/chef-public.asc

gpg:
	- gpg --import chef-public.gpg
	- gpg --import chef-private.gpg

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
