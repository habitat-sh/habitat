+++
title = "Release Notes: Chef Push Jobs 1.0 - 2.5"
draft = false

aliases = ["/release_notes_push_jobs.html"]

[menu]
  [menu.release_notes]
    title = "Chef Push Jobs"
    identifier = "release_notes/release_notes_push_jobs.md Chef Push Jobs"
    parent = "release_notes"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/release_notes_push_jobs.md)

Chef Push Jobs is an extension of the Chef server that allows jobs to be
run against nodes independently of a chef-client run. A job is an action
or a command to be executed against a subset of nodes; the nodes against
which a job is run are determined by the results of a search query made
to the Chef server.

Chef Push Jobs uses the Chef server API and a Ruby client to initiate
all connections to the Chef server. Connections use the same
authentication and authorization model as any other request made to the
Chef server. A knife plugin is used to initiate job creation and job
tracking.

## What's New in 2.5

This release includes an important fix for a number of deadlock
scenarios. We encourage anyone using Push Jobs to upgrade.

This also includes a number of maintenance items, including:

Ruby and RubyGems upgraded to 2.4.4 and 2.7.6, respectively, to include
a number of security fixes. Chef Client (packaged as a library) upgraded
to 14.0.202 Other dependencies upgraded to latest

## What's New in 2.4

### Push Jobs Client 2.4.8

-   Adds support for AIX 7.1.

## What's New in 2.2

The following items are new for Chef Push Jobs:

-   Uses Chef Server \>12.14.0 [consolidated
    credentials](/server_security/#chef-infra-server-credentials-management)
    removing credentials stored in Push Server's configuration files.

### Important Notes

-   **Push Jobs Server 2.2.0** requires Chef Server 12.14.0 or later.

## What's New in 2.1

The following items are new for Chef Push Jobs:

-   **Push Jobs client 2.1** is backwards-compatible with the Chef Push
    Jobs 1.x server when the `allow_unencrypted` option to be set to
    `true` in `/etc/chef/push-jobs-client.rb`.
-   Allow the job execution environment to be set. This includes user,
    working directory, environment variables and a data file to be set.
-   STDOUT/STDERR can now optionally be captured from job execution and
    return it to the server. Users can retrieve the output via the
    `knife job output` command.
-   We now provide two SSE feed endpoints; one provides fine grained
    per-job events, while the other provides a per-org feed of jobs
    starting and completing.
-   Command communication now uses libsodium based encryption via
    zeromq4's CurveZMQ. This replaces the signing protocol used in 1.x.
    All zeromq packets are fully encrypted and signed, except for the
    server heartbeat broadcast, which is signed, but in the clear.
-   Push Jobs Server 2.1 is now certified for use with Chef Automate.

### Important Notes

-   **Push Jobs Server 2.1 is now fully supported for use with Chef
    Automate**.
-   **Push Jobs Server 2.0 is not compatible with Push Jobs Client
    1.X**. Ensure that all Push Jobs Clients are upgraded to the current
    stable release before performing an upgrade of your Push Jobs
    Server.

### Upgrading Chef Automate Installation to use Push Jobs Server 2.1

If your Chef Automate installation uses Push Jobs Server to manage build
nodes, upgrading to Push Jobs Server 2.1 is now fully supported. To
upgrade:

-   On each build node,
    [download](https://downloads.chef.io/push-jobs-client/stable/) the
    latest Push Jobs Client release and follow the [installation
    instructions](/install_push_jobs/#install-the-client) on each
    build node. If the build node was set up using
    `automate-ctl install-build-node`, then it runs on version 2.0 or
    greater and does not need upgraded.

    {{< warning spaces =4 >}}

    Do not restart the Push Jobs Client until after the Push Jobs Server
    upgrade is completed in the steps below.

    {{< /warning >}}

-   On the Push Jobs Server node:

    -   Install the latest [Push Jobs Server
        package](https://downloads.chef.io/push-jobs-server/stable/).

    -   Run `sudo opscode-push-jobs-server-ctl reconfigure`.

        {{< note spaces=8 >}}

        Once the `reconfigure` command above is issued, build nodes and
        other push clients will not be in communication with the server
        until they are restarted.

        {{< /note >}}

-   On each build node:

    -   Remove the `allow_unencrypted` entry from
        `/etc/chef/push-jobs-client.rb`, if present.
    -   Restart Push Jobs Client as appropriate for the system:
        `sudo systemctl restart push-jobs-client` OR
        `sudo service restart push-jobs-client`

## Encryption

All command channel communication is encrypted via SSL or
[CurveZMQ](https://rfc.zeromq.org/spec:26/CURVEZMQ). CurveZMQ is based
on the [CurveCP protocol](http://curvecp.org/security.html). The one
exception to this is the server heartbeat, which is broadcast in the
clear (but is still signed with the server key for integrity).

## Command Output Capture

Both the `knife-push` library and the Chef Push Jobs API provide options
to direct the client to capture the job output and return it to the
server for inspection:

``` bash
knife job start --capture "echo foobar" node1
Started. Job ID: 26e98ba162fa7ba6fb2793125553c7ae
.Complete.

knife job output --channel stdout 26e98ba162fa7ba6fb2793125553c7ae node1
foobar
```

## Environment Control

The user has a lot more control over the execution environment of the
remote command with three new options available to the
`knife push jobs start` command.

This includes:

-   Environment variables (`--with-env`)
-   Execution directory (`--in-dir`)
-   Data file sent from the user to the push client (`--file`)

<!-- -->

``` bash
knife job start --file .chef/config.rb --capture --with-env '{"test": "foo"}' --in-dir "/tmp" --as-user daemon "print_execution_environment" node2
Started. Job ID: 26e98ba162fac37787292637362808cb
...

knife job output --channel stdout 26e98ba162fac37787292637362808cb node2
{"HOME"=>"/home/vagrant",
...
"CHEF_PUSH_JOB_FILE"=>"/tmp/pushy/pushy_file20150813-14250-125xv4u",
"CHEF_PUSH_JOB_ID"=>"26e98ba162fac37787292637362808cb",
"CHEF_PUSH_NODE_NAME"=>"test",
"PWD"=>"/srv/piab/mounts/opscode-pushy-client",
"SHELL"=>"/bin/bash",
"test"=>"foo"}
```

In addition to environment variables specified with the `--with-env`
flag, there are three new special environment variables that are made
available to you automatically:

-   `CHEF_PUSH_JOB_FILE` - The path to the temporary file containing the
    string you passed using the `--file` parameter.
-   `CHEF_PUSH_NODE_NAME` - The name of the node this instance of the
    push job is being run on.
-   `CHEF_PUSH_JOB_ID` - The ID for the job currently being run.

## Server Sent Event Feeds

There are two new endpoints that provide feeds for the state of jobs on
the server. There's an organization-level feed that provides high level
job start/completion information, and a per job feed that provides node
level state changes for a particular job. As of this release, these
feeds are only available via the [Chef Push Jobs
API](/api_push_jobs/).

Here is a quick example of what an organization-level feed might look
like.

``` xml
event: start
id: dcd37f50-2d77-4761-895b-33134dbf87d1
data: {"timestamp":"2014-07-10 05:10:40.995958Z","job":"B","command":"chef-client","run_timeout":300,"user":"rebecca","quorum":2,"node_count":2}

event: job_complete
id: dcd37f50-2d77-4761-895b-33134dbf87d2
data: {"timestamp":"2014-07-10 05:15:48.995958Z","job":"A","status":"success"}

event: start
id: dcd37f50-2d77-4761-895b-33134dbf87d3
data: {"timestamp":"2014-07-10 05:17:40.995958Z","job":"C","command":"cat /etc/passwd","run_timeout":300,"user":"charles","quorum":2,"node_count":2}

event: job_complete
id: dcd37f50-2d77-4761-895b-33134dbf87d4
data: {"timestamp":"2014-07-10 05:17:41.995958Z","job":"C","status":"success"}

event: job_complete
id: dcd37f50-2d77-4761-895b-33134dbf87d5
data: {"timestamp":"2014-07-10 05:20:48.995958Z","job":"B","status":"success"}
```
