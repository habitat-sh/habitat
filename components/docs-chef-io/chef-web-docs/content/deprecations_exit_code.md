+++
title = "Deprecation: Old Exit Codes (CHEF-2)"
draft = false
robots = "noindex"


aliases = "/deprecations_exit_code.html"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/deprecations_exit_code.md)

In older versions of Chef Client, it was not possible to discern why a
chef run exited simply by examining the error code. This makes it very
tricky for tools such as Test Kitchen to reason about the status of a
Chef Client run. Starting in Chef Client 12.11, there are now well
defined exit codes that the Chef Client can use to communicate the
status of the run.

This deprecation was added in Chef Client 12.11. In Chef Client 13, only
the extended set of exit codes will be supported. For further
information on the list of defined error codes, please see [RFC 62,
which defines
them](https://github.com/chef/chef-rfc/blob/master/rfc062-exit-status.md).

## Remediation

If you have built automation that is dependent on the old behavior of
Chef Client, we strongly recommend updating it to support the extended
set of exit codes. However, it's still possible to enable the old
behavior. Add the setting

``` ruby
exit_status :disabled
```

to the Chef config file.
