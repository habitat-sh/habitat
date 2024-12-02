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

To sync packages hosted by the core, chef, and chef-platform origins with on-prem deployments of Habitat Builder,
we've created scripts to sync packages from the public Habitat Builder.
These scripts fetch packages released through the LTS and innovation channels.

The sync script does the following:

1. It performs a pre-flight check that returns a list of packages under the core origin for that channel (for example, LTS-YYYY or Innovation-YYYY) that aren't created or maintained by Progress Chef.
1. If proceeding with the script:
   1. Those packages are demoted to the unstable channel.
   1. Packages are downloaded from the channel specified (for example, LTS-YYYY or Innovation-YYYY) from the public Habitat Builder and uploaded to the on-prem Builder.
