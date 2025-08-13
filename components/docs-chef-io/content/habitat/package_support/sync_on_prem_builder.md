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

To sync packages from the core, chef, and chef-platform origins with on-prem Habitat Builder deployments, we've created scripts to fetch packages from the public Habitat Builder. These scripts pull packages released through the LTS and innovation channels.

The sync script works as follows:

1. It performs a pre-flight check to list packages in the core origin for the specified channel (for example, LTS-YYYY or Innovation-YYYY) that aren't created or maintained by Progress Chef.
1. If you proceed with the script:
   1. Those packages are demoted to the unstable channel.
   1. Packages are downloaded from the specified channel (for example, LTS-YYYY or Innovation-YYYY) on Chef's Habitat Builder and uploaded to your Habitat Builder deployment.
