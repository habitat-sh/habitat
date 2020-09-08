+++
title = "Scan Jobs"

date = 2018-03-26T16:02:35-07:00
draft = false
[menu]
  [menu.automate]
    title = "Scan Jobs"
    parent = "automate/compliance"
    identifier = "automate/compliance/scan_jobs.md Scan Jobs"
    weight = 20
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/scan_jobs.md)

## Overview

A scan job is the equivalent of running `inspec exec` against a given set of targets.  The results of a scan job are sent to compliance reporting. Any profiles installed to the user's namespace may be used in a scan job.

### Scanning

Run scan jobs on:

* manually added nodes
* aws-ec2 instances
* aws-api regions
* azure-vm virtual machines
* azure-api subscriptions

### Scheduled Jobs

Jobs may be scheduled to be executed now, at some later point in time, as a one-off job, or a job that is executed at a given interval (like once a day, or every two hours).

### Creating a Job

When creating a scan job via ui, the user may select one or many managers. These managers would be the `Automate` manager (for nodes added manually in the ui), as well as any other managers added via the integrations (such as `aws-ec2`, or `aws-api`).
The user may then build a query based on the base manager used to limit the set of items being scanned.

### Create a Scan Job Using the API

```shell
curl -sSX POST "https://automate-url/api/v0/compliance/scanner/jobs" -d
'{
  "name": "my scan job",
  "tags": [],
  "type": "exec",
  "profiles": [
    "https://github.com/dev-sec/linux-baseline", # external url profile
    "compliance://admin/linux-baseline#2.1.1"  # installed profile (see profile)
  ],
  "recurrence": "FREQ=HOURLY;INTERVAL=1",
  "nodes": ["6c0c7942-acb6-4ede-8a3a-bf8f6beee362"], # this field can be used to run a job on a specific (static) node
  "node_selectors": [
    {
      "manager_id": "e69dc612-7e67-43f2-9b19-256afd385820",
      "filters": [
        {"key": "name", "values": ["vj-nodes*"]}, # query by name of manually added nodes
        {"key": "my-manual-node-tag-key", "values": ["unicorn"]} # query by tags of manually added nodes
      ]
    },
    {
      "manager_id": "2683f1f2-ebfd-4faa-807b-7801ec28ed17", # we could pretend this is the uuid for a user's aws-ec2 mgr
      "filters": [
        {"key": "region", "values": ["eu*", "us-east-1"], "exclude": true },
        {"key": "Name", "values": ["test*"]},
        {"key": "X-Contact", "values": ["vjeffrey"]}
      ]
    }
  ]
}'
-H "X-Data-Collector-Token: $DC_TOKEN"
```

### Supported Filters

* manual nodes mgr
  * name (exclude/include, multiple values, wildcard match)
  * tags(by key) (exclude/include, multiple values, wildcard match)
  * status (reachable, unreachable)
* aws-ec2 mgr
  * region (exclude/include, multiple values, wildcard match)
  * tags(by key) (exclude/include, multiple values, wildcard match)
* aws-api mgr
  * region (include, exclude, multiple values, exact match ("us-east-1"))
* azure-vm mgr
  * region (exclude/include, multiple values)
  * name (include/exclude, wildcard match only supported for include)
  * tags (include, multiple values, exact match)
* azure-api mgr
  * subscriptions (include, exclude, multiple values, exact match)

### Manual Nodes

For users that wish to scan nodes that are not otherwise discoverable via integrations, we provide a `manually added nodes manager`.
Users may add nodes via ui or api, specifying the node's ip/hostname, port configuration, and sudo requirement (if any). In order to be able to run a scan job on a node, the correct set of ssh, winrm, and/or sudo credentials must be associated with that node. This is done by associating the desired credential ids with the node.

```bash
curl -sSX POST "https://automate-url/api/v0/nodes" -d
'{
  "name": "my-localhost-node",
  "tags": [
    "key": "department", "value": "engineering"
  ],
  "target_config": {
    "backend": "ssh",
    "secrets": ["df312aa3-99c8-4f8a-af81-067307b31ee6"],
    "port":22,
    "sudo":false,
    "host":"localhost"
  }
}'
-H "X-Data-Collector-Token: $DC_TOKEN"
```

### Unreachable and Reachable (status)

Whenever we are provided with information about a scannable node, we run a detect job (`inspec detect`) against that scannable node. If we are able to successfully connect, it is marked as reachable. An unreachable scannable node will have an error message attached to it. A user may edit the credentials associated with the item to resolve the problem, or in some cases a user may just want to rerun a node (as in the case of a network error).

```bash
curl -sSX GET "https://automate-url/api/v0/nodes/rerun/id/d92b0c26-0c9a-4a04-b694-82fa979b2578"
```

### How to Add an Environment Value for Project Assignment

When creating a manually added node, use the 'Environment' tag to assign an environment to the node. `Environment` is the only supported field for project filtering on scan job results.
![Manual Node With Environment Tag](/images/automate/environment-tag-manual-node.png)


### FAQ

* I scheduled a job to run against my aws-ec2 instances once a day. What happens when I add new instances to the account, or remove some?

Every time the scan job is scheduled to run, we will query the provider for a current list of nodes, so we always have the most current list. These are the instances we will run the scan job against. If a query (limiting the set of items to scan) was included during job creation, we will also respect that query against the most current list of instances.
