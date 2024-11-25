+++
title = "Syncing packages to the on-prem Habitat Builder"

gh_repo = "habitat"

[menu]
  [menu.habitat]
    title = "Syncing to on-prem Builder"
    identifier = "habitat/packages/support/syncing_packages_to_on_prem_builder"
    parent = "habitat/packages/support"
    weight = 13
+++

A sync script will be provided that will:

1. Perform a pre-flight check that returns a list of packages under core origin for that channel (for example, LTS-YYYY or Innovation-YYYY) that aren't created or maintained by Progress Chef.
1. If proceeding with the script:
   1. Those packages will be demoted to the unstable channel.
   1. Packages are downloaded from the channel specified (for example, LTS-YYYY or Innovation-YYYY) from Public Builder and uploaded to their respective on-prem builders.
