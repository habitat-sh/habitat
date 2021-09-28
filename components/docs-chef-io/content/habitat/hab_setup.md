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
[\[edit on GitHub\]](https://github.com/habitat-sh/habitat/blob/master/components/docs-chef-io/content/habitat/hab_setup.md)

Once Chef Habitat has been installed, the `hab` CLI makes it easy to get your workstation configured by guiding through the setup process. To set up your workstation, run `hab cli setup` and follow the instructions.

<img alt="screenshot of hab cli setup output in CLI" src="/images/habitat/hab-setup.png">

Setup asks you to create a new origin and a set of origin keys.

Optionally, you can also provide a Chef Habitat personal access token to upload packages to the public depot and share them with the Chef Habitat community. See the [access token documentation]({{< relref "builder_profile#create-a-personal-access-token" >}}) for details on generating and using your access token.

> For more information about using Chef Habitat Builder, see the section on [Using Builder]({{< relref "builder_overview" >}}).

You will also be asked if you want to register Supervisor control gateway secret (see [Remote Command-and-Control of Supervisors]({{< relref "sup_remote_control" >}}) for further details).

You can change your settings at any time by re-running the `hab cli setup` command.

<img alt="screenshot of completed hab cli setup in CLI" src="/images/habitat/hab-setup-complete.png">

That's it. You're all set up and ready to use Chef Habitat to build and run packages!
