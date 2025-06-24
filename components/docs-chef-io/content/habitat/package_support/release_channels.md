+++
title = "Habitat package release channels"

gh_repo = "habitat"

[menu]
  [menu.habitat]
    title = "Release channels"
    identifier = "habitat/packages/support/release_channels"
    parent = "habitat/packages/support"
    weight = 11
+++

Refer to the following sections for details about the various release channels.

## Long-term support (LTS) channel

The LTS channel (designated as LTS-YYYY, where YYYY indicates the year of release) provides a stable environment with the latest refreshed packages, supported for three years. LTS releases ensure compatibility and updates over a multi-year period.

The initial LTS-YYYY release may include breaking changes due to major upgrades and the removal of end-of-life packages. However, subsequent updates within the channel will maintain stability and compatibility.

LTS versions of Chef tools (like Chef Infra 19) will be available under the chef origin in the LTS-YYYY channel. These tools are built from packages in the core origin's LTS-YYYY channel.

To retain older versions of packages after updates in LTS-YYYY, a corresponding unstable channel (for example, LTS-YYYY-unstable) will be created. This ensures deprecated packages are excluded from new LTS releases, minimizing user disruption.

The LTS-YYYY channel includes packages for common dependencies and compilers maintained by Chef, as well as packages for Chef Infra Client, Chef InSpec, and other related tools.

Overall, the LTS channel provides a reliable and consistent environment, ensuring long-term support and stability for applications and deployments.

## Innovation release channel

The innovation release channel (designated as Innovation-YYYY, where YYYY indicates the year of release) contains the latest refreshed packages between two LTS releases. This channel may introduce breaking changes to prepare for the next LTS release. The support duration for an innovation channel is shorter than that of an LTS channel and is determined by Progress Chef.

The innovation channel gives users access to the most recent updates and features, allowing them to test and adopt new changes before they're included in the next LTS release. This approach helps users stay up-to-date with the latest advancements while preparing for future LTS releases.

## Unstable channels

Unstable channels are created to retain older versions of packages after updates in the LTS-YYYY or Innovation-YYYY channels. For each LTS and innovation channel, a corresponding unstable channel (for example, LTS-YYYY-unstable or Innovation-YYYY-unstable) is created.

These unstable channels ensure deprecated packages are excluded from new LTS releases, minimizing user disruption. Users can access older versions of packages if needed while benefiting from the latest updates in the stable channels.

## Stable channels

Stable channels in the Habitat multi-channel approach refer to channels with a known and declared lifecycle and rules. These include LTS releases and innovation releases.

With the first multi-channel refresh (LTS-2024), the stable channel in the core origin will be deprecated due to the presence of legacy, unsupported, and end-of-life packages. The stable channel will remain active for a specified period to give Habitat users time to upgrade their workflows and adapt to the new approach.

After deprecation, the stable channel will no longer be actively developed or maintained and will eventually reach end-of-life status.

Once the stable channel of the core origin is declared end-of-life, no further updates will be provided for its packages. However, the channel will continue to exist until the end-of-life period for Habitat version 1.x concludes. After this period, all packages will be removed from the stable channel.
