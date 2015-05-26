world:
	- gpg --import chef-public.gpg
	- gpg --import chef-private.gpg
	./bldr-build glibc-Bldrfile
	./bldr-build libgcc-Bldrfile
	./bldr-build zlib-Bldrfile
	./bldr-build cacerts-Bldrfile
	./bldr-build busybox-Bldrfile
	./bldr-build gnupg-Bldrfile
	./bldr-build openssl-Bldrfile
	./bldr-build redis-Bldrfile
	./bldr-build bldr-Bldrfile
	cp ./chef-public.gpg /opt/bldr/cache/keys/chef-public.asc

clean:
	rm -rf /opt/bldr/pkgs/*

baseimage_root:
	tar jcvf /src/bldr-base/bldr-base.tar.bz2 \
		/opt/bldr/pkgs/bldr/glibc \
		/opt/bldr/pkgs/bldr/libgcc \
		/opt/bldr/pkgs/bldr/zlib \
		/opt/bldr/pkgs/bldr/cacerts \
		/opt/bldr/pkgs/bldr/busybox \
		/opt/bldr/pkgs/bldr/gnupg \
		/opt/bldr/pkgs/bldr/openssl \
		/opt/bldr/pkgs/bldr/bldr
