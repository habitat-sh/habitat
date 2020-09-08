+++
title = "Nodes API"

draft = false
[menu]
  [menu.automate]
    title = "Nodes"
    parent = "automate/compliance"
    identifier = "automate/compliance/nodes.md Nodes"
    weight = 40
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/nodes.md)

### Nodes

The `/nodes` endpoint in Chef Automate is something of a 'logbook' of the nodes in your infrastructure.

When a user creates a node, that node is added to the `/nodes` endpoint.

When a user adds a node integration, like aws or azure, nodes are added to the `/nodes` endpoint.

When a Chef InSpec report or a Chef Infra Client run is ingested, a node is added to the `/nodes` endpoint. If the node already exists, its last contact time, run data, and scan data are updated. When run data and scan data are updated, the latest information is stored for the run id or report id, the status, and the penultimate status.

### Node Status

All nodes have one of three possible statuses: 'unknown', 'reachable', and 'unreachable'. The default status is 'unknown'.

Each time a user adds one or more nodes manually or with a node integration (AWS, Azure, or GCP), Chef Automate runs an`inspec detect` job on the newly added node(s).
If the detect job is successful, the node status updates from 'unknown' to 'reachable', and the platform information is updated from the `inspec detect` results.
If the detect job is unsuccessful, meaning the node could not be reached, the node's status updates to 'unreachable'.
The status updates every time a scan job runs on the node.

### Node State

All nodes have a state.
Possible states are unknown(''), 'running', 'stopped', and 'terminated'. Default state: '' (unknown).

<!-- Node state can be updated manually for all Automate (manually managed) nodes.
```
``` I need to expose that endpoint in the gateway -->

 For all nodes added through integrations, node state is updated both when nodes are added and on a scheduled polling interval.

If a node is found to have a state other than 'running', the node status is then also updated to 'unreachable'.

Node state is updated to 'running' on report ingestion if the end time recorded by the inspec report is less than ten minutes from the ingestion time.

### Filtering Nodes

The `/nodes` endpoint supports filtering by:

- name
- platform_name
- platform_release
- manager_type ('automate', 'aws-ec2', 'aws-api', ...)
- manager_id
- account_id (the aws account ID or azure tenant ID)
- region
- source_id (a reference to the primary provider's node)
- state
- statechange_timerange (supports two timestamps of type "2019-03-05T00:00:00Z")
- status
- tags
- last_run_timerange (last time reported on ingested ccr: supports two timestamps of type "2019-03-05T00:00:00Z" (RFC3339))
- last_scan_timerange (last time reported on ingested scan: supports two timestamps of type "2019-03-05T00:00:00Z" (RFC3339))
- last_run_status (status on last ingested ccr)
- last_scan_status (status on last ingested scan)
- last_run_penultimate_status (status on second to last ingested ccr)
- last_scan_penultimate_status (status on second to last ingested scan)

## Examples

* Show me all nodes whose last scan had a status of failed and a penultimate status of passed

_or in other words, which nodes were previously passing their scans and just started failing?_

sample request:
```bash
curl -s --insecure -H "api-token: $token_val"
https://a2-dev.test/api/v0/nodes/search -d '{
  "filters": [
    {"key": "last_scan_status", "values": ["FAILED"]},
    {"key": "last_scan_penultimate_status", "values": ["PASSED"]}
  ]
}'
```


sample truncated response:
```
{"nodes":[{"id":"0e05fcf2-2fab-36ee-bb84-5b7d5888c33a","name":"chef-load-blue-delladonna-indigo","platform":"debian","platform_version":"8.11","last_contact":"2019-05-14T18:08:43Z","run_data":{"id":"","status":"UNKNOWN","penultimate_status":"UNKNOWN","end_time":null},"scan_data":{"id":"5640fbb7-d1ba-4c67-b0cd-9db4fcfc2598","status":"FAILED","penultimate_status":"PASSED","end_time":"2019-05-14T18:08:43Z"}}]}
```


* Show me all nodes whose last ccr passed and last scan failed, that had a penultimate ccr status of failed

_or in other words, which nodes just started passing their ccrs but are failing their scans?_

sample request:
```bash
curl -s --insecure -H "api-token: $token_val"
https://a2-dev.test/api/v0/nodes/search -d '{
  "filters": [
    {"key": "last_run_status", "values": ["PASSED"]},
    {"key": "last_scan_status", "values": ["FAILED"]},
    {"key": "last_run_penultimate_status", "values": ["FAILED"]}
  ]
}'
```


* Show me all nodes that had a last scan ingested sometime in the last 48 hours with a status of failed

_or in other words, which nodes that were ingested in the last 48 hours failed their scans?_

sample request:
```bash
curl -s --insecure -H "api-token: $token_val"
https://a2-dev.test/api/v0/nodes/search -d '{
  "filters": [
    {"key": "last_scan_status", "values": ["FAILED"]},
    {"key": "last_scan_timerange", "values": ["2019-05-12T00:00:00Z", "2019-05-16T00:00:00Z" ]}
  ]
}'
```


* Show me all nodes tagged with `deployment:staging` OR `deployment:test`. We OR between multiple values of the same key

sample request:
```bash
curl -s --insecure -H "api-token: $token_val"
https://a2-dev.test/api/v0/nodes/search -d '{
 "filters": [
   {"key": "deployment", "values": ["staging", "test"]}
 ]
}'
```


* Show me all nodes tagged with `deployment:prod` AND `org:marketing`. We AND between different tag key filters

sample request:
```bash
curl -s --insecure -H "api-token: $token_val"
https://a2-dev.test/api/v0/nodes/search -d '{
 "filters": [
   {"key": "deployment", "values": ["prod"]},
   {"key": "org", "values": ["marketing"]}
 ]
}'
```


### Bulk Node Add

 Use the `nodes/bulk-create` endpoint to add multiple nodes with the same set of tags and credentials.  Specifying a `name_prefix` for the nodes in question results in a node name of `prefix-host`.  Specified tags will be added to each node. The endpoint takes an array of node objects, allowing users to add as many nodes as needed.

```bash
curl -s --insecure -H "api-token: $token_val" https://a2-dev.test/api/v0/nodes/bulk-create -d '
{"nodes": [{
     "name_prefix": "my-ssh-node",
     "manager":"automate",
     "target_config": {
        "backend":"ssh",
        "hosts":["localhost","127.0.0.1"],
        "secrets":["2998c3a1-d596-43d4-b2b3-4837a46cee19"],
        "port": 22
      },
      "tags": [
        { "key":"test-node", "value":"is-amazing" },
        { "key":"compliance-service", "value":"rockin-like-whoa" },
        { "key":"_no_auto_detect", "value":"true" }
      ]
    },
    {
     "name": "my-other-node",
     "manager":"automate",
      "target_config": {
        "backend":"ssh",
        "hosts":["localhost"],
        "secrets":["2998c3a1-d596-43d4-b2b3-4837a46cee19"],
        "port": 22
      },
      "tags": [
        { "key":"test-node", "value":"is-more-amazing" }
      ]
    }
  ]
}'
```

### Bulk Node Delete

 The `/nodes/delete` endpoint allows users to bulk-delete nodes based on a query. To examine the outcome of this destructive action before running it, test the query first on the `api/v0/nodes/search` endpoint.

```bash
curl -s --insecure -H "api-token: $token_val"
https://a2-dev.test/api/v0/nodes/delete -d '{
  "filters": [
    {"key": "name", "values": ["vj*"]}
  ]
}'
```
