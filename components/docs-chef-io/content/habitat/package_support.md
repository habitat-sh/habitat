+++
title = "Habitat package support policy"
description = ""
gh_repo = "habitat"

[menu]
  [menu.habitat]
    title = "Package support policy"
    identifier = "habitat/packages/support"
    parent = "habitat/packages"
    weight = 10
+++

Refer to the following sections for details about the current approach its limitations and details about the multi-channel package refresh approach.

## Current approach and limitations

All Habitat packages in the core origin are regularly refreshed and released to the stable channel. Best practice is to use packages from the stable channel. Initially, the stable channel was designed for consistent stability across packages. However, this model is less effective for core libraries, which require specific lifecycle policies. No packages are removed - packages that cannot be refreshed due to incompatibility or build issues are simply skipped.

- Deprecation or relegation of packages and major version upgrades can cause potential disruption to existing applications or installations.
- Only security fixes that have a minor impact can be included. The current model prevents the deprecation or removal of outdated packages and limits the inclusion of significant security fixes or major version updates. This could lead to risks when using pinned packages because they might not upgrade to more secure versions.

For example, PostgreSQL 9.3 is still rebuilt in the core channel despite being out of support because removing it could disrupt existing applications. A shift to a multi-channel approach is suggested to better manage foundational components.

## Multi-channel package refresh approach

The new multi-channel approach for Habitat packages introduces Long-Term Support (LTS) and Innovation channels to manage package updates and lifecycles more effectively.

- **Channel definitions:** There are two types of channels: LTS-YYYY (providing multi-year support) and Innovation-YYYY (which may include breaking changes and is supported for a shorter duration).
- **LTS channel purpose:** LTS channels offer a stable environment with the latest refreshed packages that are supported for approximately three years, ensuring compatibility and updates.
- **Unstable channels:** For each LTS and Innovation channel, a corresponding unstable channel (for example, LTS-YYYY-unstable) is created to retain older package versions.
-**Deprecation strategy:** The multi-channel approach allows deprecated packages to be excluded from new LTS releases, minimizing disruption for users.
- **Impact on existing channels:** The stable channel in the core origin will be eventually deprecated with the next major Habitat release (Habitat 2.x) due to the presence of legacy and unsupported packages.
- **Changes to Habitat tools:** Updates to Habitat will allow users to specify different channels for different origins, providing more flexibility in package management.
- **Sync scripts:** Sync scripts to sync packages from core, chef, and chef-platform origins from LTS-YYYY and Innovation-YYYY channels from Public Builder to On Prem Builder.
- **Maintenance cycles:** Each core package will have an associated maintenance cycle during a package refresh to help identify whether or not multiple majors or minors for that project are available.
- **Package naming conventions:** Packages will follow specific naming conventions based on their versioning schema, ensuring consistency and reliability in updates.
- **Origins impacted:** core, chef and chef-platform. (Note: Chef 360 skills under chef-platform origin are currently in the stable channel. They will be moved to the LTS-2024 channel soon.)
- **Build function on public builder:** The build function on the SaaS builder will be disabled because building from stable by default does not align with a multi-channel approach.
- **Package intake process:** If a Habitat user wants to have a new package added to the core origin, the following points apply:
  - This must be requested in the form of an Aha Idea.
  - If the requested package meets the Support and Usage Threshold, these requests will be considered alongside other feature requests during planning periods.
