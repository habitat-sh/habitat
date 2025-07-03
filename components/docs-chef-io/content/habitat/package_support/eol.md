+++
title = "End-of-life (EOL) Habitat packages"

gh_repo = "habitat"

[menu]
  [menu.habitat]
    title = "EOL packages"
    identifier = "habitat/packages/support/end_of_life_eol_packages"
    parent = "habitat/packages/support"
    weight = 12
+++

End-of-life (EOL) packages are Chef-maintained packages that have reached the end of their support lifecycle. These packages are no longer maintained or updated and are excluded from future long-term support (LTS) releases to minimize user disruption. For example, since OpenSSL 1.1.1 has reached the end of its support lifecycle, the `core/openssl11` package won't be included in subsequent LTS release channels.

This approach allows customers to transition to the latest LTS channel at their own pace while retaining access to packages from the previous LTS channel if needed. However, older packages will no longer receive support and are not recommended for active use.

By excluding EOL packages from new LTS releases, this strategy ensures that deprecations don't negatively impact customers. It also provides the package management team with the flexibility to implement significant changes without disrupting user workflows.
