+++
title = "supermarket.rb Settings"
draft = false

aliases = ["/config_rb_supermarket.html"]

[menu]
  [menu.infra]
    title = "supermarket.rb Settings"
    identifier = "chef_infra/setup/supermarket/config_rb_supermarket.md supermarket.rb Settings"
    parent = "chef_infra/setup/supermarket"
    weight = 40
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/config_rb_supermarket.md)

{{% config_rb_supermarket_summary %}}

{{< note >}}

The `supermarket.rb` file does not exist by default. To modify the
settings for the Supermarket server, create a file named
`supermarket.rb` in the `/etc/supermarket/` directory.

{{< /note >}}

## Settings

The following settings are available in the `supermarket.rb` file.

{{< note >}}

You must run `supermarket-ctl reconfigure` to apply any changes made in
the `supermarket.rb` file.

{{< /note >}}

### General

This configuration file has the following general settings:

`default['enterprise']['name']`

:   The enterprise name that is used by the
    [enterprise-chef-common](https://github.com/chef-cookbooks/enterprise-chef-common)
    cookbook. Default value: `'supermarket'`.

`default['supermarket']['app_directory']`

:   Default value:
    `"#{node['supermarket']['install_directory']}/embedded/service/supermarket"`.

`default['supermarket']['chef_server_url']`

:   The URL of the Chef Infra Server.

`default['supermarket']['config_directory']`

:   The directory that is used to store Supermarket configuration files.
    Default value: `'/etc/supermarket'`.

`default['supermarket']['features']`

:   Use to enable additional features, such as announcements and GitHub
    integration. Default value: `'tools'`.

    Features currently available: `tools`, `fieri`, `announcement`,
    `github`, and `no_crawl`.

`default['supermarket']['fqdn']`

:   The fully qualified domain name for the Supermarket server. Defaults
    to using the current FQDN for the machine.

`default['supermarket']['from_email']`

:   The default sender address of all Supermarket mailers. Default
    value: `nil`.

`default['supermarket']['group']`

:   The system group that is used to manage Supermarket on the server.
    Default value: `'supermarket'`.

`default['supermarket']['install_directory']`

:   The directory where Supermarket is installed. Default value:
    `'/opt/supermarket'`.

`default['supermarket']['install_path']`

:   The directory in which Chef Supermarket is installed. Default value:
    `node['supermarket']['install_directory']`.

`default['supermarket']['log_directory']`

:   The directory that Supermarket will use to store logs. Default
    value: `'/var/log/supermarket'`.

`default['supermarket']['sysvinit_id']`

:   Use to specify 1-4 characters that define a unique identifier for
    the file located in `/etc/inittab`. Default value: `SUP`.

`default['supermarket']['user']`

:   The system user that is used to run Supermarket. Default value:
    `'supermarket'`.

`default['supermarket']['var_directory']`

:   The directory where data and cookbooks are installed. Default value:
    `'/var/opt/supermarket'`.

### Amazon Simple Storage Service (S3)

Use these settings to upload cookbooks to an Amazon Simple Storage
Service (S3) bucket.

{{< note >}}

Encrypted S3 buckets are currently not supported.

{{< /note >}}

`default['supermarket']['cdn_url']`

:   The URL for the content delivery network. (optional)

`default['supermarket']['s3_access_key_id']`

:   The secret key. (required to use S3)

`default['supermarket']['s3_bucket']`

:   The bucket name. (required to use S3)

`default['supermarket']['s3_path']`

:   **(Optional)** Directory structure to prepend to the standard path
    of the directory containing cookbooks. Set this if you must store
    cookbooks in a deeper directory structure within a shared bucket.
    However, keep in mind that dedicated S3 buckets are recommended for
    cookbook storage and distribution.

`default['supermarket']['s3_private_objects']`

:   Whether cookbooks stored in S3 should be public or private.
    `true/false` Default: `false`

`default['supermarket']['s3_region']`

:   The region of the bucket. (required to use S3)

`default['supermarket']['s3_secret_access_key']`

:   The access key identifier. (required to use S3)

### Database

The following database options are available:

`default['supermarket']['database']['extensions']`

:   Determines which PostgreSQL extensions are enabled. Default value:
    `{ 'pgpsql' => true, 'pg_trgm' => 'true' }`.

`default['supermarket']['database']['host']`

:   The address PostgreSQL listens on. Default value:
    `node['supermarket']['postgresql']['listen_address']`.

`default['supermarket']['database']['name']`

:   The name of the Supermarket database. Default value:
    `'supermarket'`.

`default['supermarket']['database']['pool']`

:   The number of concurrent threads a database worker can create.
    Default value: `node['supermarket']['sidekiq']['concurrency']`.

`default['supermarket']['database']['port']`

:   The port that the database listens on. Default value:
    `node['supermarket']['postgresql']['port']`.

`default['supermarket']['database']['user']`

:   The database user. Default value:
    `node['supermarket']['postgresql']['username']`.

`default['supermarket']['postgresql']['username']`

:   The system user that runs PostgreSQL. By default, this uses the
    value of `node['supermarket']['user']`.

### Fieri

Use these settings to enable [Fieri](/supermarket/#fieri), an
optional service built into Supermarket that provides cookbook quality
metrics.

As a Supermarket feature, Fieri must be enabled via the
`default['supermarket']['features']` option.

`default['supermarket']['fieri_url']`

:   The full URL that is used to access Fieri. Default value:
    `'http://localhost:13000/fieri/jobs'`

`default['supermarket']['fieri_supermarket_endpoint']`

:   The URL of the Chef Supermarket that is using Fieri. Default value:
    `'https://localhost:13000'`

`default['supermarket']['fieri_key']`

:   A string that is used as a key to authenticate Fieri. Default value:
    `nil`

### GitHub

Use these settings to integrate Supermarket with GitHub.

As a Supermarket feature, GitHub must be enabled via the
`default['supermarket']['features']` option.

`default['supermarket']['github_access_token']`

:   The access token created from your GitHub account. Default value:
    `nil`.

`default['supermarket']['github_key']`

:   The application client ID that is used to authenticate Supermarket
    to GitHub. Default value: `nil`.

`default['supermarket']['github_secret']`

:   The application client secret that is used to authenticate
    Supermarket to GitHub. Default value: `nil`.

### Google Analytics

Use this setting to set up [Google
Analytics](https://analytics.google.com) tracking for Supermarket:

`default['supermarket']['google_analytics_id']`

:   The Google Analytics [tracking
    ID](https://support.google.com/analytics/answer/7372977?hl=en) for
    Supermarket. Default value: `nil`.

### New Relic

Use these settings to integrate Supermarket with [New
Relic](https://newrelic.com/), a software analytics platform:

`default['supermarket']['newrelic_agent_enabled']`

:   Determines whether or not the New Relic agent is enabled. Default
    value: `'false'`.

`default['supermarket']['newrelic_app_name']`

:   The name used by New Relic to identify the Supermarket installation.
    Default value: `nil`.

`default['supermarket']['newrelic_license_key']`

:   The New Relic license key. Default value: `nil`.

### Nginx

This configuration file has the following settings for nginx:

`default['supermarket']['nginx']['access_log_options']`

:   A string of [additional
    options](https://nginx.org/en/docs/http/ngx_http_log_module.html) to
    be added to the nginx access log directive. Default value: `nil`.

`default['supermarket']['nginx']['cache']['directory']`

:   The directory used by nginx for caching. Default value:
    `"#{node['supermarket']['var_directory']}/nginx//cache"`.

`default['supermarket']['nginx']['cache']['enable']`

:   Determines whether or not nginx caching is enabled. Default value:
    `false`.

`default['supermarket']['nginx']['client_body_buffer_size']`

:   The
    [client_body_buffer_size](https://nginx.org/en/docs/http/ngx_http_core_module.html#client_body_buffer_size)
    used by nginx. Default value: `nil`.

`default['supermarket']['nginx']['client_max_body_size']`

:   The maximum accepted body size for a client request, as indicated by
    the `Content-Length` request header. When the maximum accepted body
    size is greater than this value, a `413 Request Entity Too Large`
    error is returned. Default value: `250m`. See the [nginx
    documentation](https://nginx.org/en/docs/http/ngx_http_core_module.html#client_max_body_size)
    for additional information.

`default['supermarket']['nginx']['daemon_disable']`

:   Determines whether or not nginx is daemonized. By default, this will
    be handled by the init system. Default value: `true`.

`default['supermarket']['nginx']['default']['modules']`

:   Determines which additional [nginx
    modules](https://www.nginx.com/resources/wiki/modules/) should be
    included. Default value: `[]`.

`default['supermarket']['nginx']['default_site_enabled']`

:   Determines whether or not the nginx default page is enabled. Default
    value: `false`.

`default['supermarket']['nginx']['dir']`

:   The working directory. The default value is the recommended value.
    Default value: `node['supermarket']['nginx']['directory']`.

`default['supermarket']['nginx']['disable_access_log']`

:   Allows you to disable the nginx access log. Default value: `false`.

`default['supermarket']['nginx']['error_log_options']`

:   A string of [additional
    options](https://nginx.org/en/docs/http/ngx_http_log_module.html) to
    be added to the nginx access log directive. Default value: `nil`.

`default['supermarket']['nginx']['enable']`

:   Enable the nginx service. Default value: `true`.

`default['supermarket']['nginx']['event']`

:   Set the event-model. By default nginx looks for the most suitable
    method for your OS. Default value: `nil`.

`default['supermarket']['nginx']['force_ssl']`

:   Force connections to use SSL. Default value: `true`.

`default['supermarket']['nginx']['group']`

:   The system group that is used to manage nginx. Default value:
    `node['supermarket']['group']`.

`default['supermarket']['nginx']['gzip']`

:   Enable gzip compression. Default value: `on`.

`default['supermarket']['gzip_buffers']`

:   Set the <span class="title-ref">gzip buffer
    \<https://nginx.org/en/docs/http/ngx_http_gzip_module.html\#gzip_buffers\></span>
    size. The nginx default is equal to one memory page. Default value:
    `nil`.

`default['supermarket']['nginx']['gzip_comp_level']`

:   The compression level used with gzip, from least amount of
    compression (`1`, fastest) to the most (`2`, slowest). Default
    value: `2`.

`default['supermarket']['gzip_disable']`

:   Disables gzip compression when a `User-Agent` field is present in
    headers matching the specified regular expressions. Default value:
    `'MSIE [1-6]\.'`.

`default['supermarket']['nginx']['gzip_http_version']`

:   Enable gzip depending on the version of the HTTP request. Default
    value: `1.0`.

`default['supermarket']['gzip_min_length']`

:   The minimum reponse length that will be compressed by gzip, as
    determined by the `Content-Length` response header. Default value:
    `1000`.

`default['supermarket']['nginx']['gzip_proxied']`

:   Determines whether or not proxied requests are compressed with gzip,
    based on the presence of the `Via` request header field. Default
    value: `any`.

`default['supermarket']['nginx']['gzip_static']`

:   Allows you to send precompressed files with the `.gz` file extension
    instead of regular files. Requires the
    [ngx_http_gzip_static_module](https://nginx.org/en/docs/http/ngx_http_gzip_static_module.html)
    module. Default value: `'off'`.

`default['supermarket']['nginx']['gzip_types']`

:   Enable compression for the specified MIME-types. Default value:
    `[ 'text/plain', 'text/css', 'application/x-javascript', 'text/xml', 'application/xml', 'application/xml+rss', 'application/atom+xml', 'text/javascript', 'application/javascript', 'application/json' ]`.

`default['supermarket']['gzip_vary']`

:   Determines whether or not the `Vary: Accept-Encoding` response
    header field is inserted when the following directives are active:
    `gzip`,`gzip_static`, or `gunzip`. Default value: `'off'`.

`default['supermarket']['nginx']['keepalive']`

:   Use to enable HTTP keepalive. Default value: `'on'`.

`default['supermarket']['nginx']['keepalive_timeout']`

:   The amount of time (in seconds) to wait for requests on a HTTP
    keepalive connection. Default value: `65`.

`default['supermarket']['nginx']['log_dir']`

:   The directory in which log data is stored. The default value is the
    recommended value. Default value:
    `node['supermarket']['nginx']['log_directory']`.

`default['supermarket']['nginx']['log_rotation']`

:   The log rotation policy for this service. Log files are rotated when
    they exceed `file_maxbytes`. The maximum number of log files in the
    rotation is defined by `num_to_keep`. Default value:
    `{ 'file_maxbytes' => 104857600, 'num_to_keep' => 10 }`

`default['supermarket']['nginx']['multi_accept']`

:   Determines whether a worker process accepts a single connection at a
    time, or all new connections at one time. The default value sets
    this to a single connection at a time. Default value: `false`.

`default['supermarket']['nginx']['non_ssl_port']`

:   The port on which the WebUI and API are bound for non-SSL
    connections. Default value: `80`. Set to `false` to disable non-SSL
    connections.

`default['supermarket']['nginx']['pid']`

:   The system process ID for the nginx service. Default value:
    `"#{node['supermarket']['nginx']['directory']}/nginx.pid"`.

`default['supermarket']['nginx']['proxy_read_timeout']`

:   Defines a timeout between two successive read operations for reading
    a response from the proxied server. Default value: `nil`.

`default['supermarket']['nginx']['redirect_to_canonical']`

:   Redirect requests to the Supermarket server FQDN. Default value:
    `true`.

`default['supermarket']['nginx']['sendfile']`

:   Copy data between file descriptors when `sendfile()` is used.
    Default value: `on`.

`default['supermarket']['nginx']['server_names_hash_bucket_size']`

:   The size of the bucket that contains the server names hash tables.
    Default value: `64`.

`default['supermarket']['nginx']['server_tokens']`

:   Determines whether or not the nginx version is included in error
    pages and the `Server` response header. Default value: `nil`.

`default['supermarket']['nginx']['ssl_port']`

:   The port that is used by nginx to terminate SSL connections. Default
    value: `443`.

`default['supermarket']['nginx']['types_hash_bucket_size']`

:   Determines the bucket size for the types hash tables. Default value:
    `64`.

`default['supermarket']['nginx']['types_hash_max_size']`

:   Sets the maximum size of the types hash table. Default value:
    `2048`.

`default['supermarket']['nginx']['user']`

:   The system user that is used to run nginx. Default value:
    `node['supermarket']['user']`.

`default['supermarket']['nginx']['worker_connections']`

:   The maximum number of simultaneous clients. Use with
    `nginx['worker_processes']` to determine the maximum number of
    allowed clients. Default value: `1024`.

`default['supermarket']['nginx']['worker_rlimit_nofile']`

:   Determines the maximum number of open files allowed for worker
    processes. Default value: `nil`.

`default['supermarket']['nginx']['worker_processes']`

:   The number of allowed worker processes. Use with
    `nginx['worker_connections']` to determine the maximum number of
    allowed clients. Default value:
    `node['cpu'] && node['cpu']['total'] ? node['cpu']['total'] : 1`.

### Oauth2

This configuration file has the following settings for the Chef Infra
Server identity service:

`default['supermarket']['chef_oauth2_app_id']`

:   The [Chef Identity](/install_supermarket/#chef-identity)
    application ID created for Supermarket on the Chef Infra Server. See
    the [Chef Identity
    configuration](/install_supermarket/#configure) section of the
    Supermarket installation guide for additional details.

`default['supermarket']['chef_oauth2_secret']`

:   The [Chef Identity](/install_supermarket/#chef-identity)
    application secret created for Supermarket on the Chef Infra Server.
    See the [Chef Identity
    configuration](/install_supermarket/#configure) section of the
    Supermarket installation guide for additional details.

`default['supermarket']['chef_oauth2_url']`

:   The URL of the Chef Infra Server that Supermarket connects to.
    Default value: `node['supermarket']['chef_server_url']`.

`default['supermarket']['chef_oauth2_verify_ssl']`

:   Determines whether or not Supermarket performs SSL verification.
    Default value: `true`. If your Chef Infra Server is using a
    self-signed certificate without a properly configured certificate
    authority, this must be set to `false`.

### PostgreSQL

This configuration file has the following settings for PostgreSQL:

`default['supermarket']['postgresql']['checkpoint_completion_target']`

:   A completion percentage that is used to determine how quickly a
    checkpoint should finish in relation to the completion status of the
    next checkpoint. For example, if the value is `0.5`, then a
    checkpoint attempts to finish before 50% of the next checkpoint is
    done. Default value: `0.5`.

`default['supermarket']['postgresql']['checkpoint_segments']`

:   The maximum amount (in megabytes) between checkpoints in log file
    segments. Default value: `3`.

`default['supermarket']['postgresql']['checkpoint_timeout']`

:   The amount of time (in minutes) between checkpoints. Default value:
    `'5min'`.

`default['supermarket']['postgresql']['checkpoint_warning']`

:   The frequency (in seconds) at which messages are sent to the server
    log files if checkpoint segments are being filled faster than their
    currently configured values. Default value: `'30s'`.

`default['supermarket']['postgresql']['data_directory']`

:   The directory in which on-disk data is stored. The default value is
    the recommended value. Default value:
    `"#{node['supermarket']['var_directory']}/postgresql/9.3/data"`.

`default['supermarket']['postgresql']['effective_cache_size']`

:   The size of the disk cache that is used for data files. Default
    value: `'128MB'`.

`default['supermarket']['postgresql']['enable']`

:   Enable a service. Default value: `true`.

`default['supermarket']['postgresql']['listen_address']`

:   The connection source to which PostgreSQL is to respond. Default
    value: `'127.0.0.1'`.

`default['supermarket']['postgresql']['log_directory']`

:   The directory in which log data is stored. The default value is the
    recommended value. Default value:
    `"#{node['supermarket']['log_directory']}/postgresql"`.

`default['supermarket']['postgresql']['log_rotation']`

:   The log rotation policy for this service. Log files are rotated when
    they exceed `file_maxbytes`. The maximum number of log files in the
    rotation is defined by `num_to_keep`. Default value:
    `{ 'file_maxbytes' => 104857600, 'num_to_keep' => 10 }`

`default['supermarket']['postgresql']['max_connections']`

:   The maximum number of allowed concurrent connections. Default value:
    `350`.

`default['supermarket']['postgresql']['md5_auth_cidr_addresses']`

:   Use to encrypt passwords using MD5 hashes. Default value:
    `['127.0.0.1/32', '::1/128']`.

`default['supermarket']['postgresql']['port']`

:   The port on which the service is to listen. Default value: `15432`.

`default['supermarket']['postgresql']['shared_buffers']`

:   The amount of memory that is dedicated to PostgreSQL for data
    caching. Default value:
    `"#{(node['memory']['total'].to_i / 4) / (1024)}MB"`.

`default['supermarket']['postgresql']['shmall']`

:   The total amount of available shared memory. Default value:
    `4194304`.

`default['supermarket']['postgresql']['shmmax']`

:   The maximum amount of shared memory. Default value: `17179869184`.

`default['supermarket']['postgresql']['work_mem']`

:   The size (in megabytes) of allowed in-memory sorting. Default value:
    `'8MB'`.

### Redis

This configuration file has the following settings for Redis:

`default['supermarket']['redis']['bind']`

:   Bind Redis to the specified IP address. Default value:
    `'127.0.0.1'`.

`default['supermarket']['redis']['directory']`

:   The working directory. The default value is the recommended value.
    Default value: `"#{node['supermarket']['var_directory']}/redis"`.

`default['supermarket']['redis']['enable']`

:   Enable a service. Default value: `true`.

`default['supermarket']['redis']['log_directory']`

:   The directory in which log data is stored. The default value is the
    recommended value. Default value:
    `"#{node['supermarket']['log_directory']}/redis"`.

`default['supermarket']['redis']['log_rotation']`

:   The log rotation policy for this service. Log files are rotated when
    they exceed `file_maxbytes`. The maximum number of log files in the
    rotation is defined by `num_to_keep`. Default value:
    `{ 'file_maxbytes' => 104857600, 'num_to_keep' => 10 }`

`default['supermarket']['redis']['port']`

:   The port on which the service is to listen. Default value:
    `'16379'`.

### Ruby on Rails

This configuration file has the following settings for Ruby on Rails:

`default['supermarket']['rails']['enable']`

:   Enable a service. Default value: `true`.

`default['supermarket']['rails']['log_directory']`

:   The directory in which log data is stored. The default value is the
    recommended value. Default value:
    `"#{node['supermarket']['log_directory']}/rails"`.

`default['supermarket']['rails']['log_rotation']`

:   The log rotation policy for this service. Log files are rotated when
    they exceed `file_maxbytes`. The maximum number of log files in the
    rotation is defined by `num_to_keep`. Default value:
    `{ 'file_maxbytes' => 104857600, 'num_to_keep' => 10 }`

`default['supermarket']['rails']['port']`

:   The port on which the service is to listen. Default value: `13000`.

### runit

This configuration file has the following settings for runit:

`default['supermarket']['runit']['svlogd_bin']`

:   Default value:
    `"#{node['supermarket']['install_directory']}/embedded/bin/svlogd"`.

### Sentry

This option is used to integrate Supermarket with the
[Sentry](https://sentry.io/welcome/) error logging service:

`default['supermarket']['sentry_url']`

:   The Sentry URL that is used to send error reports. Default value:
    `nil`.

### Sidekiq

This configuration file has the following settings for background
processes that are managed by Sidekiq:

`default['supermarket']['sidekiq']['concurrency']`

:   Determines how many threads a Sidekiq process can spin up. Default
    value: `25`.

`default['supermarket']['sidekiq']['enable']`

:   Enable the Sidekiq service. Default value: `true`.

`default['supermarket']['sidekiq']['log_directory']`

:   The directory in which log data is stored. The default value is the
    recommended value. Default value:
    `"#{node['supermarket']['log_directory']}/sidekiq"`.

`default['supermarket']['sidekiq']['log_rotation']`

:   The log rotation policy for this service. Log files are rotated when
    they exceed `file_maxbytes`. The maximum number of log files in the
    rotation is defined by `num_to_keep`. Default value:
    `{ 'file_maxbytes' => 104857600, 'num_to_keep' => 10 }`

`default['supermarket']['sidekiq']['timeout']`

:   The amount of time (in seconds) that Sidekiq should wait for a
    worker before it is terminated. Default value: `30`.

### SMTP

This configuration file has the following settings for SMTP:

`default['supermarket']['smtp_address']`

:   The SMTP server address that Supermarket will use to send mail.

`default['supermarket']['smtp_password']`

:   The SMTP server password.

`default['supermarket']['smtp_port']`

:   The port on which the service is to listen.

`default['supermarket']['smtp_user_name']`

:   The user on the SMTP server.

`default['supermarket']['from_email']`

:   The default sender address of all Supermarket mailers. Default
    value: `nil`.

### SSL

This configuration file has the following settings for SSL:

`default['supermarket']['ssl']['certificate']`

:   The SSL certificate used to verify communication over HTTPS.

`default['supermarket']['ssl']['certificate_key']`

:   The certificate key used for SSL communication.

`default['supermarket']['ssl']['company_name']`

:   The name of your company. Default value: `'My Supermarket'`.

`default['supermarket']['ssl']['country_name']`

:   The country in which your company is located. Default value: `'US'`.

`default['supermarket']['ssl']['directory']`

:   The working directory. Default value: `'/var/opt/supermarket/ssl'`.

`default['supermarket']['ssl']['email_address']`

:   The default email address for your company. Default value:
    `'you@example.com'`.

`default['supermarket']['ssl']['locality_name']`

:   The city in which your company is located. Default value:
    `'Seattle'`.

`default['supermarket']['ssl']['openssl_bin']`

:   Default value:
    `"#{node['supermarket']['install_directory']}/embedded/bin/openssl"`.

`default['supermarket']['ssl']['organizational_unit_name']`

:   The organization or group within your company that is running the
    Chef Infra Server. Default value: `'Operations'`.

`default['supermarket']['ssl']['session_cache']`

:   Default value: `'shared:SSL:4m'`.

`default['supermarket']['ssl']['session_timeout']`

:   Default value: `'5m'`.

`default['supermarket']['ssl']['ciphers']`

:   The list of supported cipher suites that are used to establish a
    secure connection. To favor AES256 with ECDHE forward security, drop
    the `RC4-SHA:RC4-MD5:RC4:RSA` prefix. See
    <https://www.openssl.org/docs/man1.0.2/man1/ciphers.html> for more
    information. For example:

    ``` ruby
    nginx['ssl_ciphers'] = HIGH:MEDIUM:!LOW:!kEDH:!aNULL:!ADH:!eNULL:!EXP:!SSLv2:!SEED:!CAMELLIA:!PSK
    ```

`default['supermarket']['ssl']['protocols']`

:   The SSL protocol versions that are enabled. Default value:
    `'TLSv1 TLSv1.1 TLSv1.2'`.

`default['supermarket']['ssl']['state_name']`

:   The state, province, or region in which your company is located.
    Default value: `'WA'`.

### StatsD

This configuration file has the following settings for reporting to a
StatsD server:

`default['supermarket']['statsd_port']`

:   The port on which the service is to listen. Default value: `nil`.

`default['supermarket']['statsd_url']`

:   The URL to which reporting metrics are sent. Default value: `nil`.

### URLs

Use these settings to replace `chef.io` URLs with your own internal
mirrors or alternatives.

`default['supermarket']['chef_blog_url']`

:   The URL of the Chef blog. Default value:
    `"https://www.#{node['supermarket']['chef_domain']}/blog"`.

`default['supermarket']['chef_docs_url']`

:   The URL of the Chef Docs site. Default value:
    `"https://docs.#{node['supermarket']['chef_domain']}"`.

`default['supermarket']['chef_downloads_url']`

:   The URL of the Chef downloads page. Default value:
    `"https://downloads.#{node['supermarket']['chef_domain']}"`.

`default['supermarket']['chef_domain']`

:   The root domain that is used by all Chef URLs. Most of the settings
    in this section rely upon this setting. Default value: `'chef.io'`.

`default['supermarket']['chef_identity_url']`

:   The URL that is used to interact with Chef Identity on the Chef
    Infra Server. Default value:
    `"#{node['supermarket']['chef_server_url']}/id"`.

`default['supermarket']['chef_profile_url']`

:   The URL that is used to log in to your Chef profile. Default value:
    `node['supermarket']['chef_server_url']`.

`default['supermarket']['chef_sign_up_url']`

:   The community signup URL. Default value:
    `"#{node['supermarket']['chef_server_url']}/signup?ref=community"`.

`default['supermarket']['chef_www_url']`

:   The Chef website URL. Default value:
    `"https://www.#{node['supermarket']['chef_domain']}"`.

`default['supermarket']['learn_chef_url']`

:   The Learn Chef Rally URL. Default value:
    `"https://learn.#{node['supermarket']['chef_domain']}"`.
