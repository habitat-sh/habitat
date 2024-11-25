+++
title = "End-of-life (EOL) packages"

gh_repo = "habitat"

[menu]
  [menu.habitat]
    title = "EOL packages"
    identifier = "habitat/packages/support/end_of_life_eol_packages"
    parent = "habitat/packages/support"
    weight = 12
+++

End-of-life (EOL) packages refer to packages that have reached the end of their support lifecycle.
These packages are no longer maintained or updated, and they're excluded from long-term support (LTS) releases to minimize disruption for users.

For example, if a package like core/openssl11 reaches the end of its support lifecycle, it won't be included in the subsequent LTS release channel.
This approach allows customers to transition to the latest LTS channel at their convenience while maintaining the previous LTS channel,
including the older packages for those who need them.
However, the older packages won't receive support or be recommended for active use.

This strategy ensures that deprecations don't adversely affect customers, granting the package management team the flexibility to implement significant changes without disrupting user workflows.
