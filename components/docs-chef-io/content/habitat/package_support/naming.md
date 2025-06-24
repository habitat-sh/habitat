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

This document explains the naming conventions for Habitat packages in the core origin.

Each package in the core origin is identified by a unique string---called a package identifier---in the following format: origin/name/version/release.
The package name may be just the package name or the package name with a version suffix.

## Single major version

If the organization developing a project supports only one major version, the package follows these guidelines:

- The value of _name_ matches the project name exactly.

  For example, since only one major version of `glibc` is maintained, the package name is simply `glibc` without a version suffix.

- The Habitat package plan file is located in a directory with the same name in the [habitat-sh/core-plans](https://github.com/habitat-sh/core-plans/) repository. For example, [core/glibc](https://github.com/habitat-sh/core-plans/tree/main/glibc).

## Multiple major versions

For projects that support more than one major version simultaneously, the package name includes the version number. For example, `postgresql16`.

When package names include a major (and minor) version suffix, the package version remains consistent with the project's source. For example, if the project uses a YYYYMMDD DateVer schema, it is not reformatted to YYYY.MM.DD. This ensures compatibility with CVE detection and automated build systems that rely on the publisher's exact versioning.

For projects that don't use SemVer or RomVer (for example, builder-api or Perl), the package name is reviewed during each refresh cycle.

### Semantic versioning

If the organization uses semantic versioning (SemVer) and supports multiple major versions:

- The value of _name_ matches the project name, with the major version appended as a suffix.
- The plan file is located in a directory with the same name (including the suffix) in the repository. For example, `core/postgresql16` or `core/postgresql17`.

### Romantic versioning

If the organization uses romantic versioning (RomVer) and supports multiple major versions:

- The value of _name_ matches the project name, with the major and minor version numbers appended as a suffix. For example, `core/foo3_0` or `core/foo3_1`.
- The plan file is located in a directory with the same name (including the suffix) in the repository.

  {{< note >}}

  Romantic versions resemble SemVer in format but may introduce breaking changes in a "minor" update. For example, upgrading from version X.Y to X.Z could include breaking changes.

  {{< /note >}}
