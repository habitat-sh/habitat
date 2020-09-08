+++
title = "Chef Push Jobs"
draft = false

aliases = ["/push_jobs.html"]

[menu]
  [menu.infra]
    title = "Push Jobs"
    identifier = "chef_infra/features/push_jobs.md Push Jobs"
    parent = "chef_infra/features"
    weight = 60
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/push_jobs.md)

{{% push_jobs_summary %}}

Install [Push Jobs](/install_push_jobs/) using the **push-jobs**
cookbook and a Chef Infra Client run on each of the target nodes.

## Requirements

Chef Push Jobs has the following requirements:

-   An on-premises Chef Infra Server. Hosted Chef does not support Chef
    Push Jobs.
-   The Chef Push Jobs client can be configured using a **push-jobs**
    cookbook, but Chef Infra Client must also be present on the node.
    Only Chef Infra Client can use a cookbook to configure a node.
-   {{% server_firewalls_and_ports_push_jobs %}}

## Components

Chef Push Jobs has three main components: jobs (managed by the Chef Push
Jobs server), a client that is installed on every node in the
organization, and one (or more) workstations from which job messages are
initiated.

All communication between these components is done with the following:

-   A heartbeat message between the Chef Push Jobs server and each
    managed node
-   A knife plugin named `knife push jobs` with four subcommands:
    `job list`, `job start`, `job status`, and `node status`
-   Various job messages sent from a workstation to the Chef Push Jobs
    server
-   A single job message that is sent (per job) from the Chef Push Jobs
    server to one (or more) nodes that are being managed by the Chef
    server

The following diagram shows the various components of Chef Push Jobs:

![image](/images/overview_push_jobs_states.png)

### Jobs

The Chef Push Jobs server is used to send job messages to one (or more)
managed nodes and also to manage the list of jobs that are available to
be run against nodes.

A heartbeat message is used to let all of the nodes in an organization
know that the Chef Push Jobs server is available. The Chef Push Jobs
server listens for heartbeat messages from each Chef Push Jobs client.
If there is no heartbeat from a Chef Push Jobs client, the Chef Push
Jobs server will mark that node as unavailable for job messages until
the heartbeat resumes.

### Nodes

The Chef Push Jobs client is used to receive job messages from the Chef
Push Jobs server and to verify the heartbeat status. The Chef Push Jobs
client uses the same authorization / authentication model as Chef Infra
Client. The Chef Push Jobs client listens for heartbeat messages from
the Chef Push Jobs server. If there is no heartbeat from the Chef Push
Jobs server, the Chef Push Jobs client will finish its current job, but
then stop accepting any new jobs until the heartbeat from the Chef Push
Jobs server resumes.

### Workstations

A workstation is used to manage Chef Push Jobs jobs, including
maintaining the **push-jobs** cookbook, using knife to start and stop
jobs, view job status, and to manage job lists.

## **push-jobs** Cookbook

The **push-jobs** cookbook contains attributes that are used to
configure the Chef Push Jobs client. In addition, Chef Push Jobs relies
on the `whitelist` attribute to manage the list of jobs (and commands)
that are available to Chef Push Jobs.

### Whitelist

A whitelist is a list of jobs and commands that are used by Chef Push
Jobs. A whitelist is saved as an attribute in the **push-jobs**
cookbook. For example:

``` ruby
default['push_jobs']['whitelist'] = {
  'job_name' => 'command',
}
```

The whitelist is accessed from a recipe using the
`node['push_jobs']['whitelist']` attribute. For example:

``` ruby
template 'name' do
  source 'name'
  ...
  variables(:whitelist => node['push_jobs']['whitelist'])
end
```

Use the `knife exec` subcommand to add a job to the whitelist. For
example:

``` bash
knife exec -E 'nodes.transform("name:A_NODE_NAME") do |n|
    n.set["push_jobs"]["whitelist"]["ntpdate"] = "ntpdate -u time"
  end'
```

where `["ntpdate"] = "ntpdate -u time"` is added to the whitelist:

``` ruby
default['push_jobs']['whitelist'] = {
  "ntpdate" => "ntpdate -u time",
}
```

## Reference

The following sections describe the knife subcommands, the Push Jobs
API, and configuration settings used by Chef Push Jobs.

## knife push jobs

{{% plugin_knife_push_jobs_summary %}}

{{< note >}}

Review the list of [common options](/workstation/knife_options/) available to
this (and all) knife subcommands and plugins.

{{< /note >}}

### job list

{{% plugin_knife_push_jobs_job_list %}}

#### Syntax

{{% plugin_knife_push_jobs_job_list_syntax %}}

#### Options

This command does not have any specific options.

### job start

{{% plugin_knife_push_jobs_job_start %}}

#### Syntax

{{% plugin_knife_push_jobs_job_start_syntax %}}

#### Options

This argument has the following options:

`--timeout TIMEOUT`

:   The maximum amount of time (in seconds) by which a job must
    complete, before it is stopped.

`-q QUORUM`, `--quorum QUORUM`

:   The minimum number of nodes that match the search criteria, are
    available, and acknowledge the job request. This can be expressed as
    a percentage (e.g. `50%`) or as an absolute number of nodes (e.g.
    `145`). Default value: `100%`.

    For example, there are ten total nodes. If `--quorum 80%` is used
    and eight of those nodes acknowledge the job request, the command
    will be run against all of the available nodes. If two of the nodes
    were unavailable, the command would still be run against the
    remaining eight available nodes because quorum was met.

#### Examples

**Run a job**

{{% plugin_knife_push_jobs_job_start_run_job %}}

**Run a job using quorum percentage**

{{% plugin_knife_push_jobs_job_start_search_by_quorum %}}

**Run a job using node names**

{{% plugin_knife_push_jobs_job_start_search_by_nodes %}}

### job status

{{% plugin_knife_push_jobs_job_status %}}

#### Syntax

{{% plugin_knife_push_jobs_job_status_syntax %}}

#### Options

This command does not have any specific options.

#### Examples

**View job status by job identifier**

{{% plugin_knife_push_jobs_job_status_by_id %}}

### node status

{{% plugin_knife_push_jobs_node_status %}}

#### Syntax

{{% plugin_knife_push_jobs_node_status_syntax %}}

#### Options

This command does not have any specific options.

## Push Jobs API

The Push Jobs API is used to create jobs and retrieve status using Chef
Push Jobs, a tool that pushes jobs against a set of nodes in the
organization. All requests are signed using the Chef Infra Server API
and the validation key on the workstation from which the requests are
made. All commands are sent to the Chef Infra Server using the
`knife exec` subcommand.

Each authentication request must include
`/organizations/organization_name/pushy/` as part of the name for the
endpoint. For example: `/organizations/organization_name/pushy/jobs/ID`
or `/organizations/organization_name/pushy/node_states`.

### connect/NODE_NAME

{{% api_push_jobs_endpoint_node_name %}}

#### GET

{{% api_push_jobs_endpoint_node_name_get %}}

### jobs

{{% api_push_jobs_endpoint_jobs %}}

#### GET

{{% api_push_jobs_endpoint_jobs_get %}}

#### POST

The `POST` method is used to start a job.

This method has no parameters.

**Request**

``` xml
POST /organizations/ORG_NAME/pushy/jobs
```

with a request body similar to:

``` javascript
{
  "command": "chef-client",
  "run_timeout": 300,
  "nodes": ["NODE1", "NODE2", "NODE3", "NODE4", "NODE5", "NODE6"]
}
```

**Response**

The response is similar to:

``` javascript
{
  "id": "aaaaaaaaaaaa25fd67fa8715fd547d3d"
}
```

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>201</code></td>
<td>Created. The object was created.</td>
</tr>
<tr class="even">
<td><code>400</code></td>
<td>Bad request. The contents of the request are not formatted correctly.</td>
</tr>
<tr class="odd">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="even">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="odd">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

### jobs/ID

{{% api_push_jobs_endpoint_jobs_id %}}

#### GET

The `GET` method is used to get the status of an individual job,
including node state (running, complete, crashed).

This method has no parameters.

The `POST` method is used to start a job.

This method has no parameters.

**Request**

``` xml
POST /organizations/ORG_NAME/pushy/jobs
```

with a request body similar to:

``` javascript
{
  "command": "chef-client",
  "run_timeout": 300,
  "nodes": ["NODE1", "NODE2", "NODE3", "NODE4", "NODE5", "NODE6"]
}
```

**Response**

The response is similar to:

``` javascript
{
  "id": "aaaaaaaaaaaa25fd67fa8715fd547d3d"
}
```

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>201</code></td>
<td>Created. The object was created.</td>
</tr>
<tr class="even">
<td><code>400</code></td>
<td>Bad request. The contents of the request are not formatted correctly.</td>
</tr>
<tr class="odd">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="even">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="odd">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

**Request**

``` xml
GET /organizations/ORG_NAME/pushy/jobs/ID
```

**Response**

The response will return something similar to:

``` javascript
{
  "id": "aaaaaaaaaaaa25fd67fa8715fd547d3d",
  "command": "chef-client",
  "run_timeout": 300,
  "status": "running",
  "created_at": "Tue, 04 Sep 2012 23:01:02 GMT",
  "updated_at": "Tue, 04 Sep 2012 23:17:56 GMT",
  "nodes": {
    "running": ["NODE1", "NODE5"],
    "complete": ["NODE2", "NODE3", "NODE4"],
    "crashed": ["NODE6"]
  }
}
```

where:

-   `nodes` is one of the following: `aborted` (node ran command,
    stopped before completion), `complete` (node ran command to
    completion), `crashed` (node went down after command started
    running), `nacked` (node was busy), `new` (node has not accepted or
    rejected command), `ready` (node has accepted command, command has
    not started running), `running` (node has accepted command, command
    is running), and `unavailable` (node went down before command
    started).
-   `status` is one of the following: `aborted` (the job was aborted),
    `complete` (the job completed; see `nodes` for individual node
    status), `quorum_failed` (the command was not run on any nodes),
    `running` (the command is running), `timed_out` (the command timed
    out), and `voting` (waiting for nodes; quorum not yet met).
-   `updated_at` is the date and time at which the job entered its
    present `status`

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>400</code></td>
<td>Bad request. The contents of the request are not formatted correctly.</td>
</tr>
<tr class="odd">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="even">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="odd">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

### node_states

{{% api_push_jobs_endpoint_node_states %}}

#### GET

{{% api_push_jobs_endpoint_node_states_get %}}

### node_states/NODE_NAME

{{% api_push_jobs_endpoint_node_name %}}

#### GET

{{% api_push_jobs_endpoint_node_name_get %}}

## push-jobs-client

{{% ctl_push_jobs_client_summary %}}

### Options

This command has the following syntax:

    push-jobs-client OPTION VALUE OPTION VALUE ...

This command has the following options:

`-c CONFIG`, `--config CONFIG`

:   The configuration file to use. Chef Infra Client and Chef Push Jobs
    client use the same configuration file: client.rb. Default value:
    `Chef::Config.platform_specific_path("/etc/chef/client.rb")`.

`-h`, `--help`

:   Show help for the command.

`-k KEY_FILE`, `--client-key KEY_FILE`

:   The location of the file that contains the client key.

`-l LEVEL`, `--log_level LEVEL`

:   The level of logging to be stored in a log file.

`-L LOCATION`, `--logfile LOCATION`

:   The location of the log file. This is recommended when starting any
    executable as a daemon.

`-N NODE_NAME`, `--node-name NODE_NAME`

:   The name of the node.

`-S URL`, `--server URL`

:   The URL for the Chef Infra Server.

`-v`, `--version`

:   The version of Chef Push Jobs.

## opscode-push-jobs-server.rb

{{% config_rb_push_jobs_server_summary %}}

### Settings

This configuration file has the following settings:

`api_port`

:   NGINX forwards requests to this port on the push-jobs server as part of the
    push-jobs communication channel. Default value: `10003`.

`command_port`

:   The port on which a Chef Push Jobs server listens for requests that
    are to be executed on managed nodes. Default value: `10002`.

`heartbeat_interval`

:   The frequency of the Chef Push Jobs server heartbeat message.
    Default value: `1000` (milliseconds).

`server_heartbeat_port`

:   The port on which the Chef Push Jobs server receives heartbeat
    messages from each Chef Push Jobs client. (This port is the `ROUTER`
    half of the ZeroMQ DEALER / ROUTER pattern.) Default value: `10000`.

`server_name`

:   The name of the Chef Push Jobs server.

`zeromq_listen_address`

:   The IP address used by ZeroMQ. Default value: `tcp://*`.
