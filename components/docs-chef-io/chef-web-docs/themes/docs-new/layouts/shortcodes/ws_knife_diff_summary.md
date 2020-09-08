Use the `knife diff` subcommand to compare the differences between files
and directories on the Chef Infra Server and in the chef-repo. For
example, to compare files on the Chef Infra Server prior to uploading or
downloading files using the `knife download` and `knife upload`
subcommands, or to ensure that certain files in multiple production
environments are the same. This subcommand is similar to the `git diff`
command that can be used to diff what is in the chef-repo with what is
synced to a git repository.