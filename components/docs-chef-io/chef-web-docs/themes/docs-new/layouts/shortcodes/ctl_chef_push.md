Use the `chef push` subcommand to upload an existing
`Policyfile.lock.json` file to the Chef Infra Server, along with all of
the cookbooks that are contained in the file. The `Policyfile.lock.json`
file will be applied to the specified policy group, which is a set of
nodes that share the same run-list and cookbooks.