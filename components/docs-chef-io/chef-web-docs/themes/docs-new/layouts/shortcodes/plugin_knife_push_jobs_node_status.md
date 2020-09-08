Use the `node status` argument to identify nodes that Chef Push Jobs may
interact with. Each node is always in one of the following states:

`new`

:   Node has neither committed nor refused to run the command.

`ready`

:   Node has committed to run the command but has not yet run it.

`running`

:   Node is presently running the command.

`succeeded`

:   Node successfully ran the command (an exit code of 0 was returned).

`failed`

:   Node failed to run the command (an exit code of non-zero was
    returned).

`aborted`

:   Node ran the command but stopped before completion.

`crashed`

:   Node went down after it started running the job.

`nacked`

:   Node was busy when asked to be part of the job.

`unavailable`

:   Node went down before it started running.

`was_ready`

:   Node was ready but quorum failed.

`timed_out`

:   Node timed out.