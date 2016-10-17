#!/bin/bash
#
# # Usage
#
# ```
# $ hab-pkg-deb [PKG ...]
# ```
#
# # Synopsis
#
# Debian package exporter for Habitat artifacts
#
# # License and Copyright
#
# ```
# Copyright: Copyright (c) 2016 Chef Software, Inc.
# License: Apache License, Version 2.0
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
# ```

# Everything in this script was shamelessy and gratefully copied from
# hab-pkg-dockerize, hab-pkg-tarize, and omnibus/packagers/deb.rb.

# defaults for the application
: "${PKG:="unknown"}"

# Fail if there are any unset variables and whenever a command returns a
# non-zero exit code.
set -eu

# If the variable `$DEBUG` is set, then print the shell commands as we execute.
if [ -n "${DEBUG:-}" ]; then
  set -x
  export DEBUG
fi

# ## Help

# **Internal** Prints help
print_help() {
  printf -- "$program $version

$author

Habitat Package Debian - Create a Debian package from a set of Habitat packages

USAGE:
  $program [FLAGS] <PKG_IDENT>

FLAGS:
    --help           Prints help information

ARGS:
    <PKG_IDENT>      Habitat package identifier (ex: acme/redis)
"
}

# **Internal** Exit the program with an error message and a status code.
#
# ```sh
# exit_with "Something bad went down" 55
# ```
exit_with() {
  case "${TERM:-}" in
    *term | xterm-* | rxvt | screen | screen-*)
      printf -- "\033[1;31mERROR: \033[1;37m$1\033[0m\n"
      ;;
    *)
      printf -- "ERROR: $1\n"
      ;;
  esac
  exit "$2"
}

# **Internal** Print a warning line on stderr. Takes the rest of the line as its
# only argument.
#
# ```sh
# warn "Checksum failed"
# ```
warn() {
  case "${TERM:-}" in
    *term | xterm-* | rxvt | screen | screen-*)
      printf -- "\033[1;33mWARN: \033[1;37m$1\033[0m\n" >&2
      ;;
    *)
      printf -- "WARN: $1\n" >&2
      ;;
  esac
}

# Build the debian package
build_deb() {
  # Install packages using the studio
  env FS_ROOT="$staging_dir" hab pkg install "$PKG"
  # Delete files we won't be needing
  rm -rf "${staging_dir:?}/hab/cache"
  # Make a DEBIAN directory
  mkdir -p "$staging_dir/DEBIAN"

	# Set these variables in advance, since they may or may not be in the manifest,
	# since they are optional
	pkg_description=
	pkg_license=
	pkg_maintainer=
	pkg_upstream_url=

	# Read the manifest to extract variables from it
	manifest="$(cat "$staging_dir"/hab/pkgs/"$PKG"/**/**/MANIFEST)"

	# TODO: Handle multi-line descriptions
	# FIXME: This probably fail when there's a ":" in them
	pkg_description="$(grep __Description__: <<< "$manifest" | cut -d ":" -f2 | sed 's/^ *//g')"
	pkg_license="$(grep __License__: <<< "$manifest" | cut -d ":" -f2 | sed 's/^ *//g')"
	pkg_maintainer="$(grep __Maintainer__: <<< "$manifest" | cut -d ":" -f2 | sed 's/^ *//g')"
	pkg_upstream_url="$(grep __Upstream\ URL__: <<< "$manifest" | cut -d ":" -f2 | sed 's/^ *//g')"

	# Get the ident and the origin and release from that
  ident="$(cat "$staging_dir"/hab/pkgs/"$PKG"/**/**/IDENT)"

  pkg_origin="$(echo "$ident" | cut -f1 -d/)"
  pkg_name="$(echo "$ident" | cut -f2 -d/)"
  pkg_version="$(echo "$ident" | cut -f3 -d/)"
  pkg_release="$(echo "$ident" | cut -f4 -d/)"

  # Write the control file
  render_control_file > "$staging_dir/DEBIAN/control"

  # TODO: Write conffiles file
  # TODO: Write scripts dir

  render_md5sums > "$staging_dir/DEBIAN/md5sums"

  # Create the package
	# TODO: Do we do something with the origin here?
  dpkg-deb -z9 -Zgzip --debug --build "$staging_dir" \
		"$(safe_base_package_name)_$(safe_version)-${pkg_release}_$(architecture).deb"
}

# Output the contents of the "control" file
render_control_file() {
# TODO: Depends/conflicts/replaces
# TODO: Should vendor be the origin or not?
control=$(cat <<EOF
Package: $(safe_base_package_name)
Version: $(safe_version)-$pkg_release
Vendor: $pkg_origin
Architecture: $(architecture)
Installed-Size: $(installed_size)
Section: $(section)
Priority: $(priority)
EOF
)

# TODO: Format the description correctly
# See https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-Description
if [[ ! -z $pkg_description ]]; then
	control="$control
Description: $pkg_description"
# Description is required, so just use the package name if we don't have one
else
	control="$control
Description: $pkg_name"
fi

if [[ ! -z $pkg_upstream_url ]]; then
	control="$control
Homepage: $pkg_upstream_url"
fi

if [[ ! -z $pkg_license ]]; then
	control="$control
License: $pkg_license"
fi

if [[ ! -z $pkg_maintainer ]]; then
	control="$control
Maintainer: $pkg_maintainer"
# Maintainer is required, so use the origin if we don't have one
else
	control="$control
Maintainer: $pkg_origin"
fi

echo "$control"
}

render_md5sums() {
  pushd "$staging_dir" > /dev/null
    find . -type f ! -regex '.*?DEBIAN.*' -printf '%P ' | xargs md5sum
  popd > /dev/null
}

# Return the Debian-ready base package name, converting any invalid characters to
# dashes (-).
safe_base_package_name() {
  if [[ $pkg_name =~ ^[a-z0-9\.\+\\-]+$ ]]; then
    echo "$pkg_name"
  else
    converted="${pkg_name,,}"
    # FIXME: I'm doing this regex wrong
    converted="${converted//[^a-z0-9\.\+\-]+/-}"
    warn "The 'name' component of Debian package names can only include "
    warn "lower case alphabetical characters (a-z), numbers (0-9), dots (.), "
    warn "plus signs (+), and dashes (-). Converting '$pkg_name' to "
    warn "'$converted'."
    echo "$converted"
  fi
}

# Return the Debian-ready version, replacing all dashes (-) with tildes
# (~) and converting any invalid characters to underscores (_).
safe_version() {
  if [[ $pkg_version == *"-"* ]]; then
    converted="${pkg_version//-/\~}"
		warn "Dashes hold special significance in the Debian package versions. "
    warn "Versions that contain a dash and should be considered an earlier "
    warn "version (e.g. pre-releases) may actually be ordered as later "
    warn "(e.g. 12.0.0-rc.6 > 12.0.0). We'll work around this by replacing "
    warn "dashes (-) with tildes (~). Converting '$pkg_version' "
    warn "to '$converted'."
    echo "$converted"
	else
  	echo "$pkg_version"
	fi
}

# The platform architecture.
architecture() {
  dpkg --print-architecture
}

# The size of the package when installed.
#
# Per http://www.debian.org/doc/debian-policy/ch-controlfields.html, the
# disk space is given as the integer value of the estimated installed
# size in bytes, divided by 1024 and rounded up.
installed_size() {
  du "$staging_dir" --apparent-size --block-size=1024 --summarize | cut -f1
}

# The package priority.
#
# Can be one of required, important, standard, optional, or extra.
# See https://www.debian.org/doc/manuals/debian-faq/ch-pkg_basics.en.html#s-priority
#
# TODO: Allow customizing this
priority() {
  echo extra
}

# The package section.
#
# See https://www.debian.org/doc/debian-policy/ch-archive.html#s-subsections
#
# TODO: Allow customizing this
section() {
  echo misc
}

# Parse the CLI flags and options
parse_options() {
  for i in "$@"
  do
    case $i in
      --help)
        print_help
        exit
        ;;
      *)
        PKG=${i}
        ;;
    esac
  done
  if [ "$PKG" == "unknown" ]; then
    print_help
    exit_with "You must specify one or more Habitat packages." 1
  fi
}

# Adjust the $PATH to make sure we're using the right binaries
PATH=$(hab pkg path core/tar)/bin:$(hab pkg path core/findutils)/bin:$(hab pkg path core/coreutils)/bin:$PATH

# The current version of Habitat Studio
version='@version@'
# The author of this program
author='@author@'
# The short version of the program name which is used in logging output
program="$(basename "$0")"
# The place where we put the files we're building
staging_dir="$(mktemp -t --directory "$program-XXXX")"

parse_options "$@"
build_deb
