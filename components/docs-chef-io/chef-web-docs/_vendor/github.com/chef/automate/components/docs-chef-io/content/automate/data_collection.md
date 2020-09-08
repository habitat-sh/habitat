+++
title = "Data Collection"

draft = false
[menu]
  [menu.automate]
    title = "Data Collection"
    parent = "automate/configuring_automate"
    identifier = "automate/configuring_automate/data_collection.md Data Collection"
    weight = 20
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/data_collection.md)

# Audit Cookbook + Inspec + Automate 2 Versions Support Matrix

Refer to the [following Supported Versions list](https://github.com/chef-cookbooks/audit#chef-automate) to confirm a full set of working versions for your chef-client, Audit cookbook, Inspec, and Automate 2. When these do not match up, ingestion problems can occur because the messages will not show up in the expected format.

# Node Run and Audit Data Collection

Nodes can send their run data to Chef Automate.
There are two steps to getting data collection running in Chef Automate:

1. You must first have an API token. You have two options:

    * [Create a new API token]({{< relref "api_tokens.md#creating-api-tokens" >}}) and add the API token to the Ingest policy, preferably at time of creation.
    * Or you can [use your existing data collector token]({{< relref "#existing-data-collector-token-setup" >}}) if you are migrating from Chef Automate 1.

1. Once you have an API token, you can either:
    * [Configure your Chef Infra Server to point to Chef Automate]({{< relref "#configure-your-chef-infra-server-to-send-data-to-chef-automate" >}}).
    **If you are using Chef Infra Server, this is the recommended method of sending data to Chef Automate.**
    * Or, you can have [Chef Infra Client send the data directly to Chef Automate]({{< relref "#configure-your-chef-client-to-send-data-to-chef-automate-without-chef-server" >}}).

## Set Up an Existing Chef Automate 1 Data Collector Token in Chef Automate 2 {#existing-data-collector-token-setup}

### Porting the Existing Chef Automate 1 Data Collector Token to Chef Automate 2

If you are migrating from Chef Automate 1, you probably have already deployed a data collector token on either your Chef Infra Servers or your Chef Infra Clients.
To re-use your existing data collector token from your Chef Automate 1 installation, you need to perform the configuration change outlined here.

For this process, you need the existing token (let's call it `A1_DC_TOKEN`), and access to the machine running the `chef-automate` CLI client.

Create a file (in this example, `data-collector-token.toml`) containing your existing token:

```toml
[auth_n.v1.sys.service]
a1_data_collector_token = "<A1_DC_TOKEN>"
```

Now apply that configuration to your Chef Automate 2 deployment:

```bash
# chef-automate config patch data-collector-token.toml

[...output omitted...]

Success: Configuration patched
```

The system will notice that configuration change after a short interval.
From that point on, requests using the `x-data-collector-token: <A1_DC_TOKEN>` header will be accepted.
When logged in with admin permissions, you will also find your added token in
`https://{{< example_fqdn "automate" >}}/admin/tokens`, under the name

> Legacy data collector token ported from A1

Now that you have a valid API token, you'll need to
[update your Chef Infra Server data collector configuration]({{< relref "#configure-your-chef-infra-server-to-send-data-to-chef-automate" >}})
if you are using a Chef Infra Server. Otherwise, you must
[configure your Chef Infra Clients to send data directly to Chef Automate]({{< relref "#configure-your-chef-client-to-send-data-to-chef-automate-without-chef-server" >}}).

## Configure your Chef Infra Server to Send Data to Chef Automate

{{< note >}} Multiple Chef Infra Servers can send data to a single Chef Automate server. {{< /note >}}

In addition to forwarding Chef run data to Chef Automate, Chef Infra Server will send messages to Chef Automate whenever an action is taken on a Chef Infra Server object, such as when a cookbook is uploaded to the Chef Infra Server or when a user edits a role.

In order to have Chef Infra Server send run data from connected Chef Infra Clients, set the data collection proxy attribute to `true`.

### Setting Up Data Collection on Chef Infra Server Versions 12.14 and Higher

Instead of setting the token directly in `/etc/opscode/chef-server.rb` as was done in older versions of the Chef Infra Server, we'll use the `set-secret` command, so that your API token does not live in
plaintext in a file:

```shell
sudo chef-server-ctl set-secret data_collector token '<API_TOKEN_FROM_STEP_1>'
sudo chef-server-ctl restart nginx
sudo chef-server-ctl restart opscode-erchef
```

Next, configure the Chef Infra Server for data collection forwarding by adding the following setting to
`/etc/opscode/chef-server.rb`:

```ruby
data_collector['root_url'] = 'https://{{< example_fqdn "automate" >}}/data-collector/v0/'
# Add for Chef Infra Client run forwarding
data_collector['proxy'] = true
# Add for compliance scanning
profiles['root_url'] = 'https://{{< example_fqdn "automate" >}}'
# Save and close the file
```

To apply the changes, run:

```shell
sudo chef-server-ctl reconfigure
```

### Setting Up Data Collection on Chef Infra Server for Versions 12.11 through 12.13

For Chef Infra Server versions between 12.11 and 12.13, simply add the `root_url` and `token` values in
`/etc/opscode/chef-server.rb`:

```ruby
data_collector['root_url'] = 'https://{{< example_fqdn "automate" >}}/data-collector/v0/'
data_collector['token'] = '<API_TOKEN_FROM_STEP_1>'
# Add for Chef Infra Client run forwarding
data_collector['proxy']= true
# Add for compliance scanning
profiles['root_url'] = 'https://{{< example_fqdn "automate" >}}'
# Save and close the file
```

To apply the changes, run:

```shell
chef-server-ctl reconfigure
```

### Setting Up Chef Infra Client to Send Compliance Scan Data Through the Chef Infra Server to Chef Automate

Now that the Chef Infra Server is configured for data collection, you can also enable Compliance Scanning

on your Chef Infra Clients via the [Audit Cookbook](https://github.com/chef-cookbooks/audit).

* Set the following attributes for the audit cookbook:

```ruby
default['audit']['reporter'] = 'chef-server-automate'
default['audit']['fetcher'] = 'chef-server'
default['audit']['profiles'].push(
  'name': 'cis-centos7-level2',
  'compliance': 'user-name/cis-centos7-level2' # in the ui for automate, this value is the identifier for the profile
)
default['audit']['interval'] = {
  'enabled': true
  'time': 1440  # once a day, the default value
}
```

Now, any node with `audit::default` its runlist will fetch and report data to and from Chef Automate
via the Chef Infra Server. Please see the audit cookbook for an
[exhaustive list of configuration options](https://github.com/chef-cookbooks/audit).

### Additional Chef Infra Server Data Collection Configuration Options

| Option | Description | Default |
| --- | --- | --- |
|`data_collector['proxy']`|If set to true, Chef Infra Server will proxy all requests sent to /data-collector to the configured Chef Automate `data_collector['root_url']`. Note that this route does not check the request signature and add the right data_collector token, but just proxies the Chef Automate endpoint as-is.|Default: `nil`|
`data_collector['timeout']`|Timeout in milliseconds to abort an attempt to send a message to the Chef Automate server.| Default: `30000`|
 `data_collector['http_init_count']`|Number of Chef Automate HTTP workers Chef Infra Server should start.|Default: `25`|
|`data_collector['http_max_count']`|Maximum number of Chef Automate HTTP workers Chef Infra Server should allow to exist at any time.|Default: `100`|
|`data_collector['http_max_age']`|Maximum age a Chef Automate HTTP worker should be allowed to live, specified as an Erlang tuple.|Default: `{70, sec}`|
|`data_collector['http_cull_interval']`|How often Chef Infra Server should cull aged-out Chef Automate HTTP workers that have exceeded their `http_max_age`, specified as an Erlang tuple.|Default: `{1, min}`|
|`data_collector['http_max_connection_duration']`|Maximum duration an HTTP connection is allowed to exist before it is terminated, specified as an Erlang tuple.|Default: `{70, sec}`|

<!-- Hiding external Elasticsearch: Consult Irving's team for solution  -->
<!-- ## Use an external Elasticsearch cluster (optional)

Chef Automate uses Elasticsearch to store its data, and the default Chef Automate install includes a single Elasticsearch service.
This is sufficient to run production workloads; however, for greater data retention, we recommend using a multi-node Elasticsearch cluster with replication and sharding to store and protect your data.

As of Chef Automate 1.7.114, the compliance service uses a ``compliance-latest`` Elasticsearch index to improves the performance of the reporting APIs at scale. Chef Automate creates this index automatically as part of the upgrade to Chef Automate 1.7.114. The index is updated with each new compliance report. If the ``compliance-latest`` Elasticsearch index becomes out of sync with the time-series data, it can be regenerated using the ``workflow-ctl migrate-compliance`` subcommand. For more information, see [migrate-compliance ](https://docs.chef.io/ctl_automate_server/#migrate-compliance).

### Prerequisites

* Chef Automate server
* Elasticsearch (version 2.4.1 or greater; version 5.x is required for Chef Automate 1.6 and above)

### Elasticsearch configuration

To utilize an external Elasticsearch installation, set the following configuration option in your
``/etc/delivery/delivery.rb``:

```ruby
elasticsearch['urls'] = ['https://my-elasticsearch-cluster.mycompany.com']
```

Or for a three node on premise install

```ruby
elasticsearch['urls'] = ['http://172.16.0.100:9200','http://172.16.0.101:9200','http://172.16.0.100:9202']
```

The ``elasticsearch['urls']`` attribute should be an array of Elasticsearch nodes over
which Chef Automate will round-robin requests. You can also supply a single entry which corresponds to
a load-balancer or a third-party Elasticsearch-as-a-service offering.

After saving the file, run ``sudo workflow-ctl reconfigure``.

An additional Elasticsearch option is ``elasticsearch['host_header']``. This is the HTTP ``Host`` header to send with the request.
When this attribute is unspecified, the default behavior is as follows:

 * If the ``urls`` parameter contains a single entry, the host of the supplied URI will be sent as the Host header.
 * If the ``urls`` parameter contains more than one entry, no Host header will be  sent.

When this attribute is specified, the supplied string will be sent as the ``Host`` header on all requests. This may be required for some third-party Elasticsearch offerings. -->

## Configure your Chef Infra Client to Send Data to Chef Automate without Chef Infra Server

If you do not use a Chef Infra Server in your environment (if you only use `chef-solo`, for example), you

can configure your Chef Infra Clients to send their run data to Chef Automate directly by performing the following:

1. Add Chef Automate SSL certificate to `trusted_certs` directory.

2. Configure Chef Infra Client to use the Data Collector endpoint and API token in Chef Automate.

### Add Chef Automate certificate to `trusted_certs` directory

**Note:** This step only applies to self-signed SSL certificates. If you are using an SSL certificate
signed by a valid certificate authority, you may skip this step.

Chef requires that the self-signed Chef Automate SSL certificate
(`HOSTNAME.crt`) is located in the `/etc/chef/trusted_certs` directory
on any node that wants to send data to Chef Automate. This directory is
the location into which SSL certificates are placed when a node has been
bootstrapped with chef-client.

To fetch the certificate onto your workstation, use `knife ssl fetch`
and pass in the URL of the Chef Automate server. You can then use
utilities such as `scp` or `rsync` to copy the downloaded cert files
from your `.chef/trusted_certs` directory to the
`/etc/chef/trusted_certs` directory on the nodes in your infrastructure
that will be sending data directly to the Chef Automate
server.

### Configure Chef Infra Client to Use the Data Collector Endpoint in Chef Automate

{{< warning >}} Chef version 12.12.15 or greater is required. {{< /warning >}}

The data collector functionality is used by the Chef Infra Client to send node
and converge data to Chef Automate. This feature works for Chef Infra Client, as well as both the default
and legacy modes of `chef-solo`.

To send node, converge, and compliance data to Chef Automate, modify
your Chef config (that is `client.rb`, `solo.rb`, or add an additional
config file in an appropriate directory, such as `client.d`) to contain
the following configuration:

```ruby
data_collector.server_url "https://{{< example_fqdn "automate" >}}/data-collector/v0/"
data_collector.token '<API_TOKEN_FROM_STEP_1>'
```

### Setting Up Chef Infra Client to Send Compliance Scan Data Directly to Chef Automate

Now that the Chef Infra Client is configured for data collection, you can also enable Compliance Scanning
on via the [Audit Cookbook](https://github.com/chef-cookbooks/audit).

* Set the following attributes for the audit cookbook:

```ruby
default['audit']['reporter'] = 'chef-automate'
default['audit']['fetcher'] = 'chef-automate'
default['audit']['token'] = '<API_TOKEN_FROM_STEP_1>'
default['audit']['profiles'].push(
  'name': 'cis-centos7-level2',
  'compliance': 'user-name/cis-centos7-level2' # in the ui for automate, this value is the identifier for the profile
)
default['audit']['interval'] = {
  'enabled': true
  'time': 1440  # once a day, the default value
}
```

Now, any node with `audit::default` its runlist will fetch and report data directly to and from
Chef Automate. Please see the audit cookbook for an
[exhaustive list of configuration options](https://github.com/chef-cookbooks/audit).

#### Additional Chef Infra Client Data Collection Configuration Options

| Configuration                     | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | Options                        | Default |
| --------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------ | ------- |
| `data_collector.mode`             | The mode in which the data collector is allowed to operate. This can be used to run data collector only when running as Chef solo but not when using Chef Infra Client.                                                                                                                                                                                                                                                                                                                                         | `:solo`, `:client`, or `:both` | `:both` |
| `data_collector.raise_on_failure` | When the data collector cannot send the "starting a run" message to the data collector server, the data collector will be disabled for that run. In some situations, such as highly-regulated environments, it may be more reasonable to Prevents data collection when the data collector cannot send the "starting a run" message to the data collector server. In these situations, setting this value to `true` will cause the Chef run to raise an exception before starting any converge activities. | `true`, `false`                | `false` |
| `data_collector.organization`     | A user-supplied organization string that can be sent in payloads generated by the data collector when Chef is run in Solo mode. This allows users to associate their Solo nodes with faux organizations without the nodes being connected to an actual Chef Infra Server.                                                                                                                                                                                                                                       | `string`                       | `none`  |

## Troubleshooting: My Data Does Not Show Up in the User Interface

Organizations without associated nodes will not show up on the Chef Automate _Nodes_ page. A node
is not associated with automate until a Chef Infra Client run has completed. This is also true for roles,
cookbooks, recipes, attributes, resources, node names, and environments but does not highlight them
in the UI. This is designed to keep the UI focused on the nodes in your cluster.
