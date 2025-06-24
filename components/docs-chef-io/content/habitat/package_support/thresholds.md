+++
title = "Package support and usage thresholds"

gh_repo = "habitat"

[menu]
  [menu.habitat]
    title = "Support and usage thresholds"
    identifier = "habitat/packages/support/support_and_usage_thresholds"
    parent = "habitat/packages/support"
    weight = 15
+++

Refer to the following sections for details about support and usage thresholds.

## Support threshold

Progress Chef supports packages that aren't dependencies for its tools under these conditions:

- There isn't a published end-of-life (EOL) date within the first two years of the LTS-YYYY support duration at the time of consideration. If the OEM drops support or doesn't fix known vulnerabilities during the LTS timeframe, no updates will be provided.
- There aren't any open high or critical CVEs for the package at the time of consideration. If a released package has a critical or high CVE post-release, it will be fixed in the next minor or directed refresh.
- The package has a valid licensing model that allows Progress Chef to distribute it as a Habitat package.

## Usage threshold

Requested packages are added to a support backlog and prioritized if they meet one of these criteria:

- It's for a commonly used development tool or programming language.
- It's for high-demand commercial off-the-shelf software.
