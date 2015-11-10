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
	cp ./chef-public.gpg /opt/bldr/cache/keys/chef-public.asc

gpg:
	- gpg --import chef-public.gpg
	- gpg --import chef-private.gpg

clean:
	rm -rf /opt/bldr/pkgs/*

baseimage_root:
	pushd / && \
	tar jcvf /src/bldr-base/bldr-base.tar.bz2 \
		/opt/bldr/pkgs/bldr/glibc \
		/opt/bldr/pkgs/bldr/libgcc \
		/opt/bldr/pkgs/bldr/zlib \
		/opt/bldr/pkgs/bldr/cacerts \
		/opt/bldr/pkgs/bldr/busybox \
		/opt/bldr/pkgs/bldr/gnupg \
		/opt/bldr/pkgs/bldr/openssl \
		/opt/bldr/pkgs/bldr/bldr \
		&& \
	popd
