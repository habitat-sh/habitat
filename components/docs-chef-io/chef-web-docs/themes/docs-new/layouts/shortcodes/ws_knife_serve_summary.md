Use the `knife serve` subcommand to run a persistent chef-zero against
the local chef-repo. (chef-zero is a lightweight Chef Infra Server that
runs in-memory on the local machine.) This is the same as running the
Chef Infra Client executable with the `--local-mode` option. The
`chef_repo_path` is located automatically and the Chef Infra Server will
bind to the first available port between `8889` and `9999`.
`knife serve` will print the URL for the local Chef Infra Server, so
that it may be added to the config.rb file.