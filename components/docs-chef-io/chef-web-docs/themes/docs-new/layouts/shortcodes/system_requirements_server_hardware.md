All machines in a Chef Infra Server deployment have the following
hardware requirements. Disk space for standalone and backend servers
should scale up with the number of nodes that the servers are managing.
A good rule to follow is to allocate 2 MB per node. The disk values
listed below should be a good default value that you will want to modify
later if/when your node count grows. Fast, redundant storage
(SSD/RAID-based solution either on-prem or in a cloud environment) is
preferred.

**All Deployments**

-   64-bit architecture

**Standalone Deployments**

-   4 total cores (physical or virtual)
-   8 GB of RAM or more
-   5 GB of free disk space in `/opt`
-   5 GB of free disk space in `/var`

<div class="admonition-note">

<p class="admonition-note-title">Note</p>

<div class="admonition-note-text">

The RAM requirement can be lowered down to a minimum of 4 GB of RAM if
the number of Chef Infra Client runs (CCRs) per minute are low (i.e.
less than 33 CCRs/min). See [Capacity
Planning](/server_overview.html#capacity-planning) for more information
on how this metric affects scalability.



</div>

</div>

For a high availability deployment:

**General Requirements**

-   Three backend servers; as many frontend servers as required
-   1 x GigE NIC interface (if on premises)

{{ readFile "themes/docs-new/layouts/shortcodes/system_requirements_ha.md" | markdownify }}