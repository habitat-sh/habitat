+++
title = "Deprecation: run_command and popen4 helper method removal (OHAI-3)"
draft = false
robots = "noindex"


aliases = "/deprecations_ohai_run_command_helpers.html"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/deprecations_ohai_run_command_helpers.md)

Ohai ships a command mixin for use by plugin authors in shelling out to
external commands. This mixin originally included `run_command` and
`popen4` methods, which were deprecated in Ohai 8.11.1 (Chef Client
12.8.1) in favor of the more robust `mixlib-shellout` gem functionality.
In Chef Client 13 these deprecated methods will be removed, breaking any
Ohai plugins authored using the deprecated methods.

## Remediation

Plugins should be updated to use mixlib-shellout instead of the
run_command.

Deprecated run_command based code:

``` ruby
status, stdout, stderr = run_command(:command => "myapp --version")
if status == 0
  version = stdout
end
```

Updated code for mixlib shellout:

``` ruby
so = shell_out("myapp --version")
if so.exitstatus == 0
  version = so.stdout
end
```

See the [mixlib-shellout repo](https://github.com/chef/mixlib-shellout)
for additional usage information.
