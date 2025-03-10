+++
title = "Habitat package naming conventions"

gh_repo = "habitat"

[menu]
  [menu.habitat]
    title = "Naming conventions"
    identifier = "habitat/packages/support/package_naming_conventions"
    parent = "habitat/packages/support"
    weight = 14
+++

This documents how Habitat packages are named in the core origin.

Each package in the core origin is identified by a unique string---called a package identifier---in the following format: origin/name/version/release.
The package name may be just the package name or the package name with a version suffix.

## Single major version

If the organization developing a project supports only one major version, the package for that project has the following guidelines:

- The value of _name_ exactly matches the name of the project.

  For example, a single release will only maintain one major version of glibc and (as such) the package name is "glibc" with no version suffix.

- The Habitat package plan file is located in a directory of the same name in the [habitat-sh/core-plans](https://github.com/habitat-sh/core-plans/) repository. For example, [core/glibc](https://github.com/habitat-sh/core-plans/tree/main/glibc).

## Multiple major versions

For projects that have more than one major version of a project supported at a time, the package name includes the version number. For example, postgresql16.

Package names that include a major (and minor) version suffix, package versions are never altered from their project's source.
For example, if the package uses a the YYYYMMDD DateVer schema, this isn't reformatted to YYYY.MM.DD.
This is to ensure the CVE detection process and automated build and detection systems can refer to the exact publishers' versions.

For projects that don't use SemVer or RomVer (for example builder-api or Perl) the package name is reviewed from refresh to refresh.

### SemVer

If the organization developing a project supports more than one major version and uses semantic versioning (SemVer):

- The value of _name_ matches the name of the project it represents, plus the major version of the package being supported (as a suffix).
- The plan file is located in a directory of the same name (including the suffix) in this repository. For example, core/postgresql16 and/or core/postgresql17.

### RomVer

If the organization developing a project supports more than one major version and uses romantic versioning (RomVer):

- The value of _name_ matches the name of the project it represents, plus the major and minor supported version number as a suffix.
  For example, core/foo3_0 or core/foo3_1.

- The plan file is located within a directory of the same name (including the suffix) in this repository.

  {{< note >}}

  Romantic versions look like a SemVer in format, but can introduce breaking changes as part of a “minor” update.
  This means you can have breaking changes if you upgrade from version X.Y to X.Z.

  {{< /note >}}
