+++
title = "knife ssl_fetch"
draft = false

aliases = ["/knife_ssl_fetch.html", "/knife_ssl_fetch/"]

[menu]
  [menu.workstation]
    title = "knife ssl_fetch"
    identifier = "chef_workstation/chef_workstation_tools/knife/knife_ssl_fetch.md knife ssl_fetch"
    parent = "chef_workstation/chef_workstation_tools/knife"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/knife_ssl_fetch.md)

{{% knife_ssl_fetch_summary %}}

## Syntax

This subcommand has the following syntax:

``` bash
knife ssl fetch (options)
```

## Options

This subcommand has the following options:

`URL_or_URI`

:   The URL or URI for the location at which the SSL certificate is
    located. Default value: the URL for the Chef Infra Server, as
    defined in the config.rb file.

## Examples

The following examples show how to use this knife subcommand:

**Fetch the SSL certificates used by Knife from the Chef server**

``` bash
knife ssl fetch
```

The response is similar to:

``` bash
WARNING: Certificates from <chef_server_url> will be fetched and placed in your trusted_cert
directory (/Users/grantmc/chef-repo/.chef/trusted_certs).

Knife has no means to verify these are the correct certificates. You should
verify the authenticity of these certificates after downloading.

Adding certificate for <chef_server_url> in /Users/grantmc/chef-repo/.chef/trusted_certs/grantmc.crt
Adding certificate for DigiCert Secure Server CA in /Users/grantmc/chef-repo/.chef/trusted_certs/DigiCert_Secure_Server_CA.crt
```

**Fetch SSL certificates from a URL or URI**

``` bash
knife ssl fetch https://www.example.com
```

**Verify Checksums**

{{% knife_ssl_fetch_verify_certificate %}}
