# This file is the heart of your application's habitat.
# See full docs at https://www.habitat.sh/docs/reference/plan-syntax/

# Required.
# Sets the name of the package. This will be used in along with `pkg_origin`,
# and `pkg_version` to define the fully-qualified package name, which determines
# where the package is installed to on disk, how it is referred to in package
# metadata, and so on.
pkg_name={{ pkg_name }}

# Required unless overridden by the `HAB_ORIGIN` environment variable.
# The origin is used to denote a particular upstream of a package.
pkg_origin={{ pkg_origin }}

# Required.
# Sets the version of the package.
{{#if pkg_version ~}}
pkg_version="{{ pkg_version }}"
{{else ~}}
pkg_version="0.1.0"
{{/if}}
# Optional.
# The name and email address of the package maintainer.
{{#if pkg_maintainer ~}}
pkg_maintainer="{{ pkg_maintainer }}"
{{else ~}}
# pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
{{/if}}
# Optional.
# An array of valid software licenses that relate to this package.
# Please choose a license from http://spdx.org/licenses/
{{#if pkg_license ~}}
pkg_license={{ pkg_license }}
{{else ~}}
# pkg_license=('Apache-2.0')
{{/if}}
# Required.
# A URL that specifies where to download the source from. Any valid wget url
# will work. Typically, the relative path for the URL is partially constructed
# from the pkg_name and pkg_version values; however, this convention is not
# required.
{{#if pkg_source ~}}
pkg_source="{{ pkg_source }}"
{{else ~}}
pkg_source="http://some_source_url/releases/${pkg_name}-${pkg_version}.tar.gz"
{{/if}}
# Optional.
# The resulting filename for the download, typically constructed from the
# pkg_name and pkg_version values.
{{#if pkg_filename ~}}
pkg_filename="{{ pkg_filename }}"
{{else ~}}
# pkg_filename="${pkg_name}-${pkg_version}.tar.gz"
{{/if}}
# Required if a valid URL is provided for pkg_source or unless do_verify() is overridden.
# The value for pkg_shasum is a sha-256 sum of the downloaded pkg_source. If you
# do not have the checksum, you can easily generate it by downloading the source
# and using the sha256sum or gsha256sum tools. Also, if you do not have
# do_verify() overridden, and you do not have the correct sha-256 sum, then the
# expected value will be shown in the build output of your package.
{{#if pkg_shasum ~}}
pkg_shasum="{{ pkg_shasum }}"
{{else ~}}
pkg_shasum="TODO"
{{/if}}
# Optional.
# An array of package dependencies needed at runtime. You can refer to packages
# at three levels of specificity: `origin/package`, `origin/package/version`, or
# `origin/package/version/release`.
{{#if pkg_deps ~}}
pkg_deps={{ pkg_deps }}
{{else ~}}
# pkg_deps=(core/glibc)
{{/if}}
# Optional.
# An array of the package dependencies needed only at build time.
{{#if pkg_build_deps ~}}
pkg_build_deps={{ pkg_build_deps }}
{{else ~}}
# pkg_build_deps=(core/make core/gcc)
{{/if}}
# Optional.
# An array of paths, relative to the final install of the software, where
# libraries can be found. Used to populate LD_FLAGS and LD_RUN_PATH for
# software that depends on your package.
{{#if pkg_lib_dirs ~}}
pkg_lib_dirs={{ pkg_lib_dirs }}
{{else ~}}
# pkg_lib_dirs=(lib)
{{/if}}
# Optional.
# An array of paths, relative to the final install of the software, where
# headers can be found. Used to populate CFLAGS for software that depends on
# your package.
{{#if pkg_include_dirs ~}}
pkg_include_dirs={{ pkg_include_dirs }}
{{else ~}}
# pkg_include_dirs=(include)
{{/if}}
# Optional.
# An array of paths, relative to the final install of the software, where
# binaries can be found. Used to populate PATH for software that depends on
# your package.
{{#if pkg_bin_dirs ~}}
pkg_bin_dirs={{ pkg_bin_dirs }}
{{else ~}}
# pkg_bin_dirs=(bin)
{{/if}}
# Optional.
# An array of paths, relative to the final install of the software, where
# pkg-config metadata (.pc files) can be found. Used to populate
# PKG_CONFIG_PATH for software that depends on your package.
{{#if pkg_pconfig_dirs ~}}
pkg_pconfig_dirs={{ pkg_pconfig_dirs }}
{{else ~}}
# pkg_pconfig_dirs=(lib/pconfig)
{{/if}}
# Optional.
# The command for the supervisor to execute when starting a service. You can
# omit this setting if your package is not intended to be run directly by a
# supervisor of if your plan contains a run hook in hooks/run.
{{#if pkg_svc_run ~}}
pkg_svc_run="{{ pkg_svc_run }}"
{{else ~}}
# pkg_svc_run="bin/haproxy -f $pkg_svc_config_path/haproxy.conf"
{{/if}}
# Optional.
# An array of ports this service exposes when you create a Docker image from
# your package.
{{#if pkg_expose ~}}
pkg_expose={{ pkg_expose }}
{{else ~}}
# pkg_expose=(80 443)
{{/if}}
# Optional.
# An array of interpreters used in shebang lines for scripts. Specify the
# subdirectory where the binary is relative to the package, for example,
# bin/bash or libexec/neverland, since binaries can be located in directories
# besides bin. This list of interpreters will be written to the metadata
# INTERPRETERS file, located inside a package, with their fully-qualified path.
# Then these can be used with the fix_interpreter function.
{{#if pkg_interpreters ~}}
pkg_interpreters={{ pkg_interpreters }}
{{else ~}}
# pkg_interpreters=(bin/bash)
{{/if}}
# Optional.
# The user to run the service as. The default is hab.
{{#if pkg_svc_user ~}}
pkg_svc_user="{{ pkg_svc_user }}"
{{else ~}}
# pkg_svc_user="hab"
{{/if}}
# Optional.
# The group to run the service as. The default is hab.
{{#if pkg_svc_group ~}}
pkg_svc_group="{{ pkg_svc_group }}"
{{else ~}}
# pkg_svc_group="$pkg_svc_user"
{{/if}}
# Required for core plans, optional otherwise.
# A short description of the package. It can be a simple string, or you can
# create a multi-line description using markdown to provide a rich description
# of your package.
{{#if pkg_description ~}}
pkg_description="{{ pkg_description }}"
{{else ~}}
# pkg_description="Some description."
{{/if}}
# Required for core plans, optional otherwise.
# The project home page for the package.
{{#if pkg_upstream_url ~}}
pkg_upstream_url="{{ pkg_upstream_url }}"
{{else ~}}
# pkg_upstream_url="http://example.com/project-name"
{{/if}}
