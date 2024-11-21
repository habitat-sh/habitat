+++
title = "Release channels"
description = ""
gh_repo = "habitat"

[menu]
  [menu.habitat]
    title = "Release channels"
    identifier = "habitat/packages/support/release_channels"
    parent = "habitat/packages/package_support"
    weight = 11
+++

## Long Term Support (LTS) channel

The LTS Channel (designated as LTS-YYYY where YYYY indicates the year of release) is designed to provide a stable environment with the latest refreshed packages that will be supported for three years. LTS releases aim to ensure compatibility and updates over a multi-year period.

The initial LTS-YYYY release may involve breaking changes due to major upgrades and the removal of end-of-life packages. However, subsequent releases within the channel will maintain stability and compatibility.

LTS versions of Chef tools (such as Chef Infra 19) will be available under chef origin in the LTS-YYYY channel. These tools will be built from packages in the core origin from the LTS-YYYY channel.

To retain older versions of packages once they are updated in LTS-YYYY, a corresponding unstable channel (for example, LTS-YYYY-unstable) will be created. This approach ensures that deprecated packages are excluded from new LTS releases, minimizing disruption for users.

The LTS-YYYY channel will contain packages for common dependencies and compilers maintained by Chef, in addition to packages for Chef Infra Client, Chef InSpec, and other related tools.

Overall, the LTS Channel provides a reliable and consistent environment for users, ensuring long-term support and stability for their applications and deployments.

## Innovation release channel

The innovation release channel (designated as Innovation-YYYY where YYYY indicates the year of release) contains the latest refreshed packages between two LTS releases. This channel may introduce breaking changes to prepare for the next LTS release. The support duration for an innovation channel is shorter than that of an LTS channel and is determined at the discretion of Progress Chef.

The innovation channel is designed to provide users with access to the most recent updates and features, allowing them to test and adopt new changes before they are included in the next LTS release. This approach ensures that users can stay up-to-date with the latest advancements while also preparing for future LTS releases.

## Unstable channels

The unstable channels are created to retain older versions of packages once they are updated in the LTS-YYYY or Innovation-YYYY channels. For each LTS and innovation channel, a corresponding unstable channel (for example, LTS-YYYY-unstable or Innovation-YYYY-unstable) is created.

These unstable channels ensure that deprecated packages are excluded from new LTS releases, minimizing disruption for users. This approach allows users to access older versions of packages if needed, while still benefiting from the latest updates and improvements in the stable channels.

## Stable channels

The stable channels in the Habitat multi-channel approach refer to the channels that have a known and declared lifecycle and rules. These channels are divided into two categories: LTS releases and Innovation releases.

With the release of the first multi-channel refresh (LTS-2024), the stable channel in the core origin will be deprecated due to the presence of multiple legacy, unsupported, and end-of-life packages. The stable channel will remain active until a specified date to provide Habitat users with sufficient time to upgrade their workflows and adapt to the new approach.

Following its deprecation, the stable channel will no longer be under active development or maintenance and will eventually reach end-of-life status.

Once the stable channel of the core origin is declared end-of-life, no further updates will be provided for the packages in that channel for any reason. However, the channel will continue to exist until the end-of-life period for Habitat version 1.x has concluded. After this period, all packages will be removed from the stable channel.
