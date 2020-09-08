+++
title = "Release Notes: Chef Manage 3.0"
draft = false

aliases = ["/release_notes_manage.html"]

[menu]
  [menu.release_notes]
    title = "Chef Manage"
    identifier = "release_notes/release_notes_manage.md Chef Manage"
    parent = "release_notes"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/release_notes_manage.md)

Chef Manage provides a web-based user interface that manages Chef Infra nodes and
other policy objects, such as data bags and roles, on the instance of Chef Infra Server
that it is installed on.

Chef Manage is [deprecated](/versions/#deprecated) and users should plan to
migrate to [Chef Automate](/automate/) as the replacement.

## Upgrading

Download the latest version of the chef-manage package for your platform from
[downloads.chef.io](https://downloads.chef.io/manage) to your Chef Infra Server,
then run:

```
# rpm -Uvh /path/to/chef-manage-*.rpm
or 
# dpkg -i /path/to/chef-manage-*.deb

then
# chef-manage-ctl reconfigure
# chef-server-ctl reconfigure
```

## What's New in 3.0

This release includes a number of important dependency updates and support for
SSL connections to the Chef Infra Server.

### Dependency Updates

* Rails is upgraded to 5.2.3
* Chef Infra Client is upgraded from 11 to 14
* Sidekiq is upgraded from 2.5 to 4 to improve the background processing

### SSL Support

The major version upgrade of Chef Infra Client in this release brings in SSL support 
between Chef Manage and Chef Infra Server. These services always run on the
same system so this feature is of limited value in most cases. For compatibility
with self-signed certificates which are commonly used on the Chef Infra Server, this
feature defaults to off in this release.

#### Configuration

The SSL support is configured using these settings in the
`/etc/chef-manage/manager.rb` configuration file. After any changes you must run
`sudo chef-manage-ctl reconfigure` to apply the changes.

`ssl_verify_mode` : Configure SSL verification for the connection to the Chef Infra
Server. By default this is set to `:verify_none`. Setting this to `:verify_peer`
will require a Chef Manage to verify the SSL certificate of the Chef Infra Server.

`trusted_certs_dir`: Provide a path to a directory containing trusted SSL
certificates. This is empty by default, but `/etc/chef/trusted_certs` is the
most likely setting for reusing certificates installed by Chef Infra Client.


### Bug Fixes

Generating a Knife Config now produces a `config.rb` instead of `knife.rb` file.
These have the same format and contents but `config.rb` has been preferred since
Chef Infra Client 12.0. (chef-manage#925)

Minor UI bug fixes (chef-manage#941)

### Supported Versions

Chef Manage 3.0 is compatible with Chef Infra Server 12 and later. Users are
always encouraged to stay up to date on Chef Infra Server releases to ensure they
have the latest security updates.

#### Added Platform Support

RHEL 7 
RHEL 8 
Ubuntu 16.04 
Ubuntu 18.04

#### Removed Platform Support

Ubuntu 10.04 
Ubuntu 12.04 
Ubuntu 14.04

