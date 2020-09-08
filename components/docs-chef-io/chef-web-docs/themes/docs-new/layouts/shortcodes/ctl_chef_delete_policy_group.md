Use the `chef delete-policy-group` subcommand to delete the named policy
group from the Chef Infra Server. Any policy revision associated with
that policy group is not deleted. (The state of the policy group is
backed up locally and may be restored using the `chef undelete`
subcommand.)