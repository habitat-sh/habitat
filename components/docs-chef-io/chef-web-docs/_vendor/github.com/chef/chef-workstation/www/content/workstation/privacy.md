+++
title = "Privacy and Telemetry"
draft = false

[menu]
  [menu.workstation]
    title = "Privacy and Telemetry"
    identifier = "chef_workstation/privacy.md Privacy and Telemetry"
    parent = "chef_workstation"
    weight = 20
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/privacy.md)

In order to continually improve Chef Workstation, we collect information to help us identify bugs and understand how people interact with Chef Workstation.

## What We Collect

We capture:

* A unique installation-id that isn't connected to user data. This helps us track the number of active Chef Workstation installations without needing to perform IP-based tracking.
* The Chef-specific commands you execute, but **none** of the arguments you pass.
* Your host operating system and version.
* A SHA256 sum of any hostname that you're connecting to via `chef-run`.
* How you connect to a remote host via `chef-run`, either WinRM or SSH.
* Target operating system of any hosts connected to via `chef-run`.

## How We Use Your Data

We use this data to track Chef Workstation usage patterns, identify bugs, and iterate development based real aggregated feedback.

Only Chef Software, Inc employees have access to your data.
We will never sell, re-sell, or use your data in a malicious manner.

## Opting out

* To stop the capture of telemetry data from a single session, set the environment variable `CHEF_TELEMETRY_OPT_OUT` to any value before running `chef-run`, for example:

  ```bash
  CHEF_TELEMETRY_OPT_OUT=1 chef-run -h
  ```

* Disable telemetry entirely by adding the following to `$HOME/.chef-workstation/config.toml`:

  ```bash
  [telemetry]
  enabled=false
  ```

## See Your Data

You can view the analytics we collect before it is sent.
Find&#8212;and remove&#8212;your data in the `HOME/.chef-workstation/telemetry/` folder.
We save the data from a current `chef-run` in the telemetry folder and collect it at the start of the next `chef-run`.

When telemetry is disabled, we won't collect your previously stored analytics.
