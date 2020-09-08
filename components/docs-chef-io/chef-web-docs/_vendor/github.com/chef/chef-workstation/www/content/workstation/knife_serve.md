+++
title = "knife serve"
draft = false

aliases = ["/knife_serve.html", "/knife_serve/"]

[menu]
  [menu.workstation]
    title = "knife serve"
    identifier = "chef_workstation/chef_workstation_tools/knife/knife_serve.md knife serve"
    parent = "chef_workstation/chef_workstation_tools/knife"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/knife_serve.md)

{{% knife_serve_summary %}}

## Syntax

This subcommand has the following syntax:

``` bash
knife serve (options)
```

## Options

{{< note >}}

{{% knife_common_see_common_options_link %}}

{{< /note >}}

This subcommand has the following options:

`--chef-repo-path PATH`

:   The path to the chef-repo. Default value: same as `chef_repo_path`
    in the client.rb file.

`--chef-zero-host IP`

:   Override the host on which chef-zero listens. Default value:
    `127.0.0.1`.

`--chef-zero-port PORT`

:   The port on which chef-zero listens. The default behavior will bind
    to the first available port between `8889` and `9999`.

`--repo-mode MODE`

:   Use to specify the local chef-repo layout. Possible values: `static`
    (for environments, roles, data bags, and cookbooks), `everything`
    (same as static, plus nodes, clients, and users),
    `hosted_everything` (for ACLs, groups). Default value:
    `everything/hosted_everything`.

## Examples

None.
