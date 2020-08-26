+++
title = "Upgrading Chef Habitat Builder on-prem"
description = ""

[menu]
  [menu.habitat]
    title = "Upgrading"
    identifier = "habitat/builder-on-prem/"
    parent = "habitat"

+++

Chef Habitat Builder on-prem services are not set to auto-upgrade. To upgrade Chef Habitat Builder on-prem, first stop and unistall the services, with our uninstall script.

The Chef Habitat Builder on-prem GitHub repository is located at [https://github.com/habitat-sh/on-prem-builder/](https://github.com/habitat-sh/on-prem-builder/)

To uninstall Chef Habitat Builder on-prem:

1. `cd ${SRC_ROOT}`
1. `sudo ./uninstall.sh`

To update Chef Habitat Builder on-prem:
1. `cd ${SRC_ROOT}`
2. `git checkout master && git pull`
3. `./install.sh`

*IMPORTANT*: Running the uninstall script will *NOT* remove any user data, so you can freely uninstall and re-install the services.
