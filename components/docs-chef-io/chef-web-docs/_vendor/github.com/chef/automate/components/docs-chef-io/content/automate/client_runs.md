+++
title = "Client Runs"

date = 2018-03-26T16:01:58-07:00
draft = false
[menu]
  [menu.automate]
    title = "Client Runs"
    parent = "automate"
    identifier = "automate/client_runs.md Client Runs"
    weight = 60
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/client_runs.md)

## Overview

The _Client Runs_ page shows all nodes connected to Chef Automate, either directly or via a Chef Infra Server proxy. 
Nodes appear in this view after a Chef Infra Client run has executed.

## Chef Infra Client Run Status Overview

The Chef Infra Client Run Status chart displays a summary of node statuses: failed, successful, or missing, as well as the total node count.
The chart changes as you select filters.

![Client Runs Overview](/images/automate/client-runs.png)

## Node List Table

The node list table shows all nodes connected to Chef Automate. 
Filter the node list table by selecting any of the status tabs below the **Chef Infra Client Run Status** box.
Sort the nodes listed on the table by selecting the arrows to the right of the column headers: _Node Name_, _Check-In_, _Uptime_, _Platform_, _Environment_ or _Policy Group_.
Selecting an entry in this table will take you to a _Node details_ page with more information about the Chef Infra Client runs associated with this node.

A node may be present in this table without any associated run history.
This situation happens when data retention settings erase the most recent run history for such a node.
In this case, a **no data** icon appears and you will be unable to view any node details.

The node remains listed as a missing node until it is deleted from Automate.
Automate automatically removes any modes deleted from the Chef Infra Server.

## Node Details

The Node Details table displays the most recent converge results.
Find more information about _Resources_, _Run List_, and _Attributes_ in the tabs below the node detail chart.
Select the tabs to switch between these three views.

_Resources_ displays the status of the most recent resources as failed, successful, unchanged, or unprocessed.
Selecting the tabs with these names will filter the list to show only those resources.

_Run List_ shows cookbooks, roles and recipes.
For a node using policyfiles, you will be able to see the policy ID's for each cookbook listed.

_Attributes_ shows an expandable list of node properties.
Use the search bar to discover the node attributes by attribute name, key name, or value.
The search results show by highlighting matching attributes. Use the _default_, _normal_, _override_, and _automatic_ buttons beneath the search bar to filter attributes by to these categories.
Learn more about [attributes](https://docs.chef.io/attributes/).

When looking at a failed Chef Infra Client run, select **View Error Log** at the top of the page to open a window showing the error message and backtrace. 
Use the downloaded button to save the error message.

Selecting a node from the node list table opens the _Node details_ page with the most recent information about that node.

To look at past run data, select **Run History** on the upper right of the page, which opens a side panel containing historical run data. 
You can filter this data by using the Run Status icons and Date Range selections.

Node history data supports up to three months of Chef Infra Client run information.
Scroll through the node history using the pagination buttons at the bottom of the side panel. 
Use the **X** button at the top of the panel to close the side panel.

## Filtering

### Search Bar

Filter nodes from the search bar based on existing node information. You can apply more than one filter to a search.
The node list table changes to display only nodes that match all applied filters.
To apply a filter, first select the filter from the dropdown list and begin typing to display autocomplete options.
To save a search, select **Share** to the right of the search bar, and copy the private URL.

#### Node Filters

[Attribute](https://docs.chef.io/attributes/)
: Search for an attribute key, this will not search across attribute values.

[Chef Organization](https://docs.chef.io/server_orgs/)
: A Chef Infra Server organization name.

[Chef Infra Server](https://docs.chef.io/server_overview/)
: A Chef Infra Server URL.

[Cookbook](https://docs.chef.io/cookbooks/)
: A cookbook name.

[Environment](https://docs.chef.io/environments/)
: Nodes can have one environment.

[Node Name](https://docs.chef.io/nodes/#about-node-names)
: Name of the node.

[Platform](https://docs.chef.io/platforms/#chef-automate-server)
: OS Platform of a node.

[Policy Group](https://docs.chef.io/policyfile/#settings)
: Policy group name, only nodes using policyfiles will appear.

[Policy Name](https://docs.chef.io/policyfile/#settings)
: Name of the policy as set in policyfile.rb, only nodes using policyfiles will appear.

[Policy Revision](https://docs.chef.io/release_notes_server/#policiesnamerevisions)
: The policy revision ID, only nodes using policyfiles will appear.

[Recipe](https://docs.chef.io/recipes/)
: A recipe within a cookbook.

[Resource Name](https://docs.chef.io/resources/)
: A resource within a cookbook.

[Role](https://docs.chef.io/roles/)
: Search by nodes assigned to a role. Nodes can have zero or multiple roles.

See more about [policyfiles](https://docs.chef.io/policyfile/).

## Managing Node Data

### Managing Missing Nodes

Configure the timing for labeling nodes as missing and then deleting them from [Data Lifecycle]({{< relref "data_lifecycle.md" >}}) on the Settings tab.

### Deleting Missing Nodes

Admins and users with the relevant permissions defined in access policies can delete missing nodes from the Chef Infra Client Runs page.
You cannot delete active nodes.

To delete one or more missing nodes, tick the checkbox to the left of the node name, and then select the red **Delete** button above the table header.
Confirm the delete action in the pop-up window.

To delete all missing nodes, tick the checkbox at the top of the Client Runs table, which selects all missing nodes on the current page.
The user can choose to deselect individual nodes by unchecking the checkboxes next to nodes.
Select the delete button and confirm the delete action in the pop-up window.

### Deleting Missing Nodes from the Command Line

Delete nodes using the Chef Automate CLI or through the Chef Automate REST API.

To delete a node from the _Client Runs_ page using the Chef Automate CLI, first locate the `node ID` on the _Node Details_ page, and then use this node ID with the `node-delete` command:

```bash
chef-automate infrastructure node-delete 3f2a2830-0ef3-474a-a835-3a7dd25361fe
```

To delete nodes using the REST API, use the `"https://automate-url/api/v0/ingest/events/chef/nodedelete"` endpoint to delete a single node, or the `"https://automate-url/api/v0/ingest/events/chef/node-multiple-deletes"` endpoint to delete multiple nodes.

Identify your node or nodes with either the _node\_id_ --which is the UUID of the node as it appears in Chef Automate--or the combination of _node name_, _organization name_, and _service hostname_.
The _service hostname_ is the `fqdn` of your Chef Infra Server, or the `localhost` of chef-solo nodes.

#### Request for deleting a node using the _node\_id_

```bash
curl -sSX POST "https://automate-url/api/v0/ingest/events/chef/nodedelete" -d
'{
  "node_id": "3f2a2830-0ef3-474a-a835-3a7dd25361fe"
}'
-H "X-Data-Collector-Token: $TOKEN"
```

#### Request for deleting multiple nodes using the _node\_id_

```bash
curl -sSX POST "https://automate-url/api/v0/ingest/events/chef/node-multiple-deletes" -d
'{
  "node_ids": ["3f2a2830-0ef3-474a-a835-3a7dd25361fe", "9c139ad0-89a5-44bc-942c-d7f248b155ba"]
}'
-H "X-Data-Collector-Token: $TOKEN"
```

#### Request for deleting a node using the _node name_, _organization name_, and _service hostname_

```bash
curl -sSX POST "https://automate-url/api/v0/ingest/events/chef/nodedelete" -d
'{
  "node_name": "somenode",
  "organization_name": "yourorg",
  "service_hostname": "chef-infra-server-fqdn"
}'
-H "X-Data-Collector-Token: $TOKEN"
```

### Managing Ephemeral Nodes

Chef Automate considers the instances of ephemeral nodes, which are nodes that are frequently created and destroyed, as new nodes by default, even if the node indefinitely uses the same name.
Set Chef Automate to consider ephemeral nodes as manifestations of the same node by configuring the UUID on the client side.
Configuring the UUID on the client side keeps the node associated with the same id, which makes Chef Automate consider it as the same node every time it recreates.
In the node's `client.rb`, set `chef_guid` to the _desired UUID_.
If the node already exists, check that it uses the correct UUID, otherwise it will appear as a new node the next time it recreates.

See the `client.rb` documentation for more information about [configuring your client nodes](https://docs.chef.io/config_rb_client/).

The following are the configuration parameters available:

| Parameter | Type | Explanation | Format | Default |
| --------- | ---- | ----------- | ------- | ------ |
|`threshold`|string|The duration after which unreported nodes are marked as missing.|`1h30m`, `1m`, `2h30m`, `1d`, etc.|`1d`|
|`every`|string|How often to scan the nodes to check if they are missing.|`1h30m`, `1m`, `2h30m`, `1d`, etc.|`15m`|
|`running`|boolean|Is the job running? Set to false to turn off missing node functionality.|n/a|`true`|

Below is an example curl command:

```bash
curl -sSX POST "https://automate-url/api/v0/retention/nodes/missing-nodes/config" -d
'{
  "threshold": "1d",
  "every": "15m",
  "running": true
}'
-H "api-token: $TOKEN"
```

You will need an [API token]({{< relref "api_tokens.md#creating-api-tokens" >}}) to send API requests. 

### Configuring Data Cleanup

By default, Chef Automate prevents irreversible destructive operations by keeping deleted node history in Elasticsearch, unless users configure this functionality.
Chef Software recommends setting the `threshold` for destroying deleted node history to 1 day and running data cleanup every 15 minutes.

Available data cleanup configuration parameters:

| Parameter | Type | Explanation | Format | Default |
| --------- | ---- | ----------- | ------- | ------ |
|`threshold`|string|The duration after which nodes marked for deletion are removed.|`1h30m`, `1m`, `2h30m`, `1d`, etc.|`1d`|
|`every`|string|How often to scan for marked nodes for deletion and removal.|`1h30m`, `1m`, `2h30m`, `1d`, etc.|`15m`|
|`running`|boolean|Is the job running, set to true to turn on data cleanup functionality.|n/a|`false`|

Below is an example curl command with the recommended data cleanup settings:

```bash
curl -sSX POST "https://automate-url/api/v0/retention/nodes/delete-nodes/config" -d
'{
  "threshold": "1d",
  "every": "15m",
  "running": true
}'
-H "api-token: $TOKEN"
```

You will need an [API token]({{< relref "api_tokens.md#creating-api-tokens" >}}) to send API requests. 
