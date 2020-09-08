+++
title = "Chef Desktop Cookbook Reference"
draft = false
publishDate = "2020-06-02"

[menu]
  [menu.desktop]
    title = "Chef Desktop Cookbook Reference"
    identifier = "desktop/desktop-cookbook.md Desktop Cookbook"
    parent = "desktop"
    weight = 100
+++

[\[edit on GitHub\]](https://github.com/chef/desktop-config/blob/master/docs/content/desktop/desktop-cookbook.md)

# Chef Desktop

The Chef Desktop cookbook provides Windows and macOS desktop administrators a straightforward experience for configuring and managing remote devices without requiring deep command-line knowledge or experience with Ruby or Chef Infra. Administrators can set common configuration options in Chef Desktop cookbook, such as using FileVault or BitLocker drive encryption, and then deploy the configuration to the fleet.

## Requirements

The Chef Desktop cookbook installs onto your development environment. Once configured and tested, you then upload the Chef Desktop cookbook to your Chef Infra Server. From there, you deliver the Chef Desktop cookbook to your managed devices by adding the cookbook to the runlist in one of your Policyfiles. Like the Chef Desktop cookbook, you edit the Policyfiles for your Windows and macOS devices in your development environment and upload them to the Chef Infra Server.

### Supported Platforms

- Microsoft Windows 10
- macOS 10.13 and later

### Chef Infra Client

- Chef Infra Client 16+

## Recipes

default.rb
: The Chef Desktop cookbook's default recipe finds the underlying operating system (OS) and then calls the operating system-specific recipe. Each OS recipe is easy to understand and configure.

windows.rb
: The Windows recipe is a series of settings for Windows 10 desktops. Example:

  ```ruby
  disk_encryption 'Turns on BitLocker Drive Encryption' do
    action :enable
    # valid options include :enable, :disable, :nothing
  end
  ```

mac.rb
: The macOS recipe follows the same format, and as much as possible, the same spelling and parameters for a strong consistency between the recipes and no change in verbiage. Example:

  ```ruby
  chef_client_launchd 'Setup the Chef client to run every 30 minutes' do
    interval 30
    action :enable
    # valid options include :enable, :disable
  end
  ```

### Usage

Policyfile
: Policyfiles are the preferred way to manage role, environment, and community cookbook data. It is a single file that you upload to Chef Infra Server. A Policyfile is associated with a group of nodes, cookbooks, and settings. When these nodes perform a Chef Infra Client run, they use the recipes specified in the Policyfile run-list. Versioned Policyfiles are safe to promote through your deployment pipeline and reliably deploy new configuration settings.

This cookbook uses a Policyfile, which is preset to run the cookbook when you call from the directory above where your Chef Desktop cookbook lives. Example:

  ```shell
  chef-client -z -o chefdesktop
  ```

## Resources

Resources with operating system label prefixes indicate the operating system they belong to, such as `osName_app_management` or `(os name) disk_encryption`. Resources with no operating system label prefix work interchangeably between macOS and Windows.

See the [Desktop resources documentation](/desktop/resources/) for information about each resource.
