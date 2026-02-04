# shellcheck disable=2034

pkg_name="hab-plan-build"
pkg_origin="chef"
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')

pkg_bin_dirs=(bin)

pkg_deps=(
	core/bash
	core/cacerts
	core/coreutils
	core/file
	core/findutils
	core/gawk
	core/grep
	core/gzip
	chef/hab
	core/toml-cli
	core/sed
	core/tar
	core/unzip
	core/wget
	core/xz
	core/xcode
)
pkg_build_deps=(
	core/bats
)

program="hab-plan-build"

pkg_version() {
	cat "$SRC_PATH/../../VERSION"
}

do_before() {
	build_line $PWD
	do_default_before
	update_pkg_version

	pkg_filename=${pkg_name}-${pkg_version}.tar.gz
}

do_download() {
	local tar_binary
	tar_binary=$(pkg_path_for tar)/bin/tar

	pushd $INITIAL_PWD > /dev/null || exit

	build_line "Creating The source tar file. $pkg_filename in $PWD."
	$tar_binary -czf $HAB_CACHE_SRC_PATH/$pkg_filename components/ test-services/ Cargo.toml Cargo.lock  || exit

	popd
}

do_verify() {
	return 0
}

do_unpack() {
	local tar_binary
	tar_binary=$(pkg_path_for tar)/bin/tar

	build_line "Unpacking the sources."

	pushd $HAB_CACHE_SRC_PATH > /dev/null || exit

	mkdir $pkg_dirname
	tar -C $pkg_dirname -xzf $pkg_filename

	popd
}

runtime_sandbox() {
	echo '(version 1)
(allow file* process-exec process-fork
	(literal "/usr/bin/strip"))
'
}

do_build() {
	pushd "$HAB_CACHE_SRC_PATH/$pkg_dirname" > /dev/null || exit
	cp -v components/plan-build/bin/${program}-"${pkg_target#*-}".sh "$CACHE_PATH/$program"

	# Use the bash from our dependency list as the shebang. Also, embed the
	# release version of the program.
	# shellcheck disable=2154
	sed \
		-e "s,^HAB_PLAN_BUILD=0\.0\.1\$,HAB_PLAN_BUILD=$pkg_version/$pkg_release," \
		-e "s,^pkg_target='@@pkg_target@@'\$,pkg_target='$pkg_target'," \
		-i "$CACHE_PATH/$program"

	popd
}

do_check() {
	bats test
}

# shellcheck disable=2154
do_install() {
	pushd "$HAB_CACHE_SRC_PATH/$pkg_dirname" > /dev/null || exit

	install -D "$CACHE_PATH/$program" "$pkg_prefix"/bin/$program
	install -D components/plan-build/bin/shared.bash "$pkg_prefix"/bin/
	install -D components/plan-build/bin/public.bash "$pkg_prefix"/bin/
	install -D components/plan-build/bin/environment.bash "$pkg_prefix"/bin/
	install -D components/plan-build/bin/darwin-sandbox.sb "$pkg_prefix"/bin/
	install -D components/plan-build/bin/hab-plan-build-darwin-internal.bash "$pkg_prefix"/bin/

	# Fix scripts
	fix_interpreter "${pkg_prefix}/bin/*" core/bash bin/bash

	popd
}
