world:
	- gpg --import chef-public.gpg
	- gpg --import chef-private.gpg
	cd glibc && ../bldr-build Bldrfile
	cd libgcc && ../bldr-build Bldrfile
	cd zlib && ../bldr-build Bldrfile
	cd cacerts && ../bldr-build Bldrfile
	cd busybox && ../bldr-build Bldrfile
	cd gnupg && ../bldr-build Bldrfile
	cd openssl && ../bldr-build Bldrfile
	cd bldr && ../bldr-build Bldrfile
	cd redis && ../bldr-build Bldrfile
	cd ncurses && ../bldr-build Bldrfile
	cd libedit && ../bldr-build Bldrfile
	cd bzip2 && ../bldr-build Bldrfile
	cd pcre && ../bldr-build Bldrfile
	cd nginx && ../bldr-build Bldrfile
	cd haproxy && ../bldr-build Bldrfile
	cp ./chef-public.gpg /opt/bldr/cache/keys/chef-public.asc

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
