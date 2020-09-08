+++
title = "client.rb"
draft = false

aliases = ["/config_rb_client.html"]

[menu]
  [menu.infra]
    title = "client.rb"
    identifier = "chef_infra/setup/nodes/config_rb_client.md client.rb"
    parent = "chef_infra/setup/nodes"
    weight = 40
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/config_rb_client.md)

{{% config_rb_client_summary %}}

## Settings

This configuration file has the following settings:

`add_formatter`

:   A 3rd-party formatter. (See
    [nyan-cat](https://github.com/andreacampi/nyan-cat-chef-formatter)
    for an example of a 3rd-party formatter.) Each formatter requires
    its own entry.

`authentication_protocol_version`

:   Sets the authentication protocol that is used to communicate with
    Chef Infra Server. For example, specify protocol version 1.3 to
    enable support for SHA-256 algorithms:

    ``` ruby
    knife[:authentication_protocol_version] = '1.3'
    ```

    {{< note spaces=4 >}}

    Authentication protocol 1.3 is only supported on Chef Server
    versions 12.4.0 and above.

    {{< /note >}}

`automatic_attribute_blacklist`

:   A hash that blacklists `automatic` attributes, preventing
    blacklisted attributes from being saved.

`automatic_attribute_whitelist`

:   A hash that whitelists `automatic` attributes, preventing
    non-whitelisted attributes from being saved.

`cache_path`

:   The home directory for the user that runs Chef Infra Client as a
    non-root user.

`checksum_path`

:   The location in which checksum files are stored. These are used to
    validate individual cookbook files, such as recipes. The checksum
    itself is stored in the Chef Infra Server database and is then
    compared to a file in the checksum path that has a filename
    identical to the checksum.

`chef_guid`

:   The node UUID used by Automate. Setting this allows the node UUID to
    be specified, and can be carried across instances of a node.

`chef_license`

:   Used to accept the Chef license. Can be set to `accept` or
    `accept-no-persist`, which persists the license acceptance to disk.
    If passed to versions where the license is not required this
    configuration option is a no-op.

`chef_repo_path`

:   The path to the chef-repo containing cookbooks and other files, such
    as environments or data bags, when running Chef Infra Client in
    local mode.

`chef_server_url`

:   The URL for the Chef Infra Server. For example:

    ``` ruby
    https://localhost/organizations/ORG_NAME
    ```

`chef_zero.enabled`

:   Enable chef-zero. This setting requires `local_mode` to be set to
    `true`. Default value: `true` if running in local-mode, otherwise
    `false`.

`chef_zero.port`

:   The port on which chef-zero is to listen. If specified as a range,
    Chef Infra Client will take the first available port in the range.
    For example `10,20,30` or `10000-20000`. Default value: `8889-9999`.

`clear_gem_sources`

:   Globally sets the default of the `clear_sources` property on the
    `gem_package` and `chef_gem` resources. Default value: `false`.

`client_fork`

:   Contain Chef Infra Client runs in a secondary process with dedicated
    RAM. When a Chef Infra Client run is complete, the RAM is returned
    to the master process. This setting helps ensure that a Chef Infra
    Client uses a steady amount of RAM over time because the master
    process does not run recipes. This setting also helps prevent memory
    leaks such as those that can be introduced by the code contained
    within a poorly designed cookbook. Default value: `true`. Set to
    `false` to disable running Chef Infra Client in fork node.

    {{< note spaces=4 >}}

    Must be set to `false` up to Chef Client 13.11.3 to gather the
    standard return code offered by `exit_status true`. Later versions
    run as expected without changes to the configuration file.

    {{< /note >}}

`client_key`

:   The location of the file that contains the client key. Default
    value: `/etc/chef/client.pem`.

`client_registration_retries`

:   The number of times a Chef Infra Client will attempt to register
    with a Chef Infra Server. Default value: `5`.

`client_d_dir`

:   A directory that contains additional configuration files for Chef
    Infra Client.

`cookbook_path`

:   The sub-directory for Chef Infra Client cookbooks. This value can be
    a string or an array of file system locations, processed in the
    specified order. The last cookbook is considered to override local
    modifications.

`cookbook_sync_threads`

:   The number of helper threads available for parallel cookbook
    synchronization. Increasing this value **may** increase the
    frequency of gateway errors from the Chef Infra Server (503 and 504
    errors). Decreasing this number reduces the frequency of gateway
    errors, if present. Default value: `10`.

`data_bag_decrypt_minimum_version`

:   The minimum required version of data bag encryption. Possible
    values: `1`, `2`, and `3`. It is recommended to use the default
    value of `3` for additional encrypted data bag security.

`data_bag_path`

:   The location from which a data bag is loaded. Default value:
    `/var/chef/data_bags`.

`data_collector.server_url`

:   The fully qualified URL to the data collector server API.

`data_collector.token`

:   The shared data collector security token. When configured, the token
    will be passed as an HTTP header named `x-data-collector-token`
    which the server can choose to accept or reject.

`data_collector.mode`

:   The Chef Infra Client mode in which the Data Collector will be
    enabled. Possible values: `:solo`, `:client`, or `:both`. The
    `:solo` value is used for Chef Infra Client operating in Chef Solo
    Mode or Chef Solo Legacy Mode. Default value: `both`.

`data_collector.raise_on_failure`

:   When enabled, Chef Infra Client raises an error if it cannot
    successfully POST to the data collector server. Default value:
    `false`.

`default_attribute_blacklist`

:   A hash that blacklists `default` attributes, preventing blacklisted
    attributes from being saved.

`default_attribute_whitelist`

:   A hash that whitelists `default` attributes, preventing
    non-whitelisted attributes from being saved.

`diff_disabled`

:   Cause Chef Infra Client to create a diff when changes are made to a
    file. Default value: `false`.

`diff_filesize_threshold`

:   The maximum size (in bytes) of a file for which Chef Infra Client
    can create a diff. Default value: `10000000`.

`diff_output_threshold`

:   The maximum size (in bytes) of a diff file Chef Infra Client can
    create. Default value: `1000000`.

`disable_event_logger`

:   Enable or disable sending events to the Microsoft Windows
    "Application" event log. When `false`, events are sent to the
    Microsoft Windows "Application" event log at the start and end of a
    Chef Infra Client run, and also if a Chef Infra Client run fails.
    Set to `true` to disable event logging. Default value: `false`.

`enable_reporting`

:   Cause Chef Infra Client to send run data to the Automate server.

`enable_reporting_url_fatals`

:   Cause a Chef Infra Client run to fail when run data cannot be sent
    to the Automate server (for any reason).

`enable_selinux_file_permission_fixup`

:   SELinux environments only. Cause Chef Infra Client to attempt to
    apply the correct file permissions to an updated file via the
    `restorecon` command. Set this value to `false` to prevent Chef
    Infra Client from attempting this action.

`encrypted_data_bag_secret`

:   The subdirectory in which encrypted data bag secrets are located.

`enforce_path_sanity`

:   Turn on path sanity in resources that shellout so that expected
    paths like /sbin or /bin are added to the PATH. Disabled by default.

`environment`

:   The name of the Chef Infra environment.

`environment_path`

:   The path to the environment file. Default value:
    `/var/chef/environments`.

`exit_status`

:   When set to `:enabled`, Chef Infra Client will use [standardized
    exit
    codes](https://github.com/chef/chef-rfc/blob/master/rfc062-exit-status.md#exit-codes-in-use)
    for the Chef Infra Client run status, and any non-standard exit
    codes will be converted to `1` or `GENERIC_FAILURE`. This setting
    can also be set to `:disabled` to use the pre-Chef Infra Client 13
    exit code behavior. Default value: `nil`.

`file_atomic_update`

:   Apply atomic file updates to all resources. Set to `true` for global
    atomic file updates. Set to `false` for global non-atomic file
    updates. (Use the `atomic_update` setting on a per-resource basis to
    override this setting.) Default value: `true`.

    {{< warning spaces=4 >}}

    Changing this setting to `false` may cause file corruption, data
    loss, or instability. Use the `atomic_update` property on the
    **cookbook_file**, **file**, **remote_file**, and **template**
    resources to tune this behavior at the recipe level.

    {{< /warning >}}

`file_backup_path`

:   The location in which backup files are stored. If this value is
    empty, backup files are stored in the directory of the target file.
    Default value: `/var/chef/backup`.

`file_cache_path`

:   The location in which cookbooks (and other transient data) files are
    stored when they are synchronized. This value can also be used in
    recipes to download files with the **remote_file** resource.

`file_staging_uses_destdir`

:   How file staging (via temporary files) is done. When `true`,
    temporary files are created in the directory in which files will
    reside. When `false`, temporary files are created under
    `ENV['TMP']`. Default value: `true`.

`fips`

:   Allows OpenSSL to enforce FIPS-validated security during a Chef
    Infra Client run. Set to `true` to enable FIPS-validated security.

`force_formatter`

:   Using <span class="title-ref">force_formatter</span> causes chef to
    default to formatter output when STDOUT is not a tty

`force_logger`

:   Using <span class="title-ref">force_logger</span> causes chef to
    default to logger output when STDOUT is a tty

`ftp_proxy`

:   The proxy server for FTP connections.

`ftp_proxy_pass`

:   The password for the proxy server when the proxy server is using an
    FTP connection. Default value: `nil`.

`ftp_proxy_user`

:   The user name for the proxy server when the proxy server is using an
    FTP connection. Default value: `nil`.

`group`

:   The group that owns a process. This is required when starting any
    executable as a daemon. Default value: `nil`.

`gem_installer_bundler_options`

:   Additional options to pass to bundler when installing metadata for
    cookbook. Default value: `nil`. For example

    ``` ruby
    gem_installer_bundler_options = [
      "--local", "--clean"
    ]
    ```

    or

    ``` ruby
    gem_installer_bundler_options = "--local"
    ```

`http_proxy`

:   The proxy server for HTTP connections. Default value: `nil`.

`http_proxy_pass`

:   The password for the proxy server when the proxy server is using an
    HTTP connection. Default value: `nil`.

`http_proxy_user`

:   The user name for the proxy server when the proxy server is using an
    HTTP connection. Default value: `nil`.

`http_retry_count`

:   The number of retry attempts. Default value: `5`.

`http_retry_delay`

:   The delay (in seconds) between retry attempts. Default value: `5`.

`https_proxy`

:   The proxy server for HTTPS connections. Default value: `nil`.

`https_proxy_pass`

:   The password for the proxy server when the proxy server is using an
    HTTPS connection. Default value: `nil`.

`https_proxy_user`

:   The user name for the proxy server when the proxy server is using an
    HTTPS connection. Default value: `nil`.

`interval`

:   The frequency (in seconds) at which Chef Infra Client runs. Default
    value: `1800`.

`json_attribs`

:   The path to a file that contains JSON data.

`listen`

:   Run chef-zero in socketless mode. Set to `false` to disable port
    binding and HTTP requests on localhost.

`local_key_generation`

:   Whether the Chef Infra Server or Chef Infra Client generates the
    private/public key pair. When `true`, Chef Infra Client generates
    the key pair, and then sends the public key to the Chef Infra
    Server. Default value: `true`.

`local_mode`

:   Run Chef Infra Client in local mode. This allows all commands that
    work against the Chef Infra Server to also work against the local
    chef-repo.

`lockfile`

:   The location of the Chef Infra Client lock file. This value is
    typically platform dependent, so it should be a location defined by
    `file_cache_path`. The default location of a lock file should not be
    on an NFS mount. Default value: a location defined by
    `file_cache_path`.

`log_level`

:   The level of logging to be stored in a log file. Possible levels:
    `:auto` (default), `:trace`(starting in 14.0.202), `:debug`, `:info`, `:warn`, `:error`,
    or `:fatal`. Default value: `:warn` (when a terminal is available)
    or `:info` (when a terminal is not available).

`log_location`

:   The location of the log file. Possible values:
    `/path/to/log_location`, `STDOUT`, `STDERR`, `:win_evt` (Windows
    Event Logger), or `:syslog` (writes to the syslog daemon facility
    with the originator set as `chef-client`). The application log will
    specify the source as `Chef`. Default value: `STDOUT`.

`minimal_ohai`

:   Run the Ohai plugins for name detection and resource/provider
    selection and no other Ohai plugins. Set to `true` during
    integration testing to speed up test cycles.

`named_run_list`

:   The run-list associated with a policy file.

`no_lazy_load`

:   Download all cookbook files and templates at the beginning of a Chef
    Infra Client run. Default value: `true`.

`no_proxy`

:   A comma-separated list of URLs that do not need a proxy. Default
    value: `nil`.

`node_name`

:   The name of the node. Determines which configuration should be
    applied and sets the `client_name`, which is the name used when
    authenticating to a Chef Infra Server. The default value is the Chef
    Infra Client FQDN, as detected by Ohai. In general, Chef recommends
    that you leave this setting blank and let Ohai assign the FQDN of
    the node as the `node_name` during each Chef Infra Client run.

`node_path`

:   The location in which nodes are stored during a Chef Infra Client
    run in local mode. Default value: `/var/chef/node`.

`normal_attribute_blacklist`

:   A hash that blacklists `normal` attributes, preventing blacklisted
    attributes from being saved.

`override_attribute_blacklist`

:   A hash that blacklists `override` attributes, preventing blacklisted
    attributes from being saved.

`normal_attribute_whitelist`

:   A hash that whitelists `normal` attributes, preventing
    non-whitelisted attributes from being saved.

`override_attribute_whitelist`

:   A hash that whitelists `override` attributes, preventing
    non-whitelisted attributes from being saved.

`pid_file`

:   The location in which a process identification number (pid) is
    saved. An executable, when started as a daemon, writes the pid to
    the specified file. Default value: `/tmp/name-of-executable.pid`.

`policy_group`

:   The name of a policy group that exists on the Chef Infra Server.
    `policy_name` must also be specified.

`policy_group_path`

:   The location of policy_groups on disk.

`policy_name`

:   The name of a policy, as identified by the `name` setting in a
    Policyfile.rb file. `policy_group` must also be specified.

`policy_path`

:   The location of policies on disk.

`recipe_url`

:   A URL to download recipes from when running in local mode.

`rest_timeout`

:   The time (in seconds) after which an HTTP REST request is to time
    out. Default value: `300`.

`role_path`

:   The location in which role files are located. Default value:
    `/var/chef/roles`.

`rubygems_url`

:   The location to source rubygems. It can be set to a string or array
    of strings for URIs to set as rubygems sources. This allows
    individuals to setup an internal mirror of rubygems for "airgapped"
    environments. Default value: `https://www.rubygems.org`. If a
    `source` is specified in either `gem_package` of `chef_gem`
    resources it will be added to the values provided here.

`run_lock_timeout`

:   The amount of time (in seconds) to wait for a Chef Infra Client lock
    file to be deleted. A Chef Infra Client run will not start when a
    lock file is present. If a lock file is not deleted before this time
    expires, the pending Chef Infra Client run will exit. Default value:
    not set (indefinite). Set to `0` to cause a second Chef Infra Client
    to exit immediately.

`script_path`

:   An array of paths to search for knife exec scripts if they aren't in
    the current directory

`skip_gem_metadata_installation`

:   when `skip_gem_metadata_installation` is set to true, cookbook gem
    installation will be skipped. Default value: `false`

`splay`

:   A random number between zero and `splay` that is added to
    `interval`. Use splay to help balance the load on the Chef Infra
    Server by ensuring that many Chef Infra Client runs are not
    occurring at the same interval. Default value: `nil`.

`stream_execute_output`

:   Always stream the output of `execute` resources even if the
    `live_stream` property isn't set to true. Default value: `false`

`show_download_progress`

:   Using show_download_progress will display the overall progress of
    a `remote_file` download. Default value: `false`

`download_progress_interval`

:   When `show_download_progress` is set to true this is the interval in
    seconds to write out download progress. Default value: `10`

`ssl_ca_file`

:   The file in which the OpenSSL key is saved. Chef Infra Client
    generates this setting automatically and most users do not need to
    modify it.

`ssl_ca_path`

:   The path to where the OpenSSL key is located. Chef Infra Client
    generates this setting automatically and most users do not need to
    modify it.

`ssl_client_cert`

:   The OpenSSL X.509 certificate used for mutual certificate
    validation. This setting is only necessary when mutual certificate
    validation is configured on the Chef Infra Server. Default value:
    `nil`.

`ssl_client_key`

:   The OpenSSL X.509 key used for mutual certificate validation. This
    setting is only necessary when mutual certificate validation is
    configured on the Chef Infra Server. Default value: `nil`.

`ssl_verify_mode`

:   Set the verify mode for HTTPS requests.

    -   Use `:verify_none` for no validation of SSL certificates.
    -   Use `:verify_peer` for validation of all SSL certificates,
        including the Chef Infra Server connections, S3 connections, and
        any HTTPS **remote_file** resource URLs used in Chef Infra
        Client runs. This is the recommended setting.

    Depending on how OpenSSL is configured, the `ssl_ca_path` may need
    to be specified. Default value: `:verify_peer`.

`trusted_certs_dir`

:   A directory that contains additional SSL certificates to trust. Any
    certificates in this directory will be added to whatever CA bundle
    ruby is using. Use this to add self-signed certs for your Chef Infra
    Server or local HTTP file servers. Default value: `trusted_certs`
    directory in your chef configuration directory.

`umask`

:   The file mode creation mask, or umask. Default value: `0022`.

`use_policyfile`

:   Chef Infra Client automatically checks the configuration, node JSON,
    and the stored node on the Chef Infra Server to determine if
    Policyfile files are in use, and then automatically updates this
    flag. Default value: `false`.

`user`

:   The user that owns a process. This is required when starting any
    executable as a daemon. Default value: `nil`.

`validation_client_name`

:   The name of the chef-validator key that Chef Infra Client uses to
    access the Chef Infra Server during the initial Chef Infra Client
    run. This is only used by the legacy validator based bootstrapping.

`validation_key`

:   The location of the file that contains the key used when a Chef
    Infra Client is registered with a Chef Infra Server. A validation
    key is signed using the `validation_client_name` for authentication.
    Default value: `/etc/chef/validation.pem`. This is only used by the
    legacy validator based bootstrapping.

`verbose_logging`

:   Set the log level. Options: `true`, `nil`, and `false`. When this is
    set to `false`, notifications about individual resources being
    processed are suppressed (and are output at the `:info` logging
    level). Setting this to `false` can be useful when a Chef Infra
    Client is run as a daemon. Default value: `nil`.

`verify_api_cert`

:   Verify the SSL certificate on the Chef Infra Server. When `true`,
    Chef Infra Client always verifies the SSL certificate. When `false`,
    Chef Infra Client uses the value of `ssl_verify_mode` to determine
    if the SSL certificate requires verification. Default value:
    `false`.

`whitelist`

:   A hash that contains the whitelist of allowed commands used by Chef
    Push Jobs. For example:

    ``` ruby
    whitelist {
      'job-name' => 'command',
      'job-name' => 'command',
      'chef-client' => 'chef-client'
    }
    ```

    A job entry may also be `'job-name' => {:lock => true}`, which will
    check the `lockfile` setting in the client.rb file before starting
    the job.

### Automatic Proxy Config

{{% proxy_env %}}

## .d Directories

{{% config_rb_client_dot_d_directories %}}

## Ohai Settings

{{% config_rb_ohai %}}

{{< readFile_shortcode file="config_rb_ohai_settings.md" >}}

## Example

A sample client.rb file that contains the most simple way to connect to
<https://manage.chef.io>:

``` ruby
log_level        :info
log_location     STDOUT
chef_server_url  'https://api.chef.io/organizations/<orgname>'
validation_client_name '<orgname>-validator'
validation_key '/etc/chef/validator.pem'
client_key '/etc/chef/client.pem'
```
