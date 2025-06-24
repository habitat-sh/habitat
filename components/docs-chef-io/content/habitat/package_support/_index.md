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

This page outlines the previous Habitat package support policy and the current multi-channel Habitat package refresh policy.

## Previous package support policy and limitations

Originally, all Habitat packages in the core origin were released and refreshed using the stable channel, which prioritized consistent stability. However, this approach led to several challenges:

- **Inability to Update Major Changes:** Significant changes in new releases, such as major version upgrades or critical security fixes, could not be included in package refreshes to avoid disrupting existing applications or installations.
- **Retention of Outdated Packages:** Packages could not be removed, deprecated, or downgraded, even if the original project became outdated, to prevent disruptions.
- **Skipped Refreshes:** Some package refreshes were skipped due to incompatibility or build issues.
- **Limited Security Fixes:** Only minor-impact security fixes were included in package refreshes.

For example, PostgreSQL 9.3 continues to be rebuilt in the core origin despite being out of support, as its removal could disrupt existing applications.

To address these issues, we adopted a multi-channel approach to better manage package updates and lifecycles.

## Multi-channel package refresh policy

Chef Habitat packages hosted in the core, chef, and chef-platform origins are now released through three types of channels: long-term support (LTS), innovation, and unstable. This approach improves package lifecycle management and allows deprecated packages to be excluded from new LTS releases, minimizing user disruption.

### Supported channels

Packages are released through the following channels, each appended with the year support began (e.g., LTS-2024 or Innovation-2024):

- **LTS Channels:** Provide a stable environment with the latest refreshed packages and are supported for approximately three years, ensuring compatibility and updates.
- **Innovation Channels:** Supported for a shorter duration and may include breaking changes.
- **Unstable Channels:** Created for each LTS or innovation channel to retain older package versions. These channels are named after their parent channel (e.g., LTS-2024-unstable or Innovation-2024-unstable).

For more details, see the [release channels](release_channels) documentation.

### Stable channel deprecation

The stable channel in the core origin is deprecated with the release of Chef Habitat 2.x due to the presence of legacy and unsupported packages.

### Chef Habitat support

Chef Habitat 2.x allows users to specify different channels for different origins, offering greater flexibility in package management.

### Habitat Builder support

The build function in Habitat Builder is disabled because building from the stable channel by default is incompatible with the multi-channel approach.

To sync packages from the core, chef, and chef-platform origins with on-prem Habitat Builder deployments, scripts are available to fetch packages from the public Habitat Builder. These scripts retrieve packages released through the LTS and innovation channels.

### Package maintenance

Each package undergoes a maintenance cycle during a package refresh to determine whether multiple major or minor versions of the project are available. For more details, see the [package maintenance cycle documentation](maintenance_cycles).

### Package naming

Packages follow specific naming conventions based on their versioning schema to ensure consistency and reliability. For more details, see the [package naming documentation](naming).

### Requesting new packages

To request a new package for the core origin, the following applies:

- Requests must be submitted as an Aha! Idea.
- If the requested package meets the [support and usage threshold](thresholds), it will be considered alongside other feature requests during planning periods.
