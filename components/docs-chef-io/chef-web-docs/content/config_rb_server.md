+++
title = "chef-server.rb Settings"
draft = false

aliases = ["/config_rb_server.html"]

[menu]
  [menu.infra]
    title = "chef-server.rb"
    identifier = "chef_infra/managing_chef_infra_server/config_rb_server.md chef-server.rb"
    parent = "chef_infra/managing_chef_infra_server"
    weight = 170
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/config_rb_server.md)

{{% config_rb_server_summary %}}

## Use Conditions

{{% config_add_condition %}}

## Recommended Settings

{{% server_tuning_general %}}

### NGINX SSL Protocols

{{% server_tuning_nginx %}}

## Optional Settings

The following settings are often used for performance tuning of the Chef
Infra Server in larger installations.

{{< note >}}

{{% notes_config_rb_server_must_reconfigure %}}

{{< /note >}}

{{< note >}}

Review the full list of [optional
settings](/config_rb_server_optional_settings/) that can be added to
the chef-server.rb file. Many of these optional settings should not be
added without first consulting with Chef support.

{{< /note >}}

### bookshelf

{{% server_tuning_bookshelf %}}

{{< warning >}}

{{% notes_server_aws_cookbook_storage %}}

{{< /warning >}}

### opscode-account

The following setting is often modified from the default as part of the
tuning effort for the **opscode-account** service:

`opscode_account['worker_processes']`

:   The number of allowed worker processes. This value should be
    increased if requests made to the **opscode-account** service are
    timing out, but only if the front-end machines have available CPU
    and RAM. Default value: `4`.

### opscode-erchef

{{% server_tuning_erchef %}}

#### Data Collector

The following settings are often modified from the default as part of
the tuning effort for the **data_collector** **opscode-erchef**
application:

`data_collector['http_max_count']`

:   The maximum worker count for the HTTP connection pool that is used
    by the data collector. If failures indicate that **opscode-erchef**
    application has run out of HTTP connections for the
    **data_collector** then increase this value. Default value: <span
    class="title-ref">100</span>.

### opscode-expander

{{% server_tuning_expander %}}

### opscode-solr4

{{% server_tuning_solr %}}

#### Available Memory

{{% server_tuning_solr_available_memory %}}

#### Large Node Sizes

{{% server_tuning_solr_large_node_sizes %}}

#### Update Frequency

{{% server_tuning_solr_update_frequency %}}

### postgresql

{{% server_tuning_postgresql %}}

`postgresql['sslmode']`

:   SSL encryption mode between the Chef Infra Server and PostgreSQL.
    Valid settings are `'disable'` and `'require'`. Default value:
    `'disable'`.
