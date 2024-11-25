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

Refer to the following sections for details about the support and usage thresholds.

## Support threshold

Progress Chef supports packages that aren't dependencies for Progress Chef tools under the following conditions:

- There isn't a published end-of-life (EOL) date within the first two years of the LTS-YYYY support duration at the time of consideration.
  If the OEM drops support or doesn't provide a fix to known vulnerabilities during the LTS time frame, no updates will be made available.
- There aren't any open high or critical CVEs for that package at the time of consideration.
  If any released package has a critical or high CVE post-release, the package will be fixed as part of the next minor or directed refresh.
- It should have a valid licensing model that allows Progress Chef to distribute it as a Habitat package.

## Usage threshold

A requested package will be added for backlog prioritization it if falls in one of the following categories:

- It's a commonly-used development tool or programming language.
- It's high-demand commercial off-the-shelf software.
