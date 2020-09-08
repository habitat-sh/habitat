+++
title = "Configure Data Collection"
draft = false
robots = "noindex"


aliases = ["/data_collection.html", "/ingest_data_chef_automate.html", "/release/automate/ingest_data_chef_automate.html", "/release/automate/setup_visibility_chef_automate.html", "/setup_visibility_chef_automate.html"]

[menu]
  [menu.legacy]
    title = "Configure Data Collection"
    identifier = "legacy/workflow/workflow_basics/data_collection.md Configure Data Collection"
    parent = "legacy/workflow/workflow_basics"
    weight = 40
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/data_collection.md)



{{% chef_automate_mark %}}

{{% EOL_a1 %}}

## Automatic Node Run Data Collection with Chef Infra Server

{{< note >}}

Requires Chef Client 12.16.42 or greater, and Chef Server 12.11.0 or
greater.

{{< /note >}}

Nodes can send their run data to Chef Automate through the Chef Infra
Server automatically. To enable this functionality, you must perform the
following steps:

-   [Configure a Data Collector token in Chef
    Automate](/data_collection/#step-1-configure-a-data-collector-token-in-chef-automate)
-   [Configure your Chef Infra Server to point to Chef
    Automate](/data_collection/#step-2-configure-your-chef-server-to-point-to-chef-automate)

Multiple Chef Servers can send data to a single Chef Automate server.

### Step 1: Configure a Data Collector token in Chef Automate

All messages sent to Chef Automate are performed over HTTP and are
authenticated with a pre-shared key called a `token`. Every Chef
Automate installation configures a token by default, but we strongly
recommend that you create your own.

{{< note >}}

The Data Collector token has no minimum or maximum character length
restrictions. While the UTF-8 character set is supported,
[US-ASCII](http://www.columbia.edu/kermit/ascii.html) is recommended for
best results.

{{< /note >}}

To set your own token, add the following to your
`/etc/delivery/delivery.rb` file:

``` ruby
data_collector['token'] = 'sometokenvalue'
# Save and close the file
```

To apply the changes, run:

``` shell
sudo automate-ctl reconfigure
```

If you do not configure a token, the default token value is:
`93a49a4f2482c64126f7b6015e6b0f30284287ee4054ff8807fb63d9cbd1c506`

### Step 2: Configure your Chef Infra Server to point to Chef Automate

In addition to forwarding Chef run data to Automate, Chef Infra Server
will send messages to Chef Automate whenever an action is taken on a
Chef Infra Server object, such as when a cookbook is uploaded to the
Chef Infra Server or when a user edits a role.

{{< warning >}}

If running Chef Client releases prior to Chef Client 14, please disable
the Ohai Passwd and Sessions plugins on your nodes in
`/etc/chef/client.rb` or using the Chef Infra Client cookbook to keep
the data sent to your Automate system to a minimum. This improves search
performance and reduces disk space requirements.

{{< /warning >}}

``` shell
ohai.disabled_plugins = [ :Passwd, :Sessions ]
```

[Ohai Plugin Detail](/ohai/#ohai-settings-in-client-rb)

#### Setting up data collection on Chef Server versions 12.14 and higher

Channel the token setting through the veil secrets library because the
token is considered a secret, and cannot appear in
`/etc/opscode/chef-server.rb`:

``` shell
sudo chef-server-ctl set-secret data_collector token 'TOKEN'
sudo chef-server-ctl restart nginx
sudo chef-server-ctl restart opscode-erchef
```

Then add the following setting to `/etc/opscode/chef-server.rb` on the
Chef Infra Server:

``` ruby
data_collector['root_url'] = 'https://my-automate-server.mycompany.com/data-collector/v0/'
# Add for compliance scanning
profiles['root_url'] = 'https://my-automate-server.mycompany.com'
# Save and close the file
```

To apply the changes, run:

``` ruby
chef-server-ctl reconfigure
```

where `my-automate-server.mycompany.com` is the fully-qualified domain
name of your Chef Automate server.

#### Setting up data collection on Chef Server versions 12.13 and lower

On versions 12.13 and prior, simply add the `'root_url'` and `token`
values in `/etc/opscode/chef-server.rb`:

``` ruby
data_collector['root_url'] = 'https://my-automate-server.mycompany.com/data-collector/v0/'
data_collector['token'] = 'TOKEN'
# Add for compliance scanning
profiles['root_url'] = 'https://my-automate-server.mycompany.com'
# Save and close the file
```

To apply the changes, run:

``` ruby
chef-server-ctl reconfigure
```

where `my-automate-server.mycompany.com` is the fully-qualified domain
name of your Chef Automate server, and `TOKEN` is either the default
value or the token value you configured in the [prior
section](#configure-a-data-collector-token-in-chef-automate).

#### Additional options

<table style="width:100%;">
<colgroup>
<col style="width: 14%" />
<col style="width: 57%" />
<col style="width: 28%" />
</colgroup>
<thead>
<tr class="header">
<th>Option</th>
<th>Description</th>
<th>Default</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>data_collector['timeout']</code></td>
<td>Timeout in milliseconds to abort an attempt to send a message to the Chef Automate server.</td>
<td>Default: <code>30000</code>.</td>
</tr>
<tr class="even">
<td><code>data_collector['http_init_count']</code></td>
<td>Number of Chef Automate HTTP workers Chef Infra Server should start.</td>
<td>Default: <code>25</code>.</td>
</tr>
<tr class="odd">
<td><code>data_collector['http_max_count']</code></td>
<td>Maximum number of Chef Automate HTTP workers Chef Infra Server should allow to exist at any time.</td>
<td>Default: <code>100</code>.</td>
</tr>
<tr class="even">
<td><code>data_collector['http_max_age']</code></td>
<td>Maximum age a Chef Automate HTTP worker should be allowed to live, specified as an Erlang tuple.</td>
<td>Default: <code>{70, sec}</code>.</td>
</tr>
<tr class="odd">
<td><code>data_collector['http_cull_interval']</code></td>
<td>How often Chef Infra Server should cull aged-out Chef Automate HTTP workers that have exceeded their <code>http_max_age</code>, specified as an Erlang tuple.</td>
<td>Default: <code>{1, min}</code>.</td>
</tr>
<tr class="even">
<td><code>data_collector['http_max_connection_duration']</code></td>
<td>Maximum duration an HTTP connection is allowed to exist before it is terminated, specified as an Erlang tuple.</td>
<td>Default: <code>{70, sec}</code>.</td>
</tr>
</tbody>
</table>

## Use an external Elasticsearch cluster (optional)

Chef Automate uses Elasticsearch to store its data, and the default Chef
Automate install includes a single Elasticsearch service. This is
sufficient to run production workloads; however, for greater data
retention, we recommend using a multi-node Elasticsearch cluster with
replication and sharding to store and protect your data.

As of Automate 1.7.114, the compliance service uses a
`compliance-latest` Elasticsearch index to improves the performance of
the reporting APIs at scale. Automate creates this index automatically
as part of the upgrade to Automate 1.7.114. The index is updated with
each new compliance report. If the `compliance-latest` Elasticsearch
index becomes out of sync with the time-series data, it can be
regenerated using the `automate-ctl migrate-compliance` subcommand. For
more information, see
[migrate-compliance](/ctl_automate_server/#migrate-compliance).

### Prerequisites

-   Chef Automate server
-   Elasticsearch (version 2.4.1 or greater; version 5.x is required for
    Chef Automate 1.6 and above)

### Elasticsearch configuration

To utilize an external Elasticsearch installation, set the following
configuration option in your `/etc/delivery/delivery.rb`:

``` ruby
elasticsearch['urls'] = ['https://my-elasticsearch-cluster.mycompany.com']
```

Or for a three node on premise install

``` ruby
elasticsearch['urls'] = ['http://172.16.0.100:9200', 'http://172.16.0.101:9200', 'http://172.16.0.100:9202']
```

The `elasticsearch['urls']` attribute should be an array of
Elasticsearch nodes over which Chef Automate will round-robin requests.
You can also supply a single entry which corresponds to a load-balancer
or a third-party Elasticsearch-as-a-service offering.

After saving the file, run `sudo automate-ctl reconfigure`.

An additional Elasticsearch option is `elasticsearch['host_header']`.
This is the HTTP `Host` header to send with the request. When this
attribute is unspecified, the default behavior is as follows:

> -   If the `urls` parameter contains a single entry, the host of the
>     supplied URI will be sent as the Host header.
> -   If the `urls` parameter contains more than one entry, no Host
>     header will be sent.

When this attribute **is** specified, the supplied string will be sent
as the `Host` header on all requests. This may be required for some
third-party Elasticsearch offerings.

## Troubleshooting: My data does not show up in the UI

{{% chef_automate_visibility_no_data_troubleshoot %}}

## Next Steps

-   [Perform a Compliance Scan](/perform_compliance_scan/)
-   [Data Collection with a Chef HA Cluster](/data_collection_ha/)
-   [Data Collection without Chef Infra
    Server](/data_collection_without_server/)
