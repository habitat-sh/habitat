pkg_name=gnupg
pkg_distname=$pkg_name
pkg_derivation=chef
pkg_version=2.1.10
pkg_license=('gplv3+')
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_source=ftp://ftp.gnupg.org/gcrypt/${pkg_distname}/${pkg_distname}-${pkg_version}.tar.bz2
pkg_shasum=93bd58d81771a4fa488566e5d2e13b1fd7afc86789401eb41731882abfd26cf9
pkg_deps=(chef/glibc chef/libgpg-error chef/libassuan chef/libgcrypt chef/libksba chef/npth chef/zlib chef/bzip2 chef/readline)
pkg_build_deps=(chef/gcc chef/coreutils chef/sed chef/bison chef/flex chef/grep chef/bash chef/gawk chef/libtool chef/diffutils chef/findutils chef/xz chef/gettext chef/gzip chef/make chef/patch chef/texinfo chef/util-linux)
pkg_binary_path=(bin)
pkg_gpg_key=3853DA6B

do_build() {
  ./configure \
    --prefix=${pkg_prefix} \
    --sbindir=$pkg_prefix/bin
  make
}

do_check() {
  find tests -type f \
    | xargs sed -e "s,/bin/pwd,$(pkg_path_for coreutils)/bin/pwd,g" -i

  GNUPGHOME=`pwd` ./agent/gpg-agent --daemon make check
}

do_install() {
  do_default_install

  # Add symlinks for older tools
  ln -sv gpg2 $pkg_path/bin/gpg
  ln -sv gpgv2 $pkg_path/bin/gpgv
}
