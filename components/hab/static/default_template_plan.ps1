{{#if with_docs ~}}
# This file is the heart of your application's habitat.
# See full docs at https://www.habitat.sh/docs/reference/plan-syntax/

# Required.
# Sets the name of the package. This will be used along with `$pkg_origin`,
# and `$pkg_version` to define the fully-qualified package name, which determines
# where the package is installed to on disk, how it is referred to in package
# metadata, and so on.
{{/if ~}}
$pkg_name="{{ pkg_name }}"
{{#if with_docs ~}}
# Required unless overridden by the `HAB_ORIGIN` environment variable.
# The origin is used to denote a particular upstream of a package.
{{/if ~}}
$pkg_origin="{{ pkg_origin }}"
{{#if with_docs ~}}
# Required.
# Sets the version of the package
{{/if ~}}
{{#if pkg_version ~}}
$pkg_version="{{ pkg_version }}"
{{/if ~}}
$pkg_scaffolding="{{ scaffolding_ident }}"
{{#if with_callbacks ~}}
function Invoke-Begin {}

function Invoke-Download {
  Invoke-DefaultDownload
}

function Invoke-Verify {
  Invoke-DefaultVerify
}

function Invoke-Clean {
  Invoke-DefaultClean
}

function Invoke-Unpack {
  Invoke-DefaultUnpack
}

function Invoke-Prepare {}

function Invoke-Build {
  Invoke-DefaultBuild
}

function Invoke-Check {}

function Invoke-Install {
  Invoke-DefaultInstall
}

function Invoke-End {}
{{/if ~}}
