+++
title = "delivery.rb Optional Settings"
draft = false
robots = "noindex"


aliases = ["/config_rb_delivery_optional_settings.html", "/release/automate/config_rb_delivery_optional_settings.html"]


[menu]
  [menu.legacy]
    title = "delivery.rb Optional Settings"
    identifier = "legacy/workflow/reference/config_rb_delivery_optional_settings.md delivery.rb Optional Settings"
    parent = "legacy/workflow/reference"
    weight = 30
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/config_rb_delivery_optional_settings.md)

{{< warning >}}

The configuration settings in the `delivery.rb` file should not be
modified before discussing those changes with Chef. Some of these
settings should be considered for tuning (see [Workflow Server
Tuning](/delivery_server_tuning/)), but many of them should be left
as default values.

{{< /warning >}}

## Settings

The following sections describe the various settings that are available
in the `delivery.rb` file.

### General

This configuration file has the following general settings:

`bootstrap['enable']`

:   Default value: `true`.

`chef_base_path`

:   Default value: `"/opt/opscode"`.

`cookbook_path`

:   Default value:

    ``` ruby
    File.join(
      node['delivery']['install_path'],
      "embedded", "cookbooks"
    )
    ```

`dir`

:   The working directory. The default value is the recommended value.
    Default value: `"/opt/delivery"`.

`fips['enable']`

:   Set to <span class="title-ref">true</span> to run the server in FIPS
    compliance mode. Set to <span class="title-ref">false</span> to
    force the server to run without FIPS compliance mode. Default value
    is whatever the kernel is configured to. See [Fips kernel
    settings](/fips/#FIPS-kernel-settings).

`install_path`

:   Default value: `"/opt/delivery"`.

`name`

:   Default value: `'delivery'`.

`default['delivery']['user']['comment']`

:   This is the "GECOS" field for a Unix user (e.g., a human-readable
    name). Default value: `"CHEF Delivery"`.

`ip_version`

:   This specifies the IP protocol version to be used when configuring
    the embedded services. Can be either `"ipv4"` or `"ipv6"`. Default
    value: `"ipv4"`.

### admin

This configuration file has the following settings for `admin`:

`admin['account_name']`

:   Default value: `'admin'`.

`admin['email']`

:   Default value: `'admin@example.com'`.

`admin['full_name']`

:   Default value: `'Chef Delivery Administrator'`.

`admin['password']`

:   Default value: `'snakes'`.

### backup

This configuration file has the following settings for `backup`:

`backup['access_key_id']`

:   Amazon Web Services (AWS) Access Key ID for uploading Chef Automate
    backup archives to S3. Only use this if you cannot configure the
    machine with an instance profile, shared credentials, or environment
    variables. Default value: `nil`.

`backup['create_bucket']`

:   Create an S3 bucket for backup archives if it does not exist.
    Default value: `true`.

`backup['bucket']`

:   S3 bucket for storing Chef Automate backup archives. Default value:
    `nil`.

`backup['base_path']`

:   Optional S3 base path prefix. This is used if you wish to store Chef
    Automate backup archives in a nested path in the S3 bucket. Default
    value: `nil`.

`backup['census']['enabled']`

:   Back up Chef Automate Census data. Default value: `true`.

`backup['chef_server_config']`

:   Back up the Chef Infra Server configuration directory. Usefull for
    instances when Chef Automate and Chef Infra Server are installed on
    a single node. Default value: `false`.

`backup['compliance_profiles']['enabled']`

:   Back up the Chef Automate compliance profiles. Default value:
    `true`.

`backup['config']['enabled']`

:   Back up the Chef Automate configuration directory. Default value:
    `true`.

`backup['cron']['enabled']`

:   Create a cron job that manages backups. Default value: `false`.

`backup['cron']['max_archives']`

:   Maximum number of backup archives to be kept. Default value: `7`.

`backup['cron']['max_snapshots']`

:   Maximum number of backup snapshots to be kept. Default value: `7`.

`backup['cron']['notation']`

:   Time notation for backup cron job. Default value: `'0 0 * * *'`.

`backup['db']['enabled']`

:   Back up the Chef Automate PostgreSQL database. Default value:
    `true`.

`backup['delete']['pattern']`

:   The pattern to match when deleting backup archives and Elasticsearch
    snapshots. Default value: `nil`.

`backup['delete']['max_archives']`

:   The maximum number of backup archives to keep. Default value: `nil`.

`backup['delete']['max_snapshots']`

:   The maximum number of Elasticsearch snapshots to keep. Default
    value: `nil`.

`backup['digest']['enabled']`

:   Output the SHA digest of the backup archive to STDOUT. Default
    value: `true`.

`backup['digest']['length']`

:   The SHA digest length to use. Valid options are `256`, `384`, and
    `512`. Default value: `256`.

`backup['elasticsearch']['access_key_id']`

:   Amazon Web Services (AWS) Access Key ID for uploading Chef Automate
    Elasticsearch snapshots to S3. Only use this if you cannot configure
    the machine with an instance profile, shared credentials, or
    environment variables. Default value: `nil`.

`backup['elasticsearch']['bucket']`

:   S3 bucket for storing Chef Automate Elasticsearch snapshots. Default
    value: `nil`.

`backup['elasticsearch']['base_path']`

:   Optional S3 base path prefix. This is used if you wish to store Chef
    Automate Elasticsearch snapshots in a nested path in the S3 bucket.
    Default value: `nil`.

`backup['elasticsearch']['enabled']`

:   Create Chef Automate Elasticsearch snapshots. Default value: `true`.

`backup['elasticsearch']['location']`

:   Shared filesystem repository location for Elasticsearch snapshots.
    Default value: `/var/opt/delivery/elasticsearch_backups`.

`backup['elasticsearch']['max_restore_bytes_per_sec']`

:   Maximum snapshot speed when restoring shared filesystem
    Elasticsearch snaphots. Default value: `40mb`.

`backup['elasticsearch']['max_snapshot_bytes_per_sec']`

:   Maximum snapshot speed when creating shared filesystem Elasticsearch
    snaphots. Default value: `40mb`.

`backup['elasticsearch']['poll_interval']`

:   How many seconds to wait between polling requests while waiting for
    Elasticsearch operations. Default value `5`.

`backup['elasticsearch']['region']`

:   Amazon Web Services (AWS) region to use for Chef Automate S3
    Elasticsearch snapshots. Default value `nil`.

`backup['elasticsearch']['request_timeout']`

:   Maximum seconds an Elasticsearch request can wait before timing out.
    Default value `'300'`.

`backup['elasticsearch']['retry_limit']`

:   Maximum number of times to retry failed Elasticsearch requests.
    Default value `3`.

`backup['elasticsearch']['secret_access_key']`

:   Amazon Web Services (AWS) Secret Key for uploading Chef Automate
    Elasticsearch snapshots in S3. Only use this if you cannot configure
    the machine with an instance profile, shared credentials, or
    environment variables. Default value: `nil`.

`backup['elasticsearch']['server_side_encryption']`

:   Enable Amazon Web Services (AWS) SSE-S3 AES256 Server Side
    Encryption for Elasticsearch snapshots in S3. Default value: `true`.

`backup['elasticsearch']['type']`

:   Which backup type to use for Chef Automate Elasticsearch snapshots.
    Shared filesystem and S3 backups are currently supported by using
    the `fs` and `s3` types. Default value: `fs`.

`backup['elasticsearch']['wait_for_lock']`

:   Enable or disable waiting for the Chef Automate exclusive
    Elasticsearch lock when performing major operations. Default value:
    `true`.

`backup['force']`

:   Agree to any prompts or warnings during the Chef Automate backup
    procedure. Default value: `false`.

`backup['git']['enabled']`

:   Back up the Chef Automate git repositories. Default value: `true`.

`backup['license']['enabled']`

:   Back up the Chef Automate license file. Default value: `true`.

`backup['list']['types']`

:   Types to list when running the `automate-ctl list-backups` command.
    Options are `all`, `automate`, and `elasticsearch`. Default value:
    `all`.

`backup['list']['format']`

:   Format to return when running the `automate-ctl list-backups`
    command. Options are `text` and `json`. Default value: `text`.

`backup['location']`

:   Location on disk to store Chef Automate backup archives. Default
    value: `/var/opt/delivery/backups`.

`backup['name']`

:   Name to use for Chef Automate backup archives and snapshots. When
    omitted a default will used automatically. Default value: `nil`.

`backup['notifications']['enable']`

:   Back up the Chef Automate notification rules. Default value: `true`.

`backup['quiet']`

:   Silence non-error information during the Chef Automate backup
    procedure. Default value: `false`.

`backup['rabbit']['enabled']`

:   Back up the Chef Automate RabbitMQ queues. It is disabled by default
    because it's rare to have a lengthy RabbitMQ queue and the backup
    procedure requires temporarily shutting down Chef Automate services
    when backing up the queues. Default value: `false`.

`backup['region']`

:   Amazon Web Services (AWS) region to use when storing Chef Automate
    backup archives in S3. Default value `nil`.

`backup['secret_access_key']`

:   Amazon Web Services (AWS) Secret Key for uploading Chef Automate
    backup archives to S3. Only use this if you cannot configure the
    machine with an instance profile, shared credentials, or environment
    variables. Default value: `nil`.

`backup['server_side_encryption']`

:   Enable Amazon Web Services (AWS) SSE-S3 AES256 Server Side
    Encryption for backup archives in S3. To use SSE-KMS set the value
    to `aws:kms`. Default value: `AES256`.

{{< note >}}

While the backup utility currently supports encrypting backups with with
SSE-S3, SSE-KMS, and SSE-C, only SSE-S3 is currently supported for
restoration.

{{< /note >}}

`backup['staging_dir']`

:   A local directory to use for temporary files when creating a backup
    archive. The directory will be cleared during backup and used for
    storing the backup archive, database dump, and configuration file.
    When not configured it will use a default Ruby temporary directory
    which is usually nested in `/tmp` on linux but will also honor the
    value of the `TMPDIR` environment variable. Default value: `nil`.

`backup['sse_customer_algorithm']`

:   The SSE-C algorithm to use for customer Server Side Encryption.
    Default value: `nil`.

`backup['sse_customer_key']`

:   The SSE-C key to use for customer Server Side Encryption. Default
    value `nil`.

`backup['sse_customer_key_md5']`

:   The MD5 hash of the customer key for customer Server Side
    Encryption. Default value: `nil`.

`backup['ssekms_key_id']`

:   The SSE-KMS key id to use for customer Server Side Encryption.
    Default value: `nil`

`backup['type']`

:   Which backup type to use for Chef Automate backup archives. Local
    filesystem and S3 backups are currently supported by using the `fs`
    and `s3` types. Default value: `fs`.

`backup['retry_limit']`

:   The maximum of times to retry when uploading backup archives to a
    remote repository like Amazon Web Services (AWS) S3. Default value:
    `5`.

`backup['wait']`

:   Wait for non-blocking steps during the backup procedure. Useful if
    you'd like the backup to to return early without waiting for the
    Elasticsearch snapshot to complete. Default setting: `true`.

### deliv_notify

This configuration file has the following settings for `deliv_notify`:

`deliv_notify['config']`

:   Default value: `[]`.

### delivery

This configuration file has the following settings for `delivery`:

`delivery['api_port']`

:   Default value: `9611`.

`delivery['audit_max_events']`

:   Maximum number of audit events to keep in memory. Default value:
    `100`.

`delivery['ca_cert_chain_depth']`

:   Default value: `2`.

`delivery['chef_config']`

:   Default value:

    ``` ruby
    File.join(node['delivery']['delivery']['etc_dir'], "erlang.cfg")
    ```

`delivery['chef_private_key']`

:   Default value: `"/etc/delivery/delivery-cd.pem"`.

`delivery['chef_server']`

:   Default value: `'https://localhost/organizations/cd'`.

`delivery['chef_server_webui']`

:   This should be programmatically derived from the chef_server
    attribute above. Default value: `'https://localhost'`.

`delivery['chef_username']`

:   Default value: `"delivery-cd"`.

`delivery['db_name']`

:   Default value: `"delivery"`.

`delivery['db_pool_init_count']`

:   The number of open connections to PostgreSQL that are maintained by
    the service. Default value: `20`.

`delivery['db_pool_max_count']`

:   The maximum number of open connections to PostgreSQL. Default value:
    `100`.

`delivery['default_search']`

:   The default search to use for build nodes if it is not specified in
    `delivery.rb`. Default value:

    ``` ruby
    "(recipes:delivery_builder OR " +
      "recipes:delivery_builder\\\\:\\\\:default OR " +
      "recipes:delivery_build OR " +
      "recipes:delivery_build\\\\:\\\\:default)"
    ```

`delivery['dir']`

:   The working directory. The default value is the recommended value.
    Default value: `"/var/opt/delivery/delivery"`.

`delivery['enable']`

:   Enable a service. Default value: `true`.

`delivery['etc_dir']`

:   Default value: `"/var/opt/delivery/delivery/etc"`.

`delivery['git_repo_template']`

:   Where to look for the delivery git repo template must remain
    consistent with where omnibus-delivery's 'delivery' software
    definition puts it. Default value:

    ``` ruby
    ::File.join(node['delivery']['user']['home'], 'etc', 'deliv_git_repo_template')
    ```

`delivery['git_repos']`

:   Default value:

    ``` ruby
    ::File.join(node['delivery']['delivery']['dir'], 'git_repos')
    ```

`delivery['git_working_tree_dir']`

:   Define default directory location for the git working tree. Default
    value:

    ``` ruby
    ::File.join(node['delivery']['delivery']['dir'], 'git_workspace')
    ```

`delivery['is_dev_box']`

:   Default value: `false`.

`delivery['ldap_attr_full_name']`

:   The attribute that contains a full or display name for a user.
    Default value: `'fullName'`.

`delivery['ldap_attr_login']`

:   The attribute that maps to a user's unique logon name. This is the
    attribute used for searching and will be used to map a user name
    into Chef Automate. Default value: `'sAMAccountName'`.

`delivery['ldap_attr_mail']`

:   The attribute that maps to user email address. Default value:
    `'mail'`.

`delivery['ldap_base_dn']`

:   The root LDAP node under which all other nodes exist in the
    directory structure. Default value:

    ``` ruby
    "OU=Employees,OU=Domain users,DC=examplecorp,DC=com"
    ```

`delivery['ldap_bind_dn']`

:   The distinguished name used to bind to the LDAP server. Default
    value: `"ldapbind"`.

`delivery['ldap_bind_dn_password']`

:   The password for the binding user. Default value: `"secret123"`.

`delivery['ldap_encryption']`

:   `"start_tls"`, `"simple_tls"`, or `"no_tls"`. Default value:
    `"no_tls"`.

`delivery['ldap_hosts']`

:   The name (or IP address) of the LDAP server. Default value: `[]`.

`delivery['ldap_port']`

:   An integer that specifies the port on which the LDAP server listens.
    Default value: `3269`.

`delivery['ldap_timeout']`

:   The amount of time (in seconds) to wait before timing out. Default
    value: `5000`.

`delivery['listen']`

:   The virtual IP address. Default value: `'127.0.0.1'`.

`delivery['log_directory']`

:   The directory in which log data is stored. The default value is the
    recommended value. Default value: `"/var/log/delivery/delivery"`.

`delivery['log_rotation']['file_maxbytes']`

:   The log rotation policy for this service. Log files are rotated when
    they exceed `file_maxbytes`. The maximum number of log files in the
    rotation is defined by `num_to_keep`. Default value:
    `1024 * 1000 * 10`.

`delivery['log_rotation']['num_to_keep']`

:   The log rotation policy for this service. Log files are rotated when
    they exceed `file_maxbytes`. The maximum number of log files in the
    rotation is defined by `num_to_keep`. Default value: `10`.

`delivery['phase_job_confirmation_timeout']`

:   Timeout for waiting for phase job to confirm completion. Default
    value: `'5m'`.

`delivery['port']`

:   The port on which the service is to listen. Default value: `9611`.

`delivery['primary']`

:   Specifies if the Chef Automate server is the primary server. Default
    value: `true`.

`delivery['primary_ip']`

:   The IP address for the primary Chef Automate server. Default value:
    `nil`.

`delivery['push_jobs_max_retries']`

:   Maximum number of retries a push job can incur without an
    intervening nack. Default value: `3`.

`delivery['push_jobs_overall_timeout']`

:   Timeout for finding worker and then waiting for push job to
    complete. Default value: `'2h'`.

`delivery['push_jobs_run_timeout']`

:   Timeout for waiting for push job to complete once worker has been
    found. Default value: `'75m'`.

`delivery['read_ttl']`

:   The amount of time after which the `READ` token expires. This value
    may be specified a string with units (e.g., `"4d"`, `"3h"`, `"2m"`,
    `"1s"`), or as bare integers (interpreted as seconds). Valid units
    are: `d` (days), `h` (hours), `m` (minutes), or `s` (seconds).
    Default value: `'7d'`.

    {{< note spaces=4 >}}

    While the `delivery['read_ttl']` and `delivery['write_ttl']` values
    may be tuned separately, it is recommended that both values be
    identical.

    {{< /note >}}

`delivery['sql_password']`

:   Default value: `'pokemon'`.

`delivery['sql_repl_password']`

:   Default value: `'pokemon_repl'`.

`delivery['sql_repl_user']`

:   Default value: `'delivery_repl'`.

`delivery['sql_ro_password']`

:   Default value: `'pokemon_ro'`.

`delivery['sql_ro_user']`

:   Default value: `'delivery_ro'`.

`delivery['sql_user']`

:   Default value: `'delivery'`.

`delivery['ssl_certificates']`

:   A hash of SSL certificate files to use for FQDNs. Will use
    `remote_file` to download the key and crt specified. If you wanted
    to use a pre-generated SSL certificate for the main fqdn
    (`delivery_fqdn`) you could specify that here. For example:

    ``` ruby
    delivery['ssl_certificates'] = {
      'delivery.example.com' => {
        'key' => 'https://my_bucket/ssl_certificates/delivery.example.com.key',
        'crt' => 'https://my_bucket/ssl_certificates/delivery.example.com.crt'
      }
    }
    ```

`delivery['no_ssl_verification']`

:   An array of hostnames that are whitelisted from requiring SSL
    verification. For example:

    ``` ruby
    delivery['no_ssl_verification'] = ['self-signed.badssl.com', 'untrusted-root.badssl.com']
    ```

`delivery['standby_ip']`

:   The IP address for the cold standby Chef Automate server. Default
    value: `nil`.

`delivery['use_ssl_termination']`

:   Default value: `false`.

`delivery['write_ttl']`

:   The amount of time after which the `WRITE` token expires. This value
    may be specified a string with units (e.g., `"4d"`, `"3h"`, `"2m"`,
    `"1s"`), or as bare integers (interpreted as seconds). Valid units
    are: `d` (days), `h` (hours), `m` (minutes), or `s` (seconds).
    Default value: `'7d'`.

    {{< note spaces=4 >}}

    While the `delivery['read_ttl']` and `delivery['write_ttl']` values
    may be tuned separately, it is recommended that both values be
    identical.

    {{< /note >}}

`delivery['vip']`

:   The virtual IP address. Default value: `'127.0.0.1'`.

### elasticsearch

This configuration file has the following settings for `elasticsearch`:

`elasticsearch['urls']`

:   The fully qualified domain name(s) of your Elasticsearch cluster. If
    not specified a local elasticsearch cluster will be utilized.
    Default value: `"http://127.0.0.1:9200"`.

`elasticsearch['role_arn']`

:   The Amazon Resource Names(ARN) of IAM policies role for Amazon
    Elasticsearch Service. Default value: `nil`.

    {{< note spaces=4 >}}

    If `elasticsearch['urls']` is specified with Amazon elasticsearch
    url then `elasticsearch['role_arn']` value will be required.

    {{< /note >}}

`elasticsearch['config_directory']`

:   The working directory. The default value is the recommended value.
    Default value: `"/var/opt/delivery/elasticsearch/conf"`.

`elasticsearch['home']`

:   Default value:
    `"#{node['delivery']['user']['home']}/elasticsearch"`.

`elasticsearch['log_directory']`

:   The directory in which log data is stored. The default value is the
    recommended value. Default value:
    `"/var/log/delivery/elasticsearch"`.

`elasticsearch['log_rotation']['file_maxbytes']`

:   The log rotation policy for this service. Log files are rotated when
    they exceed `file_maxbytes`. The maximum number of log files in the
    rotation is defined by `num_to_keep`. Default value:
    `100 * 1024 * 1024` (100MB).

`elasticsearch['log_rotation']['num_to_keep']`

:   The log rotation policy for this service. Log files are rotated when
    they exceed `file_maxbytes`. The maximum number of log files in the
    rotation is defined by `num_to_keep`. Default value: `10`.

`elasticsearch['memory']`

:   The Elasticsearch JVM's heap size. Default value:

    ``` ruby
    "#{(node.memory.total.to_i * 0.4 ).floor / 1024}m"
    ```

**The following Elasticsearch options require Chef Automate 0.8.46 or
later:**

`elasticsearch['max_open_file']`

:   The maximum number of files Elasticsearch may open simultaneously.
    The default value is `65536`. Setting this to a lower value may lead
    to data loss and is highly discouraged.

`elasticsearch['max_map_count']`

:   The maximum number of memory map areas the Elasticsearch process may
    have. The default value is `262144`. Setting this to a lower value
    may cause Elasticsearch to fail with out-of-memory errors.

`elasticsearch['config']['bootstrap']['memory_lock']`

:   When set to `true`, locks the memory allocated by Elasticsearch so
    that it may not be swapped to disk by the OS. Enabling this will
    cause Elasticsearch to fail on start if there is not enough memory
    available for the configured heap size. On systems where swap is
    disabled, this setting has no effect. Default value: `false`. This
    flag was named `elasticsearch['config']['bootstrap']['mlockall']` in
    Chef Automate 1.5.x and below.

`elasticsearch['config']['indices']['breaker']['fielddata']['limit']`

:   The maximum amount of heap memory that may be consumed by fielddata.
    Any query that would result in this limit being exceeded will be
    aborted. Default value: `'60%'`.

`elasticsearch['config']['indices']['breaker']['request']['limit']`

:   The maximum amount of heap memory, excluding fielddata, that may be
    consumed by a request. Any query that would result in this limit
    being exceeded will be aborted. Default value: `'40%'`.

`elasticsearch['config']['indices']['breaker']['total']['limit']`

:   The maximum amount of combined heap memory that may be consumed by a
    single request. Any query that would result in this limit being
    exceeded will be aborted. Default value: `'70%'`.

`elasticsearch['config']['indices']['store']['throttle']['max_bytes_per_sec']`

:   The maximum throughput allowed for creating and optimizing
    Elasticsearch search indexes. When this limit is reached,
    Elasticsearch logs a message containing `now throttling indexing` at
    the `INFO` log level. If you see evidence of index throttling and
    have sufficient disk I/O capacity, you can increase this setting.
    Default value: `'100mb'`.

**The following Elasticsearch options require Chef Automate 1.6.87 or
later:**

`elasticsearch['new_memory_size']`

:   The 'new generation' heap size of the JVM running Elasticsearch.
    Default value:

    ``` ruby
    "#{elasticsearch['memory'].to_i / 16}m"
    ```

`elasticsearch['jvm_opts']`

:   A list of other JVM-related options to pass along. Note that this
    should not contain the heap memory size and the new generation
    memory size from above. Default value: `[]`. Example:

    ``` ruby
    elasticsearch['jvm_opts'] = [
      "-xoption1",
      "-xoption2",
      ...
      "optionN"
    ]
    ```

`elasticsearch['enable_gc_log']`

:   Enable garbage-collection logging on the JVM. Only set this to
    `true` if you are debugging a garbage collection-related performance
    issue. Default value: `false`.

**The following Elasticsearch options require Chef Automate 1.6.179 or
later:**

`elasticsearch['auth_user']`

:   The username that Chef Automate will use if you have Elasticsearch
    X-Pack Basic Authentication enabled on your Elasticsearch cluster.
    Default value: `nil`.

`elasticsearch['auth_password']`

:   The password that Chef Automate will use if you have Elasticsearch
    X-Pack Basic Authentication enabled on your Elasticsearch cluster.
    Default value: `nil`.

### git

This configuration file has the following settings for `git`:

`git['authkeys']`

:   Default value: `git['ssh_dir'] + "/authorized_keys"`.

`git['home']`

:   Default value: `"/var/opt/delivery/home/git"`.

`git['shell']`

:   Default value: `"/opt/delivery/embedded/bin/git-shell"`.

`git['ssh_dir']`

:   Default value: `git['home'] + "/.ssh"`.

`git['username']`

:   Default value: `"git"`.

### java

This configuration file has the following settings for `java`:

`java['java_home']`

:   Default value:

    ``` ruby
    "#{node['delivery']['install_path']}/embedded/jre/bin"
    ```

### kibana

{{% kibana_note %}}

This configuration file has the following settings for `kibana`:

`kibana['enable']`

:   Enable the Kibana service. This is disabled by default. If you
    choose to enable it, you must have at least 2GB of extra RAM for
    Kibana to perform well. Default value: `'true'`.

`kibana['conf_dir']`

:   The working directory. The default value is the recommended value.
    Default value: `'/var/opt/delivery/kibana/'`.

`kibana['log_directory']`

:   The directory in which log data is stored. The default value is the
    recommended value. Default value: `"/var/log/delivery/kibana"`.

`kibana['log_rotation']['file_maxbytes']`

:   The log rotation policy for this service. Log files are rotated when
    they exceed `file_maxbytes`. The maximum number of log files in the
    rotation is defined by `num_to_keep`. Default value:
    `100 * 1024 * 1024` (100MB).

`kibana['log_rotation']['num_to_keep']`

:   The log rotation policy for this service. Log files are rotated when
    they exceed `file_maxbytes`. The maximum number of log files in the
    rotation is defined by `num_to_keep`. Default value: `10`.

`kibana['port']`

:   The port on which the service is to listen. Default value: `5601`.

### lb

This configuration file has the following settings for `lb`:

`lb['debug']`

:   Default value: `false`.

### logstash

This configuration file has the following settings for `logstash`:

`logstash['config_dir']`

:   The working directory. The default value is the recommended value.
    Default value: `"/var/opt/delivery/logstash"`.

`logstash['filebeats']['port']`

:   Default value: 5044.

`logstash['log_directory']`

:   The directory in which log data is stored. The default value is the
    recommended value. Default value: `"/var/log/delivery/logstash"`.

`logstash['log_rotation']['file_maxbytes']`

:   The log rotation policy for this service. Log files are rotated when
    they exceed `file_maxbytes`. The maximum number of log files in the
    rotation is defined by `num_to_keep`. Default value:
    `100 * 1024 * 1024` (100MB).

`logstash['log_rotation']['num_to_keep']`

:   The log rotation policy for this service. Log files are rotated when
    they exceed `file_maxbytes`. The maximum number of log files in the
    rotation is defined by `num_to_keep`. Default value: `10`.

`logstash['port']`

:   The port on which the service is to listen. Default value: `8080`.

`logstash['heap_size']`

:   The amount of memory allocated to the logstash heap. Default value:
    10% of system memory or 128 megabytes, whichever is larger. Requires
    Automate 0.8.46 or above.

### lsyncd

This configuration file has the following settings for `lsyncd`:

`lsyncd['dir']`

:   The working directory. The default value is the recommended value.
    Default value: `"/var/opt/delivery/lsyncd"`.

`lsyncd['enable']`

:   Enable a service. Default value: `true`.

`lsyncd['log_directory']`

:   The directory in which log data is stored. The default value is the
    recommended value. Default value: `"/var/log/delivery/lsyncd"`.

`lsyncd['log_rotation']['file_maxbytes']`

:   The log rotation policy for this service. Log files are rotated when
    they exceed `file_maxbytes`. The maximum number of log files in the
    rotation is defined by `num_to_keep`. Default value:
    `100 * 1024 * 1024` (100MB).

`lsyncd['log_rotation']['num_to_keep']`

:   The log rotation policy for this service. Log files are rotated when
    they exceed `file_maxbytes`. The maximum number of log files in the
    rotation is defined by `num_to_keep`. Default value: `10`.

`lsyncd['ssh_key']`

:   Default value:

    ``` ruby
    "#{node['delivery']['user']['home']}/.ssh/id_rsa"
    ```

`lsyncd['user']`

:   Default value: `node['delivery']['user']['username']`.

### nginx

This configuration file has the following settings for `nginx`:

`nginx['cache_max_size']`

:   The `max_size` parameter used by the Nginx cache manager, which is
    part of the `proxy_cache_path` directive. When the size of file
    storage exceeds this value, the Nginx cache manager removes the
    least recently used data. Default value: `'5000m'`.

`nginx['client_max_body_size']`

:   The maximum accepted body size for a client request, as indicated by
    the `Content-Length` request header. When the maximum accepted body
    size is greater than this value, a `413 Request Entity Too Large`
    error is returned. Default value: `'250m'`.

`nginx['dir']`

:   The working directory. The default value is the recommended value.
    Default value: `"/var/opt/delivery/nginx"`.

`nginx['enable']`

:   Enable a service. Default value: `true`.

`nginx['enable_non_ssl']`

:   Allow port 80 redirects to port 443. When this value is set to
    `true`, load balancers on the front-end hardware are allowed to do
    SSL termination of the WebUI and API. Default value: `false`.

`nginx['fqdns']`

:   An array of FQDN to which Nginx responds. Default value: `[]`.

`nginx['gzip']`

:   Enable gzip compression. Possible values: `on` or `off`. Default
    value: `'on'`.

`nginx['gzip_comp_level']`

:   The compression level used with gzip, from least amount of
    compression (`1`, fastest) to the most (`2`, slowest). Possible
    values: any integer between `1` and `9` (inclusive). Default value:
    `"2"`.

`nginx['gzip_http_version']`

:   Enable gzip depending on the version of the HTTP request. Possible
    values: `1.0` or `1.1`. Default value: `"1.0"`.

`nginx['gzip_proxied']`

:   The type of compression used based on the request and response.
    Possible values: `any` (gzip everything), `auth`, `expired`,
    `no-cache`, `no-store`, `no_etag`, `no_last_modified`, `off`, or
    `private`. Default value: <span class="title-ref">"any"</span>.

`nginx['gzip_types']`

:   Enable compression for the specified MIME-types. Default value:

    ``` ruby
    [ "text/plain", "text/css",
      "application/x-javascript", "text/xml",
      "application/javascript", "application/xml",
      "application/xml+rss", "text/javascript",
      "application/json" ]
    ]
    ```

`nginx['ha']`

:   Run the Chef Infra Server in a high availability topology. When
    `topology` is set to `ha`, this setting defaults to `true`. Default
    value: `false`.

`nginx['keepalive_timeout']`

:   The amount of time (in seconds) to wait for requests on a Keepalived
    connection. Default value: `65`.

`nginx['log_directory']`

:   The directory in which log data is stored. The default value is the
    recommended value. Default value: `"/var/log/delivery/nginx"`.

`nginx['log_rotation']['file_maxbytes']`

:   The log rotation policy for this service. Log files are rotated when
    they exceed `file_maxbytes`. The maximum number of log files in the
    rotation is defined by `num_to_keep`. Default value:
    `100 * 1024 * 1024` (100MB).

`nginx['log_rotation']['num_to_keep']`

:   The log rotation policy for this service. Log files are rotated when
    they exceed `file_maxbytes`. The maximum number of log files in the
    rotation is defined by `num_to_keep`. Default value: `10`.

`nginx['non_ssl_port']`

:   The port on which the WebUI and API are bound for non-SSL
    connections. Default value: `80`. Use `nginx['enable_non_ssl']` to
    enable or disable SSL redirects on this port number. Set to `false`
    to disable non-SSL connections.

`nginx['sendfile']`

:   Copy data between file descriptors when `sendfile()` is used.
    Possible values: `on` or `off`. Default value: `'on'`.

`nginx['server_name']`

:   The FQDN for the server. Default value: `node['delivery']['fqdn']`.

`nginx['ssl_ciphers']`

:   The list of supported cipher suites that are used to establish a
    secure connection. To favor AES256 with ECDHE forward security, drop
    the `RC4-SHA:RC4-MD5:RC4:RSA` prefix. See [this
    link](https://www.openssl.org/docs/man1.0.2/man1/ciphers.html) for more
    information. Default value:

    ``` ruby
    "RC4-SHA:RC4-MD5:RC4:RSA:HIGH:MEDIUM:!LOW:!kEDH:!aNULL:!ADH:!eNULL:!EXP:!SSLv2:!SEED:!CAMELLIA:!PSK"
    ```

`nginx['ssl_company_name']`

:   The name of your company. Default value: "Chef".

`nginx['ssl_country_name']`

:   The country in which your company is located. Default value: "US".

`nginx['ssl_email_address']`

:   The default email address for your company. Default value:
    `"delivery@getchef.com"`.

`nginx['ssl_locality_name']`

:   The city in which your company is located. Default value: "Seattle".

`nginx['ssl_organizational_unit_name']`

:   The organization or group within your company that is running the
    Chef Infra Server. Default value: "Engineering".

`nginx['ssl_port']`

:   Default value: `443`.

`nginx['ssl_protocols']`

:   The SSL protocol versions that are enabled. For the highest possible
    security, disable SSL 3.0 and allow only TLS:

    ``` ruby
    nginx['ssl_protocols'] = 'TLSv1 TLSv1.1 TLSv1.2'
    ```

    Default value: Default value: `"SSLv3 TLSv1"`.

`nginx['ssl_state_name']`

:   The state, province, or region in which your company is located.
    Default value: "WA".

`nginx['strict_host_header']`

:   Whether nginx should only respond to requests where the Host header
    matches one of the configured FQDNs. Default value: `false`.

    New in Automate version 1.7

`nginx['tcp_nodelay']`

:   Enable the Nagle buffering algorithm. Possible values: `on` or
    `off`. Default value: `'on'`.

`nginx['tcp_nopush']`

:   Enable TCP/IP transactions. Possible values: `on` or `off`. Default
    value: `'on'`.

`nginx['use_implicit_hosts']`

:   Automatically add <span class="title-ref">localhost</span> and any
    local IP addresses to the configured FQDNs. Useful in combination
    with `nginx['strict_host_header']`. Default value: `true`.

    New in Automate version 1.7

`nginx['worker_connections']`

:   The maximum number of simultaneous clients. Use with
    `nginx['worker_processes']` to determine the maximum number of
    allowed clients. Default value: `10240`.

`nginx['worker_processes']`

:   The number of allowed worker processes. Use with
    `nginx['worker_connections']` to determine the maximum number of
    allowed clients. Default value: `node['cpu']['total'].to_i`.

### notifications

The following settings allow you to customize the behavior of the event
notifications engine in Chef Automate:

`notifications['enable']`

:   Set to `true` to enable the addition, deletion, processing and
    dispatch of notifications. Default value: `true`.

`notifications['port']`

:   The internal network port on which the notifications service
    listens. Only change this if you encounter port collisions with
    other services. Default value: `9603`.

`notifications['conf_dir']`

:   The working directory. The default value is the recommended value.
    Default value: `'/var/opt/delivery/notifications'`.

`notifications['rule_store_file']`

:   Path to the file where the notification rules are stored. Default
    value: `'/var/opt/delivery/notifications/rule_store'`.

`notifications['log_directory']`

:   The directory in which log data is stored. The default value is the
    recommended value. Default value:
    `'/var/log/delivery/notifications'`.

`notifications['log_rotation']['file_maxbytes']`

:   The log rotation policy for this service. Log files are rotated when
    they exceed `file_maxbytes`. The maximum number of log files in the
    rotation is defined by `num_to_keep`. Default value:
    `1024 * 1000 * 10`.

`notifications['log_rotation']['num_to_keep']`

:   The log rotation policy for this service. Log files are rotated when
    they exceed `file_maxbytes`. The maximum number of log files in the
    rotation is defined by `num_to_keep`. Default value: `10`.

### postgresql

This configuration file has the following settings for `postgresql`:

`postgresql['checkpoint_completion_target']`

:   A completion percentage that is used to determine how quickly a
    checkpoint should finish in relation to the completion status of the
    next checkpoint. For example, if the value is `0.5`, then a
    checkpoint attempts to finish before 50% of the next checkpoint is
    done. Default value: `0.5`.

`postgresql['checkpoint_segments']`

:   The maximum amount (in megabytes) between checkpoints in log file
    segments. Default value: `3`.

`postgresql['checkpoint_timeout']`

:   The amount of time (in minutes) between checkpoints. Default value:
    `"5min"`.

`postgresql['checkpoint_warning']`

:   The frequency (in seconds) at which messages are sent to the server
    log files if checkpoint segments are being filled faster than their
    currently configured values. Default value: `"30s"`.

`postgresql['data_dir']`

:   The directory in which on-disk data is stored. The default value is
    the recommended value. Default value:

    ``` ruby
    "/var/opt/delivery/postgresql/#{node['delivery']['postgresql']['version']}/data"
    ```

`postgresql['debug']`

:   Default value: `false`.

`postgresql['dir']`

:   The working directory. The default value is the recommended value.
    Default value:

    ``` ruby
    "/var/opt/delivery/postgresql/#{node['delivery']['postgresql']['version']}"
    ```

`postgresql['effective_cache_size']`

:   The size of the disk cache that is used for data files. Default
    value: `"128MB"`.

`postgresql['enable']`

:   Enable a service. Default value: `true`.

`postgresql['ha']`

:   Run the Chef Infra Server in a high availability topology. When
    `topology` is set to `ha`, this setting defaults to `true`. Default
    value: `false`.

`postgresql['home']`

:   The home directory for PostgreSQL. Default value:
    `"/var/opt/delivery/postgresql"`.

`postgresql['listen_address']`

:   The connection source to which PostgreSQL is to respond. Default
    value: `'localhost'`. In a disaster recovery configuration, this
    value is similar to: `'localhost,192.0.2.0'`.

`postgresql['log_directory']`

:   The directory in which log data is stored. The default value is the
    recommended value. Default value:

    ``` ruby
    "/var/log/delivery/postgresql/#{node['delivery']['postgresql']['version']}"
    ```

`postgresql['log_rotation']['file_maxbytes']`

:   The log rotation policy for this service. Log files are rotated when
    they exceed `file_maxbytes`. The maximum number of log files in the
    rotation is defined by `num_to_keep`. Default value:
    `100 * 1024 * 1024` (100MB).

`postgresql['log_rotation']['num_to_keep']`

:   The log rotation policy for this service. Log files are rotated when
    they exceed `file_maxbytes`. The maximum number of log files in the
    rotation is defined by `num_to_keep`. Default value: `10`.

`postgresql['max_connections']`

:   The maximum number of allowed concurrent connections. Default value:
    `350`.

`postgresql['md5_auth_cidr_addresses']`

:   Use instead of `trust_auth_cidr_addresses` to encrypt passwords
    using MD5 hashes. Default value: `[ ]`.

`postgresql['port']`

:   The port on which the service is to listen. Default value: `5432`.

`postgresql['shared_buffers']`

:   The amount of memory that is dedicated to PostgreSQL for data
    caching. Default value:

    ``` ruby
    "#{(node['memory']['total'].to_i / 4) / (1024)}MB"
    ```

`postgresql['shell']`

:   Default value: `"/bin/bash"`.

`postgresql['shmall']`

:   The total amount of available shared memory. Default value:
    `4194304`.

`postgresql['shmmax']`

:   The maximum amount of shared memory. Default value: `17179869184`.

`postgresql['sql_password']`

:   The password for the PostgreSQL user account. Default value:
    `"snakepliskin"`.

`postgresql['sql_ro_password']`

:   Default value: `"shmunzeltazzen"`.

`postgresql['sql_ro_user']`

:   Default value: `"chef_ro"`.

`postgresql['sql_user']`

:   Default value: `"chef"`.

`postgresql['trust_auth_cidr_addresses']`

:   Use for clear-text passwords. See `md5_auth_cidr_addresses`. Default
    value: `[ '127.0.0.1/32', '::1/128' ]`.

`postgresql['user_path']`

:   Default value:

    ``` ruby
    "/opt/delivery/embedded/bin:/opt/delivery/bin:$PATH"
    ```

`postgresql['username']`

:   The PostgreSQL account user name. Default value: `"chef-pgsql"`.

`postgresql['work_mem']`

:   The size (in megabytes) of allowed in-memory sorting. Default value:
    `"8MB"`.

`postgresql['version']`

:   The (currently) hardcoded version of PostgreSQL. Default value:
    `"9.2"`.

`postgresql['vip']`

:   The virtual IP address. Default value: `"127.0.0.1"`.

### rabbitmq

This configuration file has the following settings for `rabbitmq`:

`rabbitmq['dir']`

:   The working directory. The default value is the recommended value.
    Default value: `'/var/opt/delivery/rabbitmq'`.

`rabbitmq['data_dir']`

:   The directory in which on-disk data is stored. The default value is
    the recommended value. Default value:
    `'/var/opt/delivery/rabbitmq/db'`.

`rabbitmq['env_path']`

:   Default value:

    ``` ruby
    '/opt/delivery/bin:/opt/delivery/embedded/bin:/usr/bin:/bin'
    ```

`rabbitmq['log_directory']`

:   The directory in which log data is stored. The default value is the
    recommended value. Default value:

    ``` ruby
    File.join(default_log_directory, "rabbitmq")
    ```

`rabbitmq['log_rotation']['file_maxbytes']`

:   The log rotation policy for this service. Log files are rotated when
    they exceed `file_maxbytes`. The maximum number of log files in the
    rotation is defined by `num_to_keep`. Default value:
    `100 * 1024 * 1024` (100MB).

`rabbitmq['log_rotation']['num_to_keep']`

:   The log rotation policy for this service. Log files are rotated when
    they exceed `file_maxbytes`. The maximum number of log files in the
    rotation is defined by `num_to_keep`. Default value: `10`.

`rabbitmq['management_enabled']`

:   Specify if the rabbitmq-management plugin is enabled. Default value:
    `true`.

`rabbitmq['management_password']`

:   The rabbitmq-management plugin password. Default value:
    `'chefrocks'`.

`rabbitmq['management_port']`

:   The rabbitmq-management plugin port. Default value: `15672`.

`rabbitmq['management_user']`

:   The rabbitmq-management plugin user. Default value: `'rabbitmgmt'`.

`rabbitmq['node_ip_address']`

:   The bind IP address for RabbitMQ. Default value: `'0.0.0.0'`.

`rabbitmq['nodename']`

:   The name of the node. Default value: `'rabbit@localhost'`.

`rabbitmq['password']`

:   The password for the RabbitMQ user. Default value: `'chefrocks'`.

`rabbitmq['port']`

:   The port on which the service is to listen. Default value: `'5672'`.

`rabbitmq['vip']`

:   The virtual IP address. Default value: `'127.0.0.1'`.

`rabbitmq['use_ssl']`

:   Whether or not to enable the ssl service. Default value: `true`.

`rabbitmq['ssl_certificate']` and `rabbitmq['ssl_certificate_key']`

:   SSL certificate used for rabbitmq communication only if
    `rabbitmq['use_ssl']` is `true`. Certificates provide by user will
    be readable by the <span class="title-ref">delivery</span> user. If
    both of these are nil, we generate a self-signed certificate.
    Default value: `nil`.

`rabbitmq['ssl_versions']`

:   The version for the ssl service. Default value:
    `[ 'tlsv1.2', 'tlsv1.1' ]`.

### ssh_git

This configuration file has the following settings for `ssh_git`:

`ssh_git['hostname']`

:   Default value: `nil`.

`ssh_git['keys_dir']`

:   The working directory. The default value is the recommended value.
    Default value:

    ``` ruby
    "#{node['delivery']['delivery']['etc_dir']}/ssh_git_server_keys"
    ```

`ssh_git['port']`

:   The port on which the service is to listen. Default value: `8989`.

### statistics

This configuration file has the following settings for `statistics`:

`statistics['enable']`

:   Whether or not to enable the statistics service. Default value:
    `true`.

`statistics['port']`

:   The listen port of the statistics service. Default value: `7676`.

`statistics['bind_address']`

:   The listen bind address of the statistics service. Default value:
    `127.0.0.1`.

`statistics['log_directory']`

:   The statistics log file location. Default value:
    `/var/log/delivery/statistics`.

`statistics['log_rotation']['file_maxbytes']`

:   The statistics log file max bytes. Default value: `104857600`.

`statistics['log_rotation']['num_to_keep']`

:   The maxiumum number of statistics log files. Default value: `10`.

### user

This configuration file has the following settings for `user`:

`user['home']`

:   The home directory for the delivery services user. Default value:
    `"/opt/delivery/embedded"`.

`user['shell']`

:   The shell for the delivery services user. Default value:
    `"/bin/bash"`.

`user['username']`

:   The username for the delivery services user. Default value:
    `"delivery"`.
