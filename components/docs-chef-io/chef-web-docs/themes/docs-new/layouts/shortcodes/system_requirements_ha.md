**Frontend Requirements**

-   4 cores (physical or virtual)
-   4GB RAM
-   20 GB of free disk space (SSD if on premises, Premium Storage in
    Microsoft Azure, EBS-Optimized GP2 in AWS)

**Backend Requirements**

-   2 cores (physical or virtual)
-   8GB RAM
-   50 GB/backend server (SSD if on premises, Premium Storage in
    Microsoft Azure, EBS-Optimized GP2 in AWS)

<div class="admonition-warning">

<p class="admonition-warning-title">Warning</p>

<div class="admonition-warning-text">

The Chef Infra Server MUST NOT use a network file system of any
type---virtual or physical---for backend storage. The Chef Infra Server
database operates quickly. The behavior of operations, such as the
writing of log files, will be unpredictable when run over a network file
system.



</div>

</div>