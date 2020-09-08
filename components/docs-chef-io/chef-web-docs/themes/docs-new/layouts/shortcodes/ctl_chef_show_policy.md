Use the `chef show-policy` subcommand to display revisions for every
`Policyfile.rb` file that is on the Chef Infra Server. By default, only
active policy revisions are shown. When both a policy and policy group
are specified, the contents of the active `Policyfile.lock.json` file
for the policy group is returned.