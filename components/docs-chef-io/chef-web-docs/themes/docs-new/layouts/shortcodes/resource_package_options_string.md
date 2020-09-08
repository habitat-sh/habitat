When using an explicit `gem_binary`, options must be passed as a string.
When not using an explicit `gem_binary`, Chef Infra Client is forced to
spawn a gems process to install the gems (which uses more system
resources) when options are passed as a string. String options are
passed verbatim to the gems command and should be specified just as if
they were passed on a command line. For example, `--prerelease` for a
pre-release gem.