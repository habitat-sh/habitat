{{#if with_docs ~}}
# This file is the heart of your application's habitat.
# See full docs at https://www.habitat.sh/docs/reference/plan-syntax/

# Required.
# Sets the name of the package. This will be used in along with `pkg_origin`,
# and `pkg_version` to define the fully-qualified package name, which determines
# where the package is installed to on disk, how it is referred to in package
# metadata, and so on.
{{/if}}
pkg_name={{ pkg_name }}
{{#if with_docs ~}}
# Required unless overridden by the `HAB_ORIGIN` environment variable.
# The origin is used to denote a particular upstream of a package.
{{/if}}
pkg_origin={{ pkg_origin }}
{{#if with_docs ~}}
# Required.
# Sets the version of the package
{{/if}}.
{{#if pkg_version ~}}
pkg_version="{{ pkg_version }}"
{{else ~}}
pkg_version="0.1.0"
{{/if}}
pkg_scaffolding="TODO:Codebase Detection"