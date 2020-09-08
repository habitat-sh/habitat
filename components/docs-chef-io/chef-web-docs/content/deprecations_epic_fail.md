+++
title = "Deprecation: epic_fail (CHEF-24)"
draft = false
robots = "noindex"


aliases = "/deprecations_epic_fail.html"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/deprecations_epic_fail.md)

The original name for the `ignore_failure` property in resources was
`epic_fail`. Our documentation hasn't referred to `epic_fail` for years
and out of the 3500 cookbooks on the Supermarket only one uses
`epic_fail`. In Chef Client 14 we will remove the `epic_fail` property
entirely.

The [Cookstyle](/workstation/cookstyle/) cop
[ChefDeprecations/EpicFail](https://github.com/chef/cookstyle/blob/master/docs/cops_chefdeprecations.md#chefdeprecationsepicfail)
has been introduced to detect and autocorrect this deprecation.

## Remediation

Replace any usage of `epic_fail` with `ignore_failure`.
