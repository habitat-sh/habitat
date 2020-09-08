+++
title = "push-jobs-client.rb"
draft = false

aliases = ["/config_rb_push_jobs_client.html"]

[menu]
  [menu.infra]
    title = "push-jobs-client.rb"
    identifier = "chef_infra/managing_chef_infra_server/push_jobs/config_rb_push_jobs_client.md push-jobs-client.rb"
    parent = "chef_infra/managing_chef_infra_server/push_jobs"
    weight = 30
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/config_rb_push_jobs_client.md)

A `push-jobs-client.rb` file is used to specify the configuration
details for the Chef Push Jobs client.

-   This file is loaded every time this executable is run
-   This file is not created by default
-   When a `push-jobs-client.rb` file is present in the default
    location, the settings contained within that file will override the
    default configuration settings

## Settings

This configuration file has the following settings:

`allow_unencrypted`

:   Allow unencrypted connections to 1.x servers. Default value: `false`

`chef_server_url`

:   The URL for the Chef Infra Server. For example:

    ``` ruby
    https://localhost/organizations/ORG_NAME
    ```

`client_key`

:   The location of the file that contains the client key. Default
    value: `/etc/chef/client.pem`.

`file_dir`

:   The directory for temporary files. Default value: `/tmp/chef-push`.

`node_name`

:   The name of the node.

`ssl_verify_mode`

:   Set the verify mode for HTTPS requests.

    -   Use `:verify_none` to do no validation of SSL certificates.
    -   Use `:verify_peer` to do validation of all SSL certificates,
        including the Chef Infra Server connections, S3 connections, and
        any HTTPS **remote_file** resource URLs used in a Chef Infra
        Client run. This is the recommended setting.

    Depending on how OpenSSL is configured, the `ssl_ca_path` may need
    to be specified. Default value: `:verify_peer`.

`trusted_certs_dir`

:   The location of trusted certificates. Default value:
    `'/etc/chef/trusted_certs'`.

`whitelist`

:   A Hash that contains the whitelist used by Chef Push Jobs. For
    example:

    ``` ruby
    whitelist {
      'job-name' => 'command',
      'job-name' => 'command',
      'chef-client' => 'chef-client'
    }
    ```
