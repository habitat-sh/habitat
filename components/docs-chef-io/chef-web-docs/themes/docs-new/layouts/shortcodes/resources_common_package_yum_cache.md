Yum automatically synchronizes remote metadata to a local cache. Chef
Infra Client creates a copy of the local cache, and then stores it
in-memory during a Chef Infra Client run. The in-memory cache allows
packages to be installed during a Chef Infra Client run without the need
to continue synchronizing the remote metadata to the local cache while
the Chef Infra Client run is in-progress.