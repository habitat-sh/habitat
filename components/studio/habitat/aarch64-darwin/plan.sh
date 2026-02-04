# shellcheck disable=2034

pkg_name="hab-studio"
pkg_origin="chef"
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')

pkg_bin_dirs=(bin)

pkg_deps=(
	chef/hab-backline
	core/bash
)

pkg_build_deps=(
	chef/hab
	core/coreutils
	core/sed
	core/tar
)

pkg_version() {
	cat "$SRC_PATH/../../VERSION"
}

do_before() {
	build_line "$PWD"
	do_default_before
	update_pkg_version

	# shellcheck disable=2154
	pkg_filename=${pkg_name}-${pkg_version}.tar.gz
}

do_download() {
	local tar_binary
	tar_binary=$(pkg_path_for tar)/bin/tar

	pushd "$INITIAL_PWD" > /dev/null || exit

	build_line "Creating The source tar file. $pkg_filename in $PWD."
	$tar_binary -czf "$HAB_CACHE_SRC_PATH"/"$pkg_filename" components/ test-services/ Cargo.toml Cargo.lock  || exit

	popd || exit
}

do_verify() {
	return 0
}

do_unpack() {
	local tar_binary
	tar_binary=$(pkg_path_for tar)/bin/tar

	build_line "Unpacking the sources."

	pushd "$HAB_CACHE_SRC_PATH" > /dev/null || exit

	# shellcheck disable=2154
	mkdir "$pkg_dirname"
	tar -C "$pkg_dirname" -xzf "$pkg_filename"

	popd || exit
}

do_build() {
	return 0;
}

# shellcheck disable=2154
do_install() {
	pushd "$HAB_CACHE_SRC_PATH/$pkg_dirname" > /dev/null || exit

	# shellcheck disable=2154
	install -v -D components/studio/bin/hab-studio-"${pkg_target#*-}".sh "$pkg_prefix"/bin/hab-studio
	install -v -D components/studio/libexec/darwin-sandbox.sb "$pkg_prefix"/libexec/darwin-sandbox.sb
	install -v -D components/studio/libexec/hab-studio-darwin-profile.sh "$pkg_prefix"/libexec/hab-studio-darwin-profile.sh
	for f in components/studio/libexec/hab-studio-type-*.sh; do
		[[ -e $f ]] || break # see http://mywiki.wooledge.org/BashPitfalls#pf1
		install -v -D "$f" "$pkg_prefix"/libexec/"$(basename "$f")"
	done

	sed \
		-e "s,@author@,$pkg_maintainer,g" \
		-e "s,@version@,$pkg_version/$pkg_release,g" \
		-i "$pkg_prefix"/bin/hab-studio

	# Install a copy of a hab under `libexec/`
	install -v -D "$(pkg_path_for hab)"/bin/hab "$pkg_prefix/libexec/hab"

	cp -rv components/studio/defaults "${pkg_prefix}"

	fix_interpreter "${pkg_prefix}/bin/*" core/bash bin/sh

	popd || exit
}
