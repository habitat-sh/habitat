#!/usr/bin/env bash
# Generate a very minimal filesystem based on busybox-static,
# and load it into the local docker under the name "busybox".

# Thanks to these guys:
# http://stackoverflow.com/questions/369758/how-to-trim-whitespace-from-bash-variable
trim() {
    local var="$*"
    var="${var#"${var%%[![:space:]]*}"}"   # remove leading whitespace characters
    var="${var%"${var##*[![:space:]]}"}"   # remove trailing whitespace characters
    echo "$var"
}

latest_package() {
	latest_package_flags=$(echo $1 | grep -o '/' | wc -l)
	case $(trim $latest_package_flags) in
		"3")
			echo "/opt/bldr/pkgs/$1" ;;
		"2")
			echo $(find /opt/bldr/pkgs/${1} -maxdepth 1 -type d | sort --version-sort -r | head -n 1) ;;
		"1")
			echo $(find /opt/bldr/pkgs/${1} -maxdepth 2 -type d | sort --version-sort -r | head -n 1) ;;
	esac
}

set -e
if [[ -n "$DEBUG" ]]; then
  set -x
fi

BUSYBOX_ROOT=$(latest_package bldr/busybox)
BLDR_ROOT=$(latest_package bldr/bldr)
GPG_ROOT=$(latest_package bldr/gnupg)
RUNIT_ROOT=$(latest_package bldr/runit)

WORKDIR="$(pwd)"
ROOTFS=${TMPDIR:-/var/tmp}/rootfs-busybox-$$-$RANDOM
mkdir $ROOTFS
cd $ROOTFS

mkdir -p bin sbin etc dev dev/pts lib proc sys tmp opt/bldr/pkgs/bldr
cat <<EOT >> etc/resolv.conf
nameserver 8.8.8.8
nameserver 8.8.4.4
EOT
cat <<EOT >> etc/nsswitch.conf
passwd:     files
group:      files
shadow:     files

hosts:      files dns
networks:   files

rpc:        files
services:   files
EOT
echo root:x:0:0:root:/:/bin/sh > etc/passwd
echo bldr:x:42:42:root:/:/bin/sh >> etc/passwd
echo root:x:0: > etc/group
echo bldr:x:42:bldr >> etc/group
ln -s lib lib64
cp -r /opt/bldr/pkgs/bldr/bldr opt/bldr/pkgs/bldr
cp -r /opt/bldr/pkgs/bldr/busybox opt/bldr/pkgs/bldr
cp -r /opt/bldr/pkgs/bldr/cacerts opt/bldr/pkgs/bldr
cp -r /opt/bldr/pkgs/bldr/glibc opt/bldr/pkgs/bldr
cp -r /opt/bldr/pkgs/bldr/gnupg opt/bldr/pkgs/bldr
cp -r /opt/bldr/pkgs/bldr/libgcc opt/bldr/pkgs/bldr
cp -r /opt/bldr/pkgs/bldr/openssl opt/bldr/pkgs/bldr
cp -r /opt/bldr/pkgs/bldr/zlib opt/bldr/pkgs/bldr
cp -r /opt/bldr/pkgs/bldr/runit opt/bldr/pkgs/bldr

for x in $($BUSYBOX_ROOT/bin/busybox --list); do
	ln -s $BUSYBOX_ROOT/bin/busybox bin/$x
done
ln -s $BUSYBOX_ROOT/bin/busybox bin/init
ln -s $BLDR_ROOT/bin/bldr bin/bldr
ln -s $GPG_ROOT/bin/gpg bin/gpg
ln -s $GPG_ROOT/bin/gpg-zip bin/gpg-zip
ln -s $GPG_ROOT/bin/gpgsplit bin/gpgsplit
ln -s $GPG_ROOT/bin/gpgv bin/gpgv
ln -sf $RUNIT_ROOT/bin/chpst bin/chpst
ln -sf $RUNIT_ROOT/bin/runit bin/runit
ln -sf $RUNIT_ROOT/bin/runit-init bin/runit-init
ln -sf $RUNIT_ROOT/bin/runsv bin/runsv
ln -sf $RUNIT_ROOT/bin/runsvchdir bin/runsvchdir
ln -sf $RUNIT_ROOT/bin/runsvdir bin/runsvdir
ln -sf $RUNIT_ROOT/bin/sv bin/sv
ln -sf $RUNIT_ROOT/bin/svlogd bin/svlogd
ln -sf $RUNIT_ROOT/bin/utmpset bin/utmpset

cp /lib/x86_64-linux-gnu/ld-linux-x86-64.so.2 lib
for X in console null ptmx random stdin stdout stderr tty urandom zero
do
    cp -a /dev/$X dev
done

tar --numeric-owner -cjf $WORKDIR/bldr-base.tar.bz2 .
