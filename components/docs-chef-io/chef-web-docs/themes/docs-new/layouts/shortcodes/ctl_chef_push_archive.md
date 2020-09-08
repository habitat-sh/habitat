The `chef push-archive` subcommand is used to publish a policy archive
file to the Chef Infra Server. (A policy archive is created using the
`chef export` subcommand.) The policy archive is assigned to the
specified policy group, which is a set of nodes that share the same
run-list and cookbooks.