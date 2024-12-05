+++
title = "Habitat package support policy"

linkTitle = "Package support policy"

gh_repo = "habitat"

[menu]
  [menu.habitat]
    title = "Overview"
    identifier = "habitat/packages/support/overview"
    parent = "habitat/packages/support"
    weight = 10
+++

This page documents our previous Habitat package support policy and the current multi-channel Habitat package refresh policy.

## Previous package support policy and limitations

Originally, all Habitat packages in the core origin were released and regularly refreshed using the stable channel,
which was designed for consistent stability for all packages.

Releasing packages through the stable channel led to the following issues:

- Projects with significant changes in a new release couldn't be updated in a package refresh because those changes could disrupt existing applications or installations.
  This prevented updates to packages where the project had a major version upgrade or significant security fixes.
- Packages couldn't be removed, deprecation, or relegated because that could disrupt existing applications or installations---even if the original project became outdated
- Package refreshes were skipped due to incompatibility or build issues.
- Only security fixes that have a minor impact were included in a package refresh.

For example, PostgreSQL 9.3 is still rebuilt in the core origin despite being out of support because removing it could disrupt existing applications.

To address these issues, we've switched to a multi-channel approach to better manage packages.

## Multi-channel package refresh policy

Chef Habitat packages hosted by the core, chef, and chef-platform origins are released through long-term support (LTS), innovation, and unstable channels.
This multi-channel approach manages package updates and lifecycles more effectively.
This approach allows deprecated packages to be excluded from new LTS releases, minimizing disruption for users.

### Supported channels

In the multi-channel package refresh policy, packages are released through three different types of support channels: long-term support, innovation, and unstable.
These channel types have the year appended to them to indicate when support for the channel began. For example, LTS-2024 or Innovation-2024.

LTS channels offer a stable environment with the latest refreshed packages and are supported for approximately three years, ensuring compatibility and updates.

Innovation channels are supported for a shorter duration and may include breaking changes.

An unstable channel is created for each LTS or innovation channel.
These channels are created to retain older package versions and are named after the innovation or LTS channel that they're based on.
For example, LTS-2024-unstable or Innovation-2024-unstable.

For more information, see the [release channels](release_channels) documentation.

### Support for stable channels

The stable channel in the core origin is deprecated with the release of Chef Habitat 2.x because of the presence of legacy and unsupported packages.

### Chef Habitat support

Chef Habitat 2.x allows users to specify different channels for different origins, providing more flexibility in package management.

### Habitat Builder support

The build function in Habitat Builder is disabled because building from a stable channel by default doesn't align with a multi-channel approach.

To sync packages hosted by the core, chef, and chef-platform origins with on-prem deployments of Habitat Builder,
we've created scripts to sync packages from the public Habitat Builder.
These scripts fetch packages released through the LTS and innovation channels.

### Package maintenance

Each package will have a maintenance cycle during a package refresh to help identify whether or not multiple major or minor versions for that project are available.
For more information, see the [package maintenance cycle documentation](maintenance_cycles).

### Package naming

Packages follow specific naming conventions based on their versioning schema, ensuring consistency and reliability in updates. For more information, see the [package naming documentation](naming).

### Requesting new packages

If a Chef Habitat user wants to have a new package added to the core origin, the following points apply:

- This must be requested in the form of an Aha! Idea.
- If the requested package meets the [support and usage threshold](thresholds), these requests will be considered alongside other feature requests during planning periods.
