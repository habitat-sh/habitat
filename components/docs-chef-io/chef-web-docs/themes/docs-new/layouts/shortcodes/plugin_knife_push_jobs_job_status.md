Use the `job status` argument to view the status of Chef Push Jobs jobs.
Each job is always in one of the following states:

`new`

:   New job status.

`voting`

:   Waiting for nodes to commit or refuse to run the command.

`running`

:   Running the command on the nodes.

`complete`

:   Ran the command. Check individual node statuses to see if they
    completed or had issues.

`quorum_failed`

:   Did not run the command on any nodes.

`crashed`

:   Crashed while running the job.

`timed_out`

:   Timed out while running the job.

`aborted`

:   Job aborted by user.