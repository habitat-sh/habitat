+++
title = "knife push jobs"
draft = false

aliases = ["/plugin_knife_push_jobs.html"]

[menu]
  [menu.infra]
    title = "knife push jobs"
    identifier = "chef_infra/managing_chef_infra_server/push_jobs/plugin_knife_push_jobs.md knife push jobs"
    parent = "chef_infra/managing_chef_infra_server/push_jobs"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/plugin_knife_push_jobs.md)

{{% plugin_knife_push_jobs_summary %}}

{{< note >}}

Review the list of [common options](/workstation/knife_options/) available to
this (and all) knife subcommands and plugins.

{{< /note >}}

## job list

{{% plugin_knife_push_jobs_job_list %}}

### Syntax

{{% plugin_knife_push_jobs_job_list_syntax %}}

### Options

This command does not have any specific options.

## job output

The `job output` command is used to view the output of Push jobs. The
output capture flag must have been set on `job start`; see the
`--capture` option.

### Syntax

This argument has the following syntax:

``` bash
knife job output (options) JOB_ID [NODE_NAME]
```

### Options

This argument has the following options:

`--channel [stderr | stdout]`

:   The output channel to capture.

### Examples

**View the STDOUT from my-node for job with ID
26e98ba162fa7ba6fb2793125553c7ae**

``` bash
knife job output --channel stdout 26e98ba162fa7ba6fb2793125553c7ae my-node
```

## job start

{{% plugin_knife_push_jobs_job_start %}}

### Syntax

{{% plugin_knife_push_jobs_job_start_syntax %}}

### Options

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

`-b`, `--nowait`

:   Exit immediately after starting a job instead of waiting for it to
    complete.

`--with-env ENVIRONMENT`

:   Accept a JSON blob of environment variables and use those to set the
    variables for the client. For example `'{"test": "foo"}'` will set
    the push client environment variable "test" to "foo".

`--in-dir DIR`

:   Execute the remote command in the directory `DIR`.

`--file DATAFILE`

:   Send the file to the client. Cleaned

`--capture`

:   Capture stdin and stdout for this job.

### Examples

**Run a job**

{{% plugin_knife_push_jobs_job_start_run_job %}}

**Run a job using quorum percentage**

{{% plugin_knife_push_jobs_job_start_search_by_quorum %}}

**Run a job using node names**

{{% plugin_knife_push_jobs_job_start_search_by_nodes %}}

## job status

{{% plugin_knife_push_jobs_job_status %}}

### Syntax

{{% plugin_knife_push_jobs_job_status_syntax %}}

### Options

This command does not have any specific options.

### Examples

**View job status by job identifier**

{{% plugin_knife_push_jobs_job_status_by_id %}}

## node status

{{% plugin_knife_push_jobs_node_status %}}

### Syntax

{{% plugin_knife_push_jobs_node_status_syntax %}}

### Options

This command does not have any specific options.
