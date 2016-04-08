pkg_origin=sap
pkg_name=hana
pkg_version=100.102.01
pkg_license=('SAP')
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_source=SAP_HANA_DATABASE100_102_01_Linux_on_x86_64.SAR
pkg_filename=SAP_HANA_DATABASE100_102_01_Linux_on_x86_64.SAR
pkg_shasum=6addbc9e75cae2b2372276163cd79f5fcea29e5bce0764f222625a891adbdea3
pkg_dirname=SAP_HANA_DATABASE
pkg_gpg_key=3853DA6B
pkg_bin_dirs=(bin)
pkg_deps=(chef/glibc chef/numactl chef/libltdl chef/libaio chef/libxml2 chef/libstdc++ chef/libgcc chef/zlib chef/linux-pam chef/util-linux chef/openssl-0.9.8)
pkg_docker_build="auto"
pkg_docker_from="ubuntu:latest"
pkg_expose=(1129 6379)

do_download() {
# I'm particularly sorry about this.
	ln -sf $PLAN_CONTEXT/$pkg_source $BLDR_SRC_CACHE
}

do_verify() {
	return 0
}

do_unpack() {
	$PLAN_CONTEXT/sapcar.exe -xvf $pkg_filename -R $BLDR_SRC_CACHE
}

do_build() {
	return 0
}

# We take the contents of the HANA install package, and write it to the package
# itself. We need to wrap sdbrun with a script that re-sets it's LD_LIBRARY_PATH,
# due to HANA injecting its own internal shared version of perl.
do_install() {
  mkdir -p $pkg_prefix/bin
  mv $BLDR_SRC_CACHE/$pkg_dirname/* $pkg_prefix/bin
  mv $pkg_prefix/bin/instruntime/sdbrun $pkg_prefix/bin/instruntime/sdbrun-real
  cat <<EOT >> $pkg_prefix/bin/instruntime/sdbrun
#!/bin/sh

export LD_LIBRARY_PATH=$LD_RUN_PATH:$pkg_prefix/bin/instruntime
exec $pkg_prefix/bin/instruntime/sdbrun-real "\$@"
EOT
  chmod a+x $pkg_prefix/bin/instruntime/sdbrun
}

