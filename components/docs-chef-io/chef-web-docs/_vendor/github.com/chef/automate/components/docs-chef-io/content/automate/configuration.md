+++
title = "Configuration"

date = 2018-05-08T18:54:09+00:00
draft = false
[menu]
  [menu.automate]
    title = "Configuration"
    parent = "automate/configuring_automate"
    identifier = "automate/configuring_automate/configuration.md Configuration"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/configuration.md)

The `chef-automate` CLI provides commands to help you work with your existing Chef Automate configuration:

* `chef-automate config show` shows your current configuration, not including default settings
* `chef-automate config patch </path/to/partial-config.toml>` updates an existing Chef Automate configuration by merging the contents of`</path/to/partial-config.toml>` with your current Chef Automate configuration, and applying any changes. This command is enough in most situations
* `chef-automate config set </path/to/full-config.toml>` replaces the current Chef Automate configuration with the provided configuration, and applies any changes. Use this command to replace your Chef Automate configuration

Update your Chef Automate configuration by generating a section of a configuration, and applying it with `chef-automate config patch`.
The rest of this document describes how to make common configuration changes.

## Use Cases

### Minimal Configuration

The `chef-automate init-config` command generates an annotated Chef Automate configuration file with the basic settings needed to deploy Chef Automate.
This section describes those settings and how to change them on an existing Chef Automate installation.

#### Chef Automate FQDN

To change the fully qualified domain name (FQDN) of your Chef Automate installation, create a TOML file that contains the partial configuration:

```TOML
[global.v1]
  fqdn = "{{< example_fqdn "automate" >}}"
```

Then run `chef-automate config patch </path/to/your-file.toml>` to deploy your change.

#### Install Channel

Chef Automate consists of [Habitat](https://www.habitat.sh/) packages installed from a release channel.
The default channel is `current`.

#### Upgrade Strategy

The upgrade strategy determines when a Chef Automate installation upgrades.
The upgrade strategy settings include:

* `at-once` (default) upgrades the installation after detecting new packages in the install channel
* `none` freezes the installation with its current set of packages

Changing the upgrade strategy from `none` to `at-once` will install the latest packages from the install channel.

To change the upgrade strategy of your Chef Automate installation, create a TOML file that contains the partial configuration:

```toml
[deployment.v1.svc]
upgrade_strategy = "at-once"
```

Then run `chef-automate config patch </path/to/your-file.toml>` to deploy your change.

To upgrade a Chef Automate installation with `upgrade_strategy` set to `none`, run:

```bash
chef-automate upgrade run
```

This command will upgrade Chef Automate to the latest version from your install channel.

#### Deployment Type

Do not change `deployment_type`.
The only supported `deployment_type` is `local`.

#### Settings

You cannot change the admin username, name, and password set during initial deployment.

To change the admin password after deployment, use the Chef Automate UI.
Sign in as the admin user, navigate to the _Users_ page under the **Settings** tab.
Select "Local Administrator" to show the admin's _User Details_ page.
Navigate to the _Reset Password_ tab.
Enter your previous password, and enter and confirm your new password in the interface.
Select the **Reset Password** button to save your changes.

To change the admin password from the command-line, first [fetch the admin user record](/automate/api/), copy the User ID, and then use:

```bash
export TOKEN=`chef-automate iam token create admin-token-1 --admin`

curl -X PUT -H "api-token: $TOKEN" -H "Content-Type: application/json" -d '{"name":"Local Administrator", "id": "<admin user ID>", "password":"<password>"}' https://{{< example_fqdn "automate" >}}/api/v0/auth/users/admin?pretty
```

#### Load Balancer Certificate and Private Key

To change the load balancer certificate and private key of your Chef Automate installation, create a TOML file that contains the partial configuration:

```toml
[[global.v1.frontend_tls]]
# The TLS certificate for the load balancer frontend.
cert = """-----BEGIN CERTIFICATE-----
<your certificate>
-----END CERTIFICATE-----
"""

# The TLS RSA key for the load balancer frontend.
key = """-----BEGIN RSA PRIVATE KEY-----
<your private key>
-----END RSA PRIVATE KEY-----
"""
```

Then run `chef-automate config patch </path/to/your-file.toml>` to deploy your change.

#### License Key

You can apply your Chef Automate license with the `chef-automate license apply` command in one of two ways:

* `chef-automate license apply </path/to/license-file.jwt>`
* `chef-automate license apply <content-of-license>`

You cannot apply a license after your initial deployment by patching the configuration file.

#### Proxy Settings

You can configure Chef Automate to use a proxy either by setting environment variables, or by setting configuration options.

The command `chef-automate deploy` without a configuration file will respect the proxy environment variables:

* `HTTPS_PROXY`/`https_proxy`
* `HTTP_PROXY`/`http_proxy`
* `NO_PROXY`/`no_proxy` (See [Required Sites and Domains]({{< relref "#required-sites-and-domains" >}}).)

Setting these environment variables before the initial deployment of Chef Automate adds them to the configuration that Chef Automate generates.

If you provide a configuration file during deployment (`chef-automate deploy /path/to/config.toml`), you must specify any proxy settings in that configuration file.

```toml
[global.v1.proxy]
host = "<your proxy host>"
port = <your proxy port>
no_proxy = ["0.0.0.0", "127.0.0.1"]
# user = "<your proxy user>"
# password = "<your proxy password>"
```

To patch the proxy settings, create a TOML file that contains the `[global.v1.proxy]` section and settings.
Then run `chef-automate config patch </path/to/your-file.toml>` to deploy your change.

##### Required Sites and Domains

Chef Automate must be able to access the following:

* `packages.chef.io`
* `licensing.chef.io`
* `raw.githubusercontent.com`
* `api.bintray.com`
* `bldr.habitat.sh`
* `akamai.bintray.com`
* `dl.bintray.com`
* `bintray.com`
* `localhost`
* `127.0.0.1`
* `0.0.0.0`

#### Global Log Level

Configure the log level for all Chef Automate services by creating a TOML file.
By default each service will initialize at the `info` level, but the following settings are available: `debug`, `info`, `warning`, `panic`, or `fatal`.

```toml
[global.v1.log]
level = "debug"
```

Then run `chef-automate config patch </path/to/your-file.toml>` to deploy your change.

#### Sample Configuration

```toml
# This is a default Chef Automate configuration file. You can run
# 'chef-automate deploy' with this config file and it should
# successfully create a new Chef Automate instance with default settings.

[global.v1]
# The external fully qualified domain name.
 # When the application is deployed you should be able to access 'https://<fqdn>/'
  # to login.
  fqdn = "chef-automate.test"

  # The following TLS certificate and RSA public key were
  # automatically generated. The certificate is a self-signed
  # certificate and will likely produce security warnings when you
  # visit Chef Automate in your web browser. We recommend using a
  # certificate signed by a certificate authority you trust.
  [[global.v1.frontend_tls]]
    # The TLS certificate for the load balancer frontend.
    cert = """-----BEGIN CERTIFICATE-----
<the load balancer's certificate>
-----END CERTIFICATE-----
"""

    # The TLS RSA key for the load balancer frontend.
    key = """-----BEGIN RSA PRIVATE KEY-----
<the load balancer's TLS RSA key>
-----END RSA PRIVATE KEY-----
"""

# Deployment service configuration.
[deployment.v1]
  [deployment.v1.svc]
    # Habitat channel to install hartifact from.
    # Can be 'dev', 'current', or 'acceptance'
    channel = "current"
    upgrade_strategy = "at-once"
    deployment_type = "local"

```

### Additional Configuration

#### General Elasticsearch Configuration

To configure Elasticsearch for your Chef Automate installation, create a TOML file that contains the partial configuration below.
Uncomment and change settings as needed, and then run `chef-automate config patch </path/to/your-file.toml>` to deploy your change.

```toml
[elasticsearch.v1.sys.proxy]
# NOTE: The elasticsearch proxy settings are derived from the global proxy settings.
# host = "<proxy host>"
# port = <proxy port>
# user = "<proxy username>"
# password = "<proxy password>"
# no_proxy = <["0.0.0.0", "127.0.0.1"]>
[elasticsearch.v1.sys.cluster]
# name = "chef-insights"
[elasticsearch.v1.sys.cluster.routing.allocation]
# node_concurrent_recoveries = 2
# node_initial_primaries_recoveries = 4
# same_shard_host = false
[elasticsearch.v1.sys.node]
# max_local_storage_nodes = 1
# master = true
# data = true
[elasticsearch.v1.sys.path]
# logs = "logs"
[elasticsearch.v1.sys.indices.recovery]
# max_bytes_per_sec = "20mb"
[elasticsearch.v1.sys.indices.breaker]
# total_limit = "70%"
# fielddata_limit = "60%"
# fielddata_overhead = "1.03"
# request_limit = "40%"
# request_overhead = "1"
[elasticsearch.v1.sys.bootstrap]
# memory_lock = false
[elasticsearch.v1.sys.network]
# host = "0.0.0.0"
# port = 10141
[elasticsearch.v1.sys.transport]
# port = "10142"
[elasticsearch.v1.sys.discovery]
# ping_unicast_hosts = "[]"
# minimum_master_nodes = 1
# zen_fd_ping_timeout = "30s"
[elasticsearch.v1.sys.gateway]
# expected_nodes = 0
# expected_master_nodes = 0
# expected_data_nodes = 0
[elasticsearch.v1.sys.action]
# destructive_requires_name = true
[elasticsearch.v1.sys.logger]
# level = "info"
[elasticsearch.v1.sys.runtime]
# max_locked_memory = "unlimited"
# es_java_opts = ""
# NOTE: see https://www.elastic.co/guide/en/elasticsearch/guide/current/heap-sizing.html
# for important guidance regarding the configuration of the heap size setting
# heapsize = "4g"
```

#### Setting Elasticsearch Heap

The Elasticsearch heap size can, and in most cases should, be set to 50% of the available system
memory. However, you should review and consider the important caveats covered in the [Elasticsearch heap size documentation](https://www.elastic.co/guide/en/elasticsearch/reference/current/heap-size.html).

For example, a system with 32GB of memory can have its Elasticsearch heap
size set to `16g`; to do so, one would first create a TOML file that contains the partial
configuration below, and then run `chef-automate config patch </path/to/your-file.toml>` to
deploy the change.

```toml
[elasticsearch.v1.sys.runtime]
heapsize = "16g"
```

#### PostgreSQL

To configure PostgreSQL for your Chef Automate installation, create a TOML file that contains the partial configuration below.
Uncomment and change settings as needed, with the following caveats:

* These configuration settings affect only the Chef Automate-deployed PostgreSQL database. They do not affect an [externally-deployed PostgreSQL database]({{< relref "install.md#configuring-an-external-postgresql-database" >}}).
* Chef Automate uses TLS mutual authentication to communicate with its PostgreSQL database.

Then run `chef-automate config patch </path/to/your-file.toml>` to deploy your change.

```toml
[postgresql.v1.sys.service]
# host = "0.0.0.0"
# port = 5432
[postgresql.v1.sys.pg]
# md5_auth_cidr_addresses = ["0.0.0.0/0", "::0/0"]
# max_wal_size = "1GB"
# min_wal_size = "80MB"
# wal_keep_segments = 32
# checkpoint_timeout = "5min"
# checkpoint_completion_target = 0.5
# max_connections = 100
# max_locks_per_transaction = 64
[postgresql.v1.sys.logger]
# level = "ERROR"
[postgresql.v1.sys.superuser]
# name = "automate"
```

#### Load Balancer

To configure your Chef Automate installation's load balancer, create a TOML file that contains the partial configuration below.
Uncomment and change settings as needed, and then run `chef-automate config patch </path/to/your-file.toml>` to deploy your change.

```toml
[load_balancer.v1.sys.service]
# https_port = 443
# http_port = 80
# NOTE: the external_fqdn setting is derived from the global settings and
# should be configured via the `[global.v1]` setting.
# external_fqdn = "<your Chef Automate fqdn>"
[load_balancer.v1.sys.log]
# level = "info"
[load_balancer.v1.sys.ngx.main]
# worker_processes = 4
# error_log = "/dev/stderr"
[load_balancer.v1.sys.ngx.events]
# worker_connections = 1024
# worker_processor_method = "epoll"
# multi_accept = "on"
[load_balancer.v1.sys.ngx.http]
# access_log = "/dev/stdout"
# client_max_body_size = "250m"
# default_type = "application/octet-stream"
# keepalive_timeout = 60
# keepalive_requests = 10000
# gzip = "on"
# gzip_comp_level = "2"
# gzip_disable = "MSIE [1-6]\\."
# gzip_http_version = "1.0"
# gzip_min_length = 10240
# gzip_proxied = "expired no-cache no-store private auth"
# gzip_types = "text/plain text/css text/xml text/javascript application/x-javascript application/xml"
# gzip_vary = "on"
# large_client_header_buffers_size = "8k"
# large_client_header_buffers_number = 4
# sendfile = "on"
# ssl_ciphers = "ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305:ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-SHA384:ECDHE-RSA-AES256-SHA384:ECDHE-ECDSA-AES128-SHA256:ECDHE-RSA-AES128-SHA256:AES256-GCM-SHA384:!aNULL:!eNULL:!EXPORT"
# ssl_protocols = "TLSv1.2"
# tcp_nodelay = "on"
# tcp_nopush = "on"
[load_balancer.v1.sys.proxy]
# NOTE: The load_balancer proxy settings are derived from the global proxy settings.
# host = "<proxy host>"
# port = <proxy port>
# user = "<proxy username>"
# password = "<proxy password>"
# no_proxy = <["0.0.0.0", "127.0.0.1"]>
[[load_balancer.v1.sys.frontend_tls]]
# NOTE: the load_balancer TLS certificate settings are derived from global
# settings and should be configured via `[[global.v1.frontend_tls]]` settings
# server_name = "<your Chef Automate server name>"
# cert = "-----BEGIN CERTIFICATE-----\n<your load balancer cert>\n-----END CERTIFICATE-----\n"
# key = "-----BEGIN RSA PRIVATE KEY-----\n<your load balancer private key>\n-----END RSA PRIVATE KEY-----\n"
```

#### Buffer Size

Configure message buffer ingest size:

```toml
[compliance.v1.sys.service]
message_buffer_size = 200
[ingest.v1.sys.service]
message_buffer_size = 200
```

#### Compliance Configuration

To configure your Chef Automate InSpec agent scans, create a TOML file that contains the partial configuration below.
Uncomment and change settings as needed, and then run `chef-automate config patch </path/to/your-file.toml>` to deploy your change.

```toml
[compliance.v1.sys.agent]
## Max number of inspec workers to run in parallel for detect and scan jobs. Default: 10
# workers = 20
## Max number of detect and scan jobs that can be accepted in the jobs workers queue. Default: 1000
# buffer_size = 2000
## Option to specify the version of inspec to use for remote(e.g. AWS SSM) scan jobs
# remote_inspec_version = "4.3.2"
## A control result message that exceeds this character limit will be truncated. Default: 10000
# result_message_limit = 20000
## The array of results per control will be truncated at this limit to avoid large reports that cannot be processed. Default: 50
# control_results_limit = 100
## Control results that have a `run_time` (in seconds) below this limit will be stripped of the `start_time` and `run_time` fields. Default: 1.0
# run_time_limit = 0.5
```

#### Configure Inflight Data Collector Request Maximum

You can specify the maximum number of inflight data collector requests. The default value is sixty times the number of the machine's available CPUs.

```toml
    [gateway.v1.sys.data_collector.limiter]
    # Setting disable to true will allow an unbounded number of
    # data collector requests to remain inflight concurrently.
    disable = false
    # max_inflight_requests will set the maximum number of
    # concurrent inflight data collector requests. By default,
    # this value is 60 * runtime.CpuCount()
    max_inflight_requests = 100
```

#### Sign-out on Browser Close

Configuration to sign out users from Chef Automate when they close the browser.

```toml
[session.v1.sys.service]
  # Setting persistent to false will disable persistent sessions.
  # Users will be signed out when their browser closes.
  persistent = false
```

### Troubleshooting

Common syntax errors may cause issues in configuration files:

* Keys: Names use underscores, not dashes.
* Ports: Use the correct type. Single numbers are integers and don't need quotation marks. Ranges are strings and require quotation marks.
* Whitespace: Both tabs and spaces are whitespace.
* Arrays: Use square brackets with comma-separated entries of the same type.

See the [TOML README](https://github.com/BurntSushi/toml-1) for more details.
