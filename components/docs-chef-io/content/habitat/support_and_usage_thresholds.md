+++
title = "Support and usage thresholds"
description = ""
gh_repo = "habitat"

[menu]
  [menu.habitat]
    title = "Support and usage thresholds"
    identifier = "habitat/packages/support/package_support/support_and_usage_thresholds"
    parent = "habitat/packages/package_support"
    weight = 15
+++

Refer to the following sections for details about the support and usage thresholds.

## Support threshold

For packages that are not dependencies for Progress Chef Tools:

- There should be no published End of Life (EOL) date within the first two years of the LTS-YYYY support duration at the time of consideration. If the OEM drops support or does not provide a fix to known vulnerabilities during the LTS timeframe, no updates will be made available.
- There should be no open High or Critical CVEs for that package at the time of consideration. If any released package has a Critical/High CVE post-release, it will be fixed as part of the next minor/directed refresh.
- It should have a valid licensing model that allows Progress Chef to distribute it as a Habitat Package.

## Usage threshold

A requested package will be added for backlog prioritization if it belongs to one of the following categories:

- Commonly-used development tools/programming languages
- High demand COTS
