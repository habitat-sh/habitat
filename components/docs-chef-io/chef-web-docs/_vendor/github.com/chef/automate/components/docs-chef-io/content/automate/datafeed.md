+++
title = "Data Feeds"
date = 2020-05-05T13:19:02-07:00
draft = false

[menu]
  [menu.automate]
    title = "Data Feeds"
    parent = "automate/settings"
    identifier = "automate/settings/datafeed.md Data Feeds"
    weight = 20
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/datafeed.md)

{{< note >}}
Data Feed is a beta feature in active development. To enable Data Feed, first select anywhere on the Chef Automate interface and enter 'feat' to open the feature flags window and then toggle "Chef Automate Data Feed" to the "ON" position.
{{< /note >}}

The Data Feed service sends node data to a 3rd party service.
This can be useful when updating configuration management databases, external security dashboards and IT service management platforms.
The following types of information are sent:

- Ohai data gathered from each managed node - This data includes hardware, operating system, and installed program information. Some variation depends on the managed operating system
- Configuration information about each managed node - This information includes Chef Client Run status, Runlists, Cookbooks, and Recipes being ran against each node
- Compliance information about each node that shows the compliance state - This information includes passed and failed controls for each profile executed against that node

A Data Feed operates by doing the following:

- Every 4 hours, the data-feed-service will aggregate the client runs and compliance reports from the previous 4 hours and send this information to the registered destinations. This time interval is 4 hours by default, but is configurable
- If there are no destinations, aggregation will not occur
- The data aggregates and sends in batches of 50 nodes at a time. The batch amount is 50 by default, but is configurable

By default, only Admin users of Chef Automate may create and manage Data Feeds.

## Adding a Data Feed Instance

A single Data Feed instance connects to one 3rd party endpoint.
Create as many Data Feed instances as needed.

To add a Data Feed instance in Chef Automate:

![Setup Data Feed Page](/images/automate/filled_form_create_data_feed.png)

1. In the **Settings** tab, navigate to _Data Feeds_ in the sidebar
1. Select **Create Data Feed**
1. Enter a unique Data Feed name
1. Enter the URL for your Data Feed endpoint, including any specific port details
1. Enter the Username and Password that your 3rd party endpoint requires for authentication
1. Select **Test Data Feed** to begin validating the connection details
1. Once the test is successful, select **Create Data Feed** to save your Data Feed configuration

## Edit a Data Feed Instance

To edit a Data Feed instance in Chef Automate:

1. In _Data Feeds_, select the Data Feed name to open its detail page
1. Edit the Data Feed name or URL
1. Use the **Save** button to save your changes

## Delete a Data Feed Instance

To delete a Data Feed instance in Chef Automate:

1. In _Data Feeds_, select **Delete Data Feed** from the menu at the end of the table row
1. Select **Delete Data Feed** to confirm permanent deletion of this Data Feed

## Configuring Global Data Feed Behavior

{{< note >}}
The Data Feed configuration settings apply across all configured Data Feed instances.
{{< /note >}}

To modify Data Feed behavior with the available configuration settings:

1. Create a configuration patch file to update the configuration settings. Save this file in the `.toml` file format and name your file as desired. For example, `data-feed-patch.toml`

1. Include one or more configuration settings and their updated value(s) in your configuration patch `.toml` file to reflect the desired global Data Feed behavior:

  - Use the `feed_interval` setting to change the interval for the Data Feed collection. The default value is four hours
  - Use the `node_batch_size` setting to change the number of sets of node data sent in each individual batch to your endpoint. The default value is 50 nodes
  - Use the `updated_nodes_only` setting to determine what data to include in each export. The default setting is `true`, which causes the aggregation of only the *changed* data of updated nodes since the last export. Set `updated_nodes_only` to `false` and it aggregates *all* data of updated nodes since the last export
  - To reduce the IP address range for the collected and processed node data, update the `disable_cidr_filter` setting to `false` **and** update the `cidr_filter` setting to cover the required IP address range. For example, you may wish to send only production or test node traffic
  - Use the `accepted_status_codes` setting to define an array of HTTP status codes that the Data Feed Service will treat as `success` if returned by the 3rd party endpoint. If the status code is not in the `accepted_status_codes` list, then an error will be logged

1. Save your configuration patch file changes before continuing to the next step.

1. Apply your configuration changes with the Chef Automate command-line tool:

```bash
    chef-automate config patch data-feed-patch.toml
```

    where `data-feed-patch.toml` is this example's configuration patch file.

### Configuration Patch File Example

```toml
[data_feed_service.v1.sys]
  [data_feed_service.v1.sys.service]
        feed_interval = "4h"
        node_batch_size = 50
        updated_nodes_only = true
        disable_cidr_filter = true
        cidr_filter = "0.0.0.0/0"
        accepted_status_codes = [200, 201, 202, 203, 204]
      [data_feed_service.v1.sys.log]
        level = "info"
```

To debug any issues with the Data Feed Service in Chef Automate, update the following section in your configuration patch file by changing the `log_level` value to "debug":

```toml
    [data_feed_service.v1.sys.log]
    log_level = "debug"
```

## Data Feed Output Syntax and Details

The outputted data from Data Feed consists of line-separated JSON strings.
Each line represents the data for one node, and contains the following properties:

```json
    {
    "attributes": {
     "node_id": "",
     "name": "",
     "run_list": [],
     "chef_environment": "",
     "normal": {},
     "default": {},
     "override":{},
     "automatic":{},
     "normal_value_count": 0,
     "default_value_count": 1,
     "override_value_count": 1,
     "all_value_count": 10,
     "automatic_value_count": 8
    },
    "report": { ... },
    "client_run": { ... },
    "node": {
     "automate_fqdn": "",
     "ip_address" : "",
     "mac_address": "",
     "description":"",
     "serial_number":"",
     "os_service_pack":""
     }
    }
```
