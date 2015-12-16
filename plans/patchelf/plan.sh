pkg_name=patchelf
pkg_derivation=chef
pkg_version=0.8
pkg_license=('GPLv3')
pkg_source=http://releases.nixos.org/patchelf/patchelf-$pkg_version/patchelf-${pkg_version}.tar.gz
pkg_shasum=14af06a2da688d577d64ff8dac065bb8903bbffbe01d30c62df7af9bf4ce72fe
pkg_gpg_key=3853DA6B
pkg_binary_path=(bin)
# For now we're not including chef/libstdc++ because that plan is using
# patchelf to fix itself. Once the proper toolchain is built up, this
# will no longer be an issue. The side-effect is patchelf will link to
# a system lib, i.e /usr/lib/x86_64-linux-gnu/libstdc++.so.6 on Ubuntu. Since
# this is a build dependency tool and our environment is not 100% clean, I'm
# okay living with this for now - FIN
pkg_deps=(chef/glibc chef/libgcc)
