pkg_name={{ pkg_name }}
pkg_origin={{ pkg_origin }}
{{#if pkg_version ~}}
pkg_version="{{ pkg_version }}"
{{~ else ~}}
pkg_version="0.1.0"
{{~ /if}}
{{#if pkg_maintainer ~}}
pkg_maintainer="{{ pkg_maintainer }}"
{{~ else ~}}
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
{{~ /if}}
{{#if pkg_license ~}}
pkg_license={{ pkg_license }}
{{~ else ~}}
pkg_license=("Apache-2.0")
{{~ /if}}
{{#if scaffolding_ident ~}}
pkg_scaffolding="{{ scaffolding_ident }}"
{{~ else ~}}
# pkg_scaffolding="some/scaffolding"
{{~ /if}}
{{#if pkg_source ~}}
pkg_source="{{ pkg_source }}"
{{~ else ~}}
# pkg_source="http://some_source_url/releases/${pkg_name}-${pkg_version}.tar.gz"
{{~ /if}}
{{#if pkg_shasum ~}}
pkg_shasum="{{ pkg_shasum }}"
{{~ else ~}}
pkg_shasum="TODO"
{{~ /if}}
{{#if pkg_deps ~}}
pkg_deps={{ pkg_deps }}
{{~ else ~}}
pkg_deps=(core/glibc)
{{~ /if}}
{{#if pkg_build_deps ~}}
pkg_build_deps={{ pkg_build_deps }}
{{~ else ~}}
pkg_build_deps=(core/make core/gcc)
{{~ /if}}
{{#if pkg_lib_dirs ~}}
pkg_lib_dirs={{ pkg_lib_dirs }}
{{~ else ~}}
# pkg_lib_dirs=(lib)
{{~ /if}}
{{#if pkg_include_dirs ~}}
pkg_include_dirs={{ pkg_include_dirs }}
{{~ else ~}}
# pkg_include_dirs=(include)
{{~ /if}}
{{#if pkg_bin_dirs ~}}
pkg_bin_dirs={{ pkg_bin_dirs }}
{{~ else ~}}
# pkg_bin_dirs=(bin)
{{~ /if}}
{{#if pkg_pconfig_dirs ~}}
pkg_pconfig_dirs={{ pkg_pconfig_dirs }}
{{~ else ~}}
# pkg_pconfig_dirs=(lib/pconfig)
{{~ /if}}
{{#if pkg_description ~}}
pkg_description="{{ pkg_description }}"
{{~ else ~}}
# pkg_description="Some description."
{{~ /if}}
{{#if pkg_upstream_url ~}}
pkg_upstream_url="{{ pkg_upstream_url }}"
{{~ else ~}}
# pkg_upstream_url="http://example.com/project-name"
{{~ /if}}

do_build() {
  do_default_build
}

do_install() {
  do_default_install
}
