+++
title = "config.rb"
draft = false

aliases = ["/config_rb.html", "/config_rb_knife.html", "/config_rb/"]

[menu]
  [menu.workstation]
    title = "config.rb (knife.rb)"
    identifier = "chef_workstation/chef_workstation_tools/knife/config_rb.md config.rb (knife.rb)"
    parent = "chef_workstation/chef_workstation_tools/knife"
    weight = 40
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/config_rb.md)

{{< warning >}}

The `config.rb` file is a replacement for the knife.rb file, starting
with the Chef Client 12.0 release. The `config.rb` file has identical
settings and behavior to the knife.rb file. Chef Infra Client looks
first for the presence of the `config.rb` file and if it is not found,
then looks for the `knife.rb` file.

{{< /warning >}}

A `config.rb` file is used to specify configuration details for knife.

A `config.rb` file:

-   Is loaded every time the knife executable is run
-   The default location at which Chef Infra Client expects to find this
    file is `~/.chef/config.rb`; use the `--config` option from the
    command line to change this location
-   Is not created by default
-   Is located by default at `~/.chef/config.rb` (UNIX and Linux
    platforms) or `c:\Users\username\.chef` (Microsoft Windows platform,
    use the `--config` option from the command line to change this
    location
-   Will override the default configuration when a `config.rb` file
    exists at the default path or the path specified by the `--config`
    option

{{< note >}}

When running Microsoft Windows, the `config.rb` file is located at
`%HOMEDRIVE%:%HOMEPATH%\.chef` (e.g. `c:\Users\<username>\.chef`). If
this path needs to be scripted, use `%USERPROFILE%\chef-repo\.chef`.

{{< /note >}}

## Settings

This configuration file has the following settings:

`bootstrap_template`

:   The path to a template file to be used during a bootstrap operation.

`chef_server_url`

:   The URL for the Chef Infra Server. For example:

    ``` ruby
    chef_server_url 'https://localhost/organizations/ORG_NAME'
    ```

`chef_zero.enabled`

:   Enable chef-zero. This setting requires `local_mode` to be set to
    `true`. Default value: `false`. For example:

    ``` ruby
    chef_zero.enabled true
    ```

`chef_zero[:port]`

:   The port on which chef-zero is to listen. Default value: `8889`. For
    example:

    ``` ruby
    chef_zero[:port] 8889
    ```

`client_d_dir`

:   A directory that contains additional configuration scripts to load
    for Chef Infra Client.

`client_key`

:   The location of the file that contains the client key, as an
    absolute path. Default value: `/etc/chef/client.pem`. For example:

    ``` ruby
    client_key '/etc/chef/client.pem'
    ```

`cookbook_copyright`

:   The name of the copyright holder. This option places a copyright
    notice that contains the name of the copyright holder in each of the
    pre-created files. If this option is not specified, a copyright name
    of "COMPANY_NAME" is used instead; it can easily be modified later.

`cookbook_email`

:   The email address for the individual who maintains the cookbook.
    This option places an email address in each of the pre-created
    files. If not specified, an email name of "YOUR_EMAIL" is used
    instead; this can easily be modified later.

`cookbook_license`

:   The type of license under which a cookbook is distributed:
    `apachev2`, `gplv2`, `gplv3`, `mit`, or `none` (default). This
    option places the appropriate license notice in the pre-created
    files: `Apache v2.0` (for `apachev2`), `GPL v2` (for `gplv2`),
    `GPL v3` (for `gplv3`), `MIT` (for `mit`), or
    `license 'Proprietary - All Rights Reserved` (for `none`). Be aware
    of the licenses for files inside of a cookbook and be sure to follow
    any restrictions they describe.

`cookbook_path`

:   The Chef Infra Client sub-directory for storing cookbooks. This
    value can be a string or an array of file system locations,
    processed in the specified order. The last cookbook is considered to
    override local modifications. For example:

    ``` ruby
    cookbook_path [
      '/var/chef/cookbooks',
      '/var/chef/site-cookbooks'
    ]
    ```

`data_bag_encrypt_version`

:   The minimum required version of data bag encryption. Possible
    values: `1` or `2`. It is recommended that this value be set
    to `2` to provide additional security. For example:

    ``` ruby
    data_bag_encrypt_version 2
    ```

`fips`

:   Allows OpenSSL to enforce FIPS-validated security during a Chef
    Infra Client run. Set to `true` to enable FIPS-validated security.

    The following operating systems are supported:

    -   Red Hat Enterprise Linux
    -   Oracle Enterprise Linux
    -   CentOS
    -   Windows

    Support for FIPS was introduced in Chef Server version 12.13. The
    following operating systems are supported:

    -   Red Hat Enterprise Linux
    -   Oracle Enterprise Linux
    -   CentOS

`local_mode`

:   Run Chef Infra Client in local mode. This allows all commands that
    work against the Chef Infra Server to also work against the local
    chef-repo. For example:

    ``` ruby
    local_mode true
    ```

`node_name`

:   The name of the node. This may be a username with permission to
    authenticate to the Chef Infra Server or it may be the name of the
    machine from which knife is run. For example:

    ``` ruby
    node_name 'user_name'
    ```

    or:

    ``` ruby
    node_name 'machine_name'
    ```

`no_proxy`

:   A comma-separated list of URLs that do not need a proxy. Default
    value: `nil`. For example:

    ``` ruby
    no_proxy 'localhost, 10.0.1.35, *.example.com, *.dev.example.com'
    ```

`ssh_agent_signing`

:   **New in 14.2** Use `ssh-agent` to authenticate. When using this
    option, specify the location of the public key in `client_key`.
    Default value: `false`. Ensure the public key is in PKCS\#1 format.
    You can convert an OpenSSH public key using `ssh-keygen`. For
    example:

    ``` bash
    ssh-keygen -f key.pub -e -m pem > key.pem
    ```

`ssh_timeout`

:   The amount of time (in seconds) to wait for an SSH connection time
    out.

`ssl_verify_mode`

:   Set the verify mode for HTTPS requests.

    -   Use `:verify_none` to do no validation of SSL certificates.
    -   Use `:verify_peer` to do validation of all SSL certificates,
        including the Chef Infra Server connections, S3 connections, and
        any HTTPS **remote_file** resource URLs used in a Chef Infra
        Client run. This is the recommended setting.

    Depending on how OpenSSL is configured, the `ssl_ca_path` may need
    to be specified. Default value: `:verify_peer`.

`syntax_check_cache_path`

:   All files in a cookbook must contain valid Ruby syntax. Use this
    setting to specify the location in which knife caches information
    about files that have been checked for valid Ruby syntax.

`tmux_split`

:   Split the Tmux window. Default value: `false`.

`validation_client_name`

:   The name of the chef-validator key that is used by Chef Infra Client
    to access the Chef Infra Server during the initial Chef Infra Client
    run. For example:

    ``` ruby
    validation_client_name 'chef-validator'
    ```

`validation_key`

:   The location of the file that contains the key used when a Chef
    Infra Client is registered with a Chef Infra Server. A validation
    key is signed using the `validation_client_name` for authentication.
    Default value: `/etc/chef/validation.pem`. For example:

    ``` ruby
    validation_key '/etc/chef/validation.pem'
    ```

`verify_api_cert`

:   Verify the SSL certificate on the Chef Infra Server. When `true`,
    Chef Infra Client always verifies the SSL certificate. When `false`,
    Chef Infra Client uses the value of `ssl_verify_mode` to determine
    if the SSL certificate requires verification. Default value:
    `false`.

`versioned_cookbooks`

:   Append cookbook versions to cookbooks. Set to `false` to hide
    cookbook versions: `cookbooks/apache`. Set to `true` to show
    cookbook versions: `cookbooks/apache-1.0.0` and/or
    `cookbooks/apache-1.0.1`. When this setting is `true`,
    `knife download` downloads ALL cookbook versions, which can be
    useful if a full-fidelity backup of data on the Chef Infra Server is
    required. For example:

    ``` ruby
    versioned_cookbooks true
    ```

`config_log_level`

:   Sets the default value of `log_level` in the client.rb file of the
    node being bootstrapped. Possible values are `:debug`, `:info`,
    `:warn`, `:error` and `:fatal`. For example:

    ``` ruby
    config_log_level :debug
    ```

`config_log_location`

:   Sets the default value of `log_location` in the client.rb file of
    the node being bootstrapped. Possible values are
    `/path/to/log_location`, `STDOUT`, `STDERR`, `:win_evt` and
    `:syslog`. For example:

    ``` ruby
    config_log_location "/path/to/log_location"   # Please make sure that the path exists
    ```

### Proxy Settings

In certain situations the proxy used by the Chef Infra Server requires
authentication. In this situation, three settings must be added to the
configuration file. Which settings to add depends on the protocol used
to access the Chef Infra Server: HTTP or HTTPS.

If the Chef Infra Server is configured to use HTTP, add the following
settings:

`http_proxy`

:   The proxy server for HTTP connections. Default value: `nil`. For
    example:

    ``` ruby
    http_proxy 'http://proxy.example.com:3128'
    ```

`http_proxy_user`

:   The user name for the proxy server when the proxy server is using an
    HTTP connection. Default value: `nil`.

`http_proxy_pass`

:   The password for the proxy server when the proxy server is using an
    HTTP connection. Default value: `nil`.

If the Chef Infra Server is configured to use HTTPS (such as the hosted
Chef Infra Server), add the following settings:

`https_proxy`

:   The proxy server for HTTPS connections. (The hosted Chef Infra
    Server uses an HTTPS connection.) Default value: `nil`.

`https_proxy_user`

:   The user name for the proxy server when the proxy server is using an
    HTTPS connection. Default value: `nil`.

`https_proxy_pass`

:   The password for the proxy server when the proxy server is using an
    HTTPS connection. Default value: `nil`.

Use the following setting to specify URLs that do not need a proxy:

`no_proxy`

:   A comma-separated list of URLs that do not need a proxy. Default
    value: `nil`.

## .d Directories

{{% config_rb_client_dot_d_directories %}}

## Optional Settings

In addition to the default settings in a `config.rb` file, there are
other subcommand-specific settings that can be added:

1.  A value passed via the command-line
2.  A value contained in the `config.rb` file
3.  The default value

A value passed via the command line will override a value in the
`config.rb` file; a value in a `config.rb` file will override a default
value. Before adding any settings to the `config.rb` file:

-   Verify the settings by reviewing the documentation for the knife
    subcommands and/or knife plugins
-   Verify the use case(s) your organization has for adding them

Also note that:

-   Custom plugins can be configured to use the same settings as the
    core knife subcommands
-   Many of these settings are used by more than one subcommand and/or
    plugin
-   Some of the settings are included only because knife checks for a
    value in the `config.rb` file

To add settings to the `config.rb` file, use the following syntax:

``` ruby
knife[:setting_name] = value
```

where `value` may require quotation marks (' ') if that value is a
string. For example:

``` ruby
knife[:ssh_port] = 22
knife[:bootstrap_template] = 'ubuntu14.04-gems'
knife[:bootstrap_version] = ''
knife[:bootstrap_proxy] = ''
```

Some of the optional `config.rb` settings are used often, such as the
template file used in a bootstrap operation. The frequency of use of any
option varies from organization to organization, so even though the
following settings are often added to a `config.rb` file, they may not
be the right settings to add for every organization:

`knife[:bootstrap_proxy]`

:   The proxy server for the node that is the target of a bootstrap
    operation.

`knife[:bootstrap_template]`

:   The path to a template file to be used during a bootstrap operation.

`knife[:bootstrap_version]`

:   The version of Chef Infra Client to install.

`knife[:editor]`

:   The \$EDITOR that is used for all interactive commands.

`knife[:ssh_gateway]`

:   The SSH tunnel or gateway that is used to run a bootstrap action on
    a machine that is not accessible from the workstation. Adding this
    setting can be helpful when a user cannot SSH directly into a host.

`knife[:ssh_port]`

:   The SSH port.

Other SSH-related settings that are sometimes helpful when added to the
`config.rb` file:

`knife[:forward_agent]`

:   Enable SSH agent forwarding.

`knife[:ssh_attribute]`

:   The attribute used when opening an SSH connection.

`knife[:ssh_password]`

:   The SSH password. This can be used to pass the password directly on
    the command line. If this option is not specified (and a password is
    required) knife prompts for the password.

`knife[:ssh_user]`

:   The SSH user name.

Some organizations choose to have all data bags use the same secret and
secret file, rather than have a unique secret and secret file for each
data bag. To use the same secret and secret file for all data bags, add
the following to config.rb:

`knife[:secret]`

:   The encryption key that is used for values contained within a data
    bag item.

`knife[:secret_file]`

:   The path to the file that contains the encryption key.

Some settings are better left to Ohai, which will get the value at the
start of a Chef Infra Client run:

`knife[:server_name]`

:   Same as node_name. Recommended configuration is to allow Ohai to
    collect this value during each Chef Infra Client run.

`node_name`

:   See the description above for this setting.

{{< warning >}}

Review the full list of [optional
settings](/workstation/config_rb_optional_settings/) that can be added to the
`config.rb` file. Many of these optional settings should not be added to
the `config.rb` file. The reasons for not adding them can vary. For
example, using `--yes` as a default in the `config.rb` file will cause
knife to always assume that "Y" is the response to any prompt, which may
lead to undesirable outcomes. Other settings, such as `--hide-by-mins`
(used only with the `knife status` subcommand) or `--bare-directories`
(used only with the `knife list` subcommand) probably aren't used often
enough (and in the same exact way) to justify adding them to the
`config.rb` file. In general, if the optional settings are not listed on
<span class="title-ref">the main </span><span
class="title-ref">config.rb</span>[topic](/workstation/config_rb/), then add
settings only after careful consideration. Do not use optional settings
in a production environment until after the setting's performance has
been validated in a safe testing environment.

{{< /warning >}}
