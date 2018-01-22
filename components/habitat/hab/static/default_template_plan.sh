{{#if with_docs ~}}
# This file is the heart of your application's habitat.
# See full docs at https://www.habitat.sh/docs/reference/plan-syntax/

# Required.
# Sets the name of the package. This will be used in along with `pkg_origin`,
# and `pkg_version` to define the fully-qualified package name, which determines
# where the package is installed to on disk, how it is referred to in package
# metadata, and so on.
{{/if ~}}
pkg_name={{ pkg_name }}
{{#if with_docs ~}}
# Required unless overridden by the `HAB_ORIGIN` environment variable.
# The origin is used to denote a particular upstream of a package.
{{/if ~}}
pkg_origin={{ pkg_origin }}
{{#if with_docs ~}}
# Required.
# Sets the version of the package
{{/if ~}}
{{#if pkg_version ~}}
pkg_version="{{ pkg_version }}"
{{/if ~}}
pkg_scaffolding="{{ scaffolding_ident }}"
{{#if with_callbacks ~}}
do_begin() {
  return 0
}

do_download() {
  do_default_download
}

do_verify() {
  do_default_verify
}

do_clean() {
  do_default_clean
}

do_unpack() {
  do_default_unpack
}

do_prepare() {
  return 0
}

do_build() {
  do_default_build
}

do_check() {
  return 0
}

do_install() {
  do_default_install
}

do_strip() {
  do_default_strip
}

do_end() {
  return 0
}
{{/if ~}}
