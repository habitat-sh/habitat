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
  if $(sort --version 2>&1 | grep -q 'GNU coreutils'); then
    sort_binary=$(which sort)
  else
    if $(/usr/bin/sort --version 2>&1 | grep -q 'GNU coreutils'); then
      sort_binary=/usr/bin/sort
    else
      exit_with "We require GNU sort to find the latest package; aborting" 1
    fi
  fi
	latest_package_flags=$(echo $1 | grep -o '/' | wc -l)
	case $(trim $latest_package_flags) in
		"3")
			echo "/opt/bldr/pkgs/$1" ;;
		"2")
			echo $(find /opt/bldr/pkgs/${1} -maxdepth 1 -type d | $sort_binary --version-sort -r | head -n 1) ;;
		"1")
			echo $(find /opt/bldr/pkgs/${1} -maxdepth 2 -type d | $sort_binary --version-sort -r | head -n 1) ;;
	esac
}

copy_package() {
	local package_path=$(latest_package $1)
	if [[ -n "$package_path" ]]; then
		mkdir -p ./$package_path
		cp -r $package_path/* ./$package_path
	else
		echo "Failed to find $1; need to copy it into the base image"
		exit 1
	fi
}

set -e
if [[ -n "$DEBUG" ]]; then
  set -x
fi

BUSYBOX_ROOT=$(latest_package chef/busybox)
BLDR_ROOT=$(latest_package chef/bldr)
GPG_ROOT=$(latest_package chef/gnupg)

WORKDIR="$(pwd)"
ROOTFS=${TMPDIR:-/var/tmp}/rootfs-busybox-$$-$RANDOM
mkdir $ROOTFS
cd $ROOTFS

mkdir -p bin sbin etc dev dev/pts lib proc sys tmp opt/bldr/pkgs/chef
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

copy_package chef/glibc
copy_package chef/libgcc
copy_package chef/zlib
copy_package chef/cacerts
copy_package chef/busybox
copy_package chef/libgpg-error
copy_package chef/libassuan
copy_package chef/gnupg
copy_package chef/gpgme
copy_package chef/openssl
copy_package chef/bzip2
copy_package chef/xz
copy_package chef/libarchive
copy_package chef/rngd
copy_package chef/bldr

for x in $($BUSYBOX_ROOT/bin/busybox --list); do
	ln -s $BUSYBOX_ROOT/bin/busybox bin/$x
done
ln -s $BUSYBOX_ROOT/bin/busybox bin/init
ln -s $BLDR_ROOT/bin/bldr bin/bldr
ln -s $GPG_ROOT/bin/gpg bin/gpg
ln -s $GPG_ROOT/bin/gpg-zip bin/gpg-zip
ln -s $GPG_ROOT/bin/gpgsplit bin/gpgsplit
ln -s $GPG_ROOT/bin/gpgv bin/gpgv

cp /lib/x86_64-linux-gnu/ld-linux-x86-64.so.2 lib
for X in null ptmx random stdin stdout stderr tty urandom zero
do
    cp -a /dev/$X dev
done

/bin/tar --numeric-owner -cjf $WORKDIR/bldr-base.tar.bz2 .
