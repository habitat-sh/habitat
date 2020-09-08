+++
title = "System Requirements"

weight = 20
draft = false

[menu]
  [menu.automate]
    title = "System Requirements"
    parent = "automate/getting_started"
    identifier = "automate/getting_started/system_requirements.md System Requirements"
    weight = 20
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/system_requirements.md)

## Hardware

Chef Automate requires a minimum of

* 16 GB of RAM
* 80 GB of disk space (available to `/hab`)
* 4 vCPUs

## Operating system

Chef Automate requires

* a Linux kernel of version 3.2 or greater
* `systemd` as the init system
* `useradd`
* `curl` or `wget`
* The shell that starts Automate should have a max open files setting of at least 65535

Commercial support for Chef Automate is available for platforms that satisfy these
criteria.

## Supported Browsers

Chef Automate supports the current browser versions for Chrome, Edge, and Firefox. Chef
Automate does not support other browsers and may not be compatible with older browser
versions.
