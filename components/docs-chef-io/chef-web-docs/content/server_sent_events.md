+++
title = "Push Jobs Server Sent Events Feed"
draft = false

aliases = ["/server_sent_events.html"]

[menu]
  [menu.infra]
    title = "Chef Infra Server Sent Events"
    identifier = "chef_infra/managing_chef_infra_server/push_jobs/server_sent_events.md Chef Infra Server Sent Events"
    parent = "chef_infra/managing_chef_infra_server/push_jobs"
    weight = 60
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/server_sent_events.md)

The Push Jobs server provides feeds of events associated with actions,
via the [Server-Sent-Events (SSE)
protocol](http://www.w3.org/TR/eventsource/). There are two SSE feeds
available:

1.  Job Feed: Stream of events for a particular job
2.  Organization Feed: Stream of events for all jobs across an
    organization

## Event Types

As mandated by the SSE protocol, each event has:

-   a type-specifier (`event`)
-   an ID (`id`)
-   `data`

The structure of an individual event appears as follows:

``` xml
event: EVENT_TYPE
id: EVENT_ID
data: {"timestamp": "2014-07-10 05:17:44.995958Z", ...}
```

-   `EVENT_TYPE` varies depending on the stream you request. In a Job
    Feed you could receive `start`, `quorum_vote`, `quorum_succeeded`,
    `run_start`, `run_complete`, `job_complete`, `rehab`, and `summary`.
    In the Organization Feed you could receive `start`, `job_complete`,
    and `start_of_history`.
-   `EVENT_ID` is not globally unique. It is an opaque string that is
    unique only to the individual stream.
-   `data` is a JSON object which content depends on the event. However,
    the JSON object for each event type includes a server-based
    timestamp in ISO8601 format.

In addition, SSE allows comments in the stream, indicated by a starting
colon. Push Jobs Server uses comments to send "no-op" events every 15
(configurable) seconds, as a form of keepalive for the socket.

### start

This event is issued when a job is requested.

**Example Event**

``` xml
event: start
id: dcd37f50-2d77-4761-895b-33134dbf87d5
data: {"timestamp":"2014-07-10 05:17:44.995958Z","job":"768b8d57-3cd0-434c-9f98-77e52cb96b86","command":"chef-client","run_timeout":300,"user":"rebecca","quorum":3,"node_count":3}
```

where:

-   `job` is the Job ID. (Not present in Job Feed)
-   `command` is the command that was run
-   `run_timeout` is the timeout in seconds specified in the job request
-   `user` is the user making the job request
-   `quorum` is the number of clients required to accept the command as
    specified in the job request
-   `node_count` is the number of nodes in the request

### quorum_vote

This event is issued as each node responds to the quorum request. (Not
available in Organization Feed)

**Example Event**

``` xml
event: quorum_vote
id: dcd37f50-2d77-4761-895b-33134dbf87d5
data: {"timestamp":"2014-07-10 05:17:44.995958Z","node":"NODE1","status":"success"}
```

where:

-   `node` is the name of the node responding to the vote
-   `status` is one of the following: `client_died_while_voting` (node
    went offline during voting), `failure` (node rejected the job),
    `lost_availability` (node become unavailable during voting),
    `success` (node accepted the job), `unexpected_commit` (node
    attempted to vote twice)

### quorum_succeeded

This event is issued when the vote is complete, and the nodes are told
to run the command. (Not available in Organization Feed)

Please note: there is no corresponding `quorum_failed` event. If the
quorum fails, then the `job_complete` event will include a
"quorum_failed" status.

**Example Event**

``` xml
event: quorum_succeeded
id: dcd37f50-2d77-4761-895b-33134dbf87d5
data: {"timestamp": "2014-07-10 05:17:44.995958Z"}
```

### run_start

This event is issued as each node acknowledges that it is running the
command. (Not available in Organization Feed)

**Example Event**

``` xml
event: run_start
id: dcd37f50-2d77-4761-895b-33134dbf87d5
data: {"timestamp":"2014-07-10 05:17:44.995958Z","node":"NODE1"}
```

where:

-   `node` is the name of the node

### run_complete

This event is issued as each node completes the command. (Not available
in Organization Feed)

**Example Event**

``` xml
event: run_complete,
id: dcd37f50-2d77-4761-895b-33134dbf87d5
data: {"timestamp":"2014-07-10 05:17:44.995958Z","node":"NODE1","status":"crashed"}
```

where:

-   `node` is the name of the node
-   `status` is one of the following: `client_died_while_running` (node
    went offline before finishing the run), `crashed` (node terminated
    run without returning status), `failure` (run failed), `run_nacked`
    (node rejected the run after quorum was reached),
    `run_nacked_while_running` (node rejected the run after starting
    it), or `success` (the run completed successfully),

### job_complete

This event is issued when the job completes.

**Example Event**

``` xml
event: job_complete
id: dcd37f50-2d77-4761-895b-33134dbf87d5
data: {"timestamp":"2014-07-10 05:17:44.995958Z","job":"768b8d57-3cd0-434c-9f98-77e52cb96b86","status":"complete"}
```

where:

-   `job` is the Job ID (Not present in Job Feed)
-   `status` is one of the following: `aborted` (the job was aborted),
    `complete` (the job completed), `quorum_failed` (the command was not
    run on any nodes), or `timed_out` (the command timed out)

### start_of_history

This synthetic event is issued when the `Last-Event-ID` header is not
recognized. (Not available in Job Feed)

**Example Event**

``` xml
event: start_of_history
id: dcd37f50-2d77-4761-895b-33134dbf87d5
data: {"timestamp":"2014-07-10 05:17:44.995958Z"}
```

### rehab

This event is issued when the server detects an abnormality with a node
and attempts to repair it. (Not available in Organization Feed)

``` xml
event: rehab
id: dcd37f50-2d77-4761-895b-33134dbf87d5
data: {"timestamp":"2014-07-10 05:17:44.995958Z","node":"NODE1"}
```

### summary

This event is issued when a request for the Job Feed comes in after the
job has completed. (Not available in Organization Feed)

``` xml
event: summary
id: dcd37f50-2d77-4761-895b-33134dbf87d5
data: {"timestamp":"2014-07-10 05:17:44.995958Z","id":"aaaaaaaaaaaa25fd67fa8715fd547d3d","command":"chef-client", ... }
```

where:

-   `data` is the same Job Summary you would get from the `job/ID`
    endpoint

## Event Stream Examples

An event stream should follow a standard structure:

### Job Feed (Normal Run Execution)

1.  `start`
2.  0 or more `quorum_vote`
3.  `quorum_succeeded`
4.  1 or more `run_start`
5.  1 or more `run_complete`
6.  `job_complete`

**Request**

``` xml
Accept: text/event-stream
GET /organizations/ORG_NAME/pushy/jobs_status_feed/JOBID
```

**Response**

The response will return something similar to:

``` xml
event: start
id: dcd37f50-2d77-4761-895b-33134dbf87d1
data: {"timestamp":"2014-07-10 05:17:40.995958Z","command":"ls /etc/chef","run_timeout":300,"user":"rebecca","quorum":2,"node_count":2}

event: quorum_vote
id: dcd37f50-2d77-4761-895b-33134dbf87d2
data: {"timestamp":"2014-07-10 05:17:41.995958Z","node":"NODE1","status":"success"}

event: quorum_vote
id: dcd37f50-2d77-4761-895b-33134dbf87d3
timestamp:
data: {"timestamp":"2014-07-10 05:17:42.995958Z","node":"NODE2","status":"success"}

event: quorum_succeeded
id: dcd37f50-2d77-4761-895b-33134dbf87d4
data: {"timestamp":"2014-07-10 05:17:43.995958Z"}

event: run_start
id: dcd37f50-2d77-4761-895b-33134dbf87d5
data: {"timestamp":"2014-07-10 05:17:44.995958Z","node":"NODE1"}

event: run_start
id: dcd37f50-2d77-4761-895b-33134dbf87d6
data: {"timestamp":"2014-07-10 05:17:45.995958Z","node":"NODE2"}

event: run_complete
id: dcd37f50-2d77-4761-895b-33134dbf87d7
data: {"timestamp":"2014-07-10 05:17:46.995958Z","node":"NODE1","status":"success"}

event: run_complete
id: dcd37f50-2d77-4761-895b-33134dbf87d58
timestamp: "
data: {"timestamp":"2014-07-10 05:17:47.995958Z","node":"NODE2","status":"success"}

event: job_complete
id: dcd37f50-2d77-4761-895b-33134dbf87d9
data": {"timestamp":"2014-07-10 05:17:48.995958Z","status":"complete"}
```

### Job Feed (Failed Quorum)

1.  `start`
2.  1+ `quorum_failed`
3.  `job_complete`

**Request**

``` xml
Accept: text/event-stream
GET /organizations/ORG_NAME/pushy/jobs_status_feed/JOBID
```

**Response**

The response will return something similar to:

``` xml
event: start
id: dcd37f50-2d77-4761-895b-33134dbf87d1
data": {"timestamp":"2014-07-10 05:17:40.995958Z","command":"ls /etc/chef","run_timeout":300,"user":"rebecca","quorum":2,"node_count":2}

event: quorum_vote
id: dcd37f50-2d77-4761-895b-33134dbf87d2
data": {"timestamp":"2014-07-10 05:17:41.995958Z","node":"NODE","status":"failure"}

event: quorum_vote
id: dcd37f50-2d77-4761-895b-33134dbf87d3
data": {"timestamp":"2014-07-10 05:17:42.995958Z","node":"NODE2","status":"success"}

event: job_complete
id: dcd37f50-2d77-4761-895b-33134dbf87d9
data": {"timestamp":"2014-07-10 05:17:48.995958","status":"quorum_failed"}
```

### Organization Feed

1.  `start` (job=B)
2.  `job_complete` (job=A)
3.  `start` (job=C)
4.  `job_complete` (job=C)
5.  `job_complete` (job=B)

**Request**

``` xml
Accept: text/event-stream
GET /organizations/ORG_NAME/pushy/jobs_status_feed/JOBID
```

**Response**

The response will return something similar to:

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
