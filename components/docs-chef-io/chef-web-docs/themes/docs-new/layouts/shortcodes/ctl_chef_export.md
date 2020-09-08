Use the `chef export` subcommand to create a chef-zero-compatible
chef-repo that contains the cookbooks described by a
`Policyfile.lock.json` file. After a chef-zero-compatible chef-repo is
copied to a node, the policy can be applied locally on that machine by
running `chef-client -z` (local mode).