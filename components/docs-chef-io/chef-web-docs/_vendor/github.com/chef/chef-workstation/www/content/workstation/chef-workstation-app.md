+++
title = "Chef Workstation App"
draft = false

[menu]
  [menu.workstation]
    title = "Chef Workstation App"
    identifier = "chef_workstation/chef_workstation_tools/chef_workstation_app.md Chef Workstation App"
    parent = "chef_workstation/chef_workstation_tools"
    weight = 61
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/chef-workstation-app.md)

# About Chef Workstation App

The Chef Workstation App (CWA) is an early-release desktop application that
provides additional services for Chef Workstation:

* Update checking and notifications
* Chef Workstation version information

Additional features and integrations will be rolled out in future updates.

## Running the Chef Workstation App

### Windows

Launch Chef Workstation App from the Chef Workstation folder in the Start menu.

### Linux

Start Chef Workstation App by running the command `chef-workstation-app`.

#### Notes

1. Chef Workstation App requires a running display manager with support for
   system tray icons.
1. On some distributions you may need to install additional libraries.  The
   post-install message shown when you install the Chef Workstation package
   from the terminal will tell you which -- if any -- additional libraries are
   required to run Chef Workstation App.

### Mac

Start `Chef Workstation App` from your Applications folder.

## Disabling Automatic Update Checks

To disable CWA's automatic update checking, add or modify the `enable` setting
under `updates` in [config.toml]({{< ref "config.md#updates" >}}):

```toml
[updates]
enable=false
```

## Setting Update Channel

To set the channel used for update checking, bring up the CWA tray app
menu (Windows/Linux) or the application menu (Mac) and select "About Chef
Workstation".

Select the "Channel" button and choose your preferred channel.  This will
trigger an immediate update check.

```toml
[updates]
channel="current"
```
