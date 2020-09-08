Use the `chef clean-policy-revisions` subcommand to delete orphaned
policy revisions to Policyfile files from the Chef Infra Server. An
orphaned policy revision is not associated to any policy group and
therefore is not in active use by any node. Use
`chef show-policy --orphans` to view a list of orphaned policy
revisions.