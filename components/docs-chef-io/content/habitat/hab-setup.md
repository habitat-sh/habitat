+++
title = "Configure the Chef Habitat CLI"
description = "Set up the Chef Habitat CLI"

[menu]
  [menu.habitat]
    title = "Set up the Chef Habitat CLI"
    identifier = "habitat/get_started/hab-setup Install Chef Habitat"
    parent = "habitat/get_started"
    weight = 20

+++

Once Chef Habitat has been installed, the `hab` CLI makes it easy to get your workstation configured by guiding through the setup process. To set up your workstation, run `hab cli setup` and follow the instructions.

<img alt="screenshot of hab cli setup output in CLI" src="/images/habitat/hab-setup.png">

Setup asks you to create a new origin and a set of origin keys.

Optionally, you can also provide a Chef Habitat personal access token to upload packages to the public depot and share them with the Chef Habitat community. See the [access token documentation](/using-builder/#builder-token) for details on generating and using your access token.

> For more information about using Chef Habitat Builder, see the section on [Using Builder](/using-builder/).

You will also be asked if you want to register Supervisor control gateway secret (see [Remote Command-and-Control of Supervisors](/using-habitat/#remote-control) for further details).

During setup, you may elect to provide anonymous usage data of the `hab` CLI tool. This information is used by the Chef Habitat team to improve the CLI experience.
For information on the types of data we gather and how we intend to use it, see [Analytics in Chef Habitat](/about-analytics).

You can change your settings at any time by re-running the `hab cli setup` command.

<img alt="screenshot of completed hab cli setup in CLI" src="/images/habitat/hab-setup-complete.png">

That's it. You're all set up and ready to use Chef Habitat to build and run packages!
