+++
title = "Syncing packages to the on-prem builder"
description = ""
gh_repo = "habitat"

[menu]
  [menu.habitat]
    title = "Syncing packages to the on-prem builder"
    identifier = "habitat/packages/support/package_support/syncing_packages_to_on_prem_builder"
    parent = "habitat/packages/support"
    weight = 13
+++

A sync script will be provided that will:

1. Perform a pre-flight check that returns a list of packages under core origin for that channel (for example, LTS-YYYY or Innovation-YYYY) that are not created/maintained by Progress Chef.
1. If proceeding with the script:
   1. Those packages will be demoted to the unstable channel.
   1. Packages are downloaded from the channel specified (for example, LTS-YYYY or Innovation-YYYY) from Public Builder and uploaded to their respective on-prem builders.
