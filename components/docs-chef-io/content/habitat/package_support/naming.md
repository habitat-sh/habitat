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

Each package in the core origin is identified by a unique string---called a package ident---in the following format: origin/name/version/release

When only one major version of a package is supported, use the following guidelines:

- The value of **name** should exactly match the name of the project it represents.
- The plan file should be located within a directory of the same name in this repository. For example, a single refresh will only maintain one major version of glibc and (as such) the package name will be core/glibc with no suffix.

When more than one major version of the package will be supported, the project uses semantic versioning (SemVer).

- If the project honors SemVer (only breaking changes occur in major releases):

  - The value of **name** should match the name of the project it represents, plus the major version of the package being supported (as a suffix).
  - The plan file should be located within a directory of the same name (including the suffix) in this repository. For example, core/postgresql16 and/or core/postgresql17.

- If the project doesn't honor SemVer (referred to as Romantic Versioning or RomVer):

  - The value of **name** should match the name of the project it represents, plus the major and minor version of the package being supported (as a suffix).
  - The plan file should be located within a directory of the same name (including the suffix) in this repository.

  {{< note >}}

  Romantic versions appear like a SemVer in format but may/can/will introduce breaking changes as part of a “minor” update. This results in Version X.Y having a breaking change versus X.Z.

  {{< /note >}}

  For example, core/foo3_0, core/ foo3_1, core/ foo3_2, and/or core/foo3_3.

- If a project doesn't use SemVer, for example builder-api or perl, the package will be reviewed from package to package and refresh to refresh.

{{< note >}}

Even though a package name may be altered to include a major (and minor) version suffix, package versions are never altered from their project's source. For example, if the package uses a DateVer schema where it is YYYYMMDD, this won't be reformatted to YYYY.MM.DD. This is to ensure the CVE detection process and automated build and detection systems can refer to the exact publishers' versions.

{{< /note >}}
