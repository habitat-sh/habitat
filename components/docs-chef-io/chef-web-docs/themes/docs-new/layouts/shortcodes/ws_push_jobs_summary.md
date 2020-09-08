Chef Push Jobs is an extension of the Chef Infra Server that allows jobs
to be run against nodes independently of a Chef Infra Client run. A job
is an action or a command to be executed against a subset of nodes; the
nodes against which a job is run are determined by the results of a
search query made to the Chef Infra Server.

Chef Push Jobs uses the Chef Infra Server API and a Ruby client to
initiate all connections to the Chef Infra Server. Connections use the
same authentication and authorization model as any other request made to
the Chef Infra Server. A knife plugin is used to initiate job creation
and job tracking.