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
These packages are no longer maintained or updated, and they're excluded from further long-term support (LTS) releases to minimize user disruption.
For example, OpenSSL 1.1.1 reached the end of it's support lifecycle, so the core/openssl11 package won't have a subsequent LTS release channel.

This approach allows customers to transition to the latest LTS channel at their convenience,
while maintaining packages released through the previous LTS channel for those who need them.

However, older packages won't receive support or be recommended for active use.

This strategy ensures that deprecations don't adversely affect customers, granting the package management team the flexibility to implement significant changes without disrupting user workflows.
