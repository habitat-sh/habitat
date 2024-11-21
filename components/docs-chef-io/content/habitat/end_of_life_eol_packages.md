+++
title = "End-of-Life (EOL) Packages"
description = ""
gh_repo = "habitat"

[menu]
  [menu.habitat]
    title = "End-of-Life (EOL) Packages"
    identifier = "habitat/packages/support/package_support/end_of_life_eol_packages"
    parent = "habitat/packages/support"
    weight = 12
+++

End-Of-Life (EOL) packages refer to packages that have reached the end of their support lifecycle. These packages are no longer maintained or updated, and they are excluded from new Long-Term Support (LTS) releases to minimize disruption for users.

For example, if a package like core/openssl11 reaches the end of its support lifecycle, it will not be included in the subsequent LTS release channel. This approach allows customers to transition to the latest LTS channel at their convenience while maintaining the previous LTS channel, including the older packages for those who need them. However, the older packages will not receive support or be recommended for active use.

This strategy ensures that deprecations do not adversely affect customers, granting the package management team the flexibility to implement significant changes without disrupting user workflows.
