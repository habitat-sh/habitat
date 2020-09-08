+++
title = "Deprecation: Deprecation of old shell_out APIs (CHEF-26)"
draft = false
robots = "noindex"


aliases = "/deprecations_shell_out.html"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/deprecations_shell_out.md)

The functionality of mutiple old <span
class="title-ref">shell_out</span> APIs has been collapsed into the
<span class="title-ref">shell_out</span> API itself, and the old
methods have been deprecated.

The <span class="title-ref">shell_out_compact</span> API has been
migrated into <span class="title-ref">shell_out</span>, so those
methods can be renamed. The functionality of <span
class="title-ref">shell_out_compact_timeout</span> and <span
class="title-ref">shell_out_with_timeout</span> have been migrated
into <span class="title-ref">shell_out</span> for internal resources,
and will be migrated into custom resources and LWRPs in Chef-15, in the
meantime consumers should use shell_out with a <span
class="title-ref">timeout: new_resource.timeout</span> option. The
functionality of <span
class="title-ref">shell_out_with_systems_locale</span> has been
replaced by the <span class="title-ref">default_env: false</span> flag.

The "banged" versions of those APIs (e.g. <span
class="title-ref">shell_out_compact!</span>) changes identically to
use <span class="title-ref">shell_out!</span>.

## Example

The following code examples need to be changed to the corresponding code
below:

``` ruby
shell_out_compact("rpm", "-qa")
shell_out_compact_timeout("rpm", "-qa")
shell_out_with_timeout("rpm", "-qa")
shell_out_with_systems_locale("rpm", "-qa")
```

## Remediation

You now need to use shell_out! instead:

``` ruby
shell_out("rpm", "-qa")
shell_out("rpm", "-qa", timeout: new_resource.timeout)
shell_out("rpm", "-qa", timeout: new_resource.timeout)
shell_out("rpm", "-qa", default_env: false)
```
