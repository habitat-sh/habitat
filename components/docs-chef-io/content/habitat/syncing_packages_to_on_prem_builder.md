+++
title = "Syncing packages to the on-prem builder"
description = ""
gh_repo = "habitat"

[menu]
  [menu.habitat]
    title = "Syncing packages to the on-prem builder"
    identifier = "habitat/packages/support/package_support/syncing_packages_to_on_prem_builder"
    parent = "habitat/packages/package_support"
    weight = 13
+++

A sync script will be provided that will:

1. Perform a pre-flight check that returns a list of packages under core origin for that channel (for example, LTS-YYYY or Innovation-YYYY) that are not created/maintained by Progress Chef.
1. If proceeding with the script:
   1. Those packages will be demoted to the unstable channel.
   1. Points to the new channel (for example, LTS-YYYY or Innovation-YYYY) for downloading packages from the Public Builder and uploading them to their respective on-prem builders.
