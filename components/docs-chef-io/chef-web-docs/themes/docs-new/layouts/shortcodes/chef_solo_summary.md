chef-solo is a command that executes Chef Infra Client in a way that
does not require the Chef Infra Server in order to converge cookbooks.
chef-solo uses Chef Infra Client's [Chef local
mode](/ctl_chef_client.html#run-in-local-mode), and **does not support**
the following functionality present in Chef Infra Client / server
configurations:

-   Centralized distribution of cookbooks
-   A centralized API that interacts with and integrates infrastructure
    components
-   Authentication or authorization

<div class="admonition-note">

<p class="admonition-note-title">Note</p>

<div class="admonition-note-text">

chef-solo can be run as a daemon.



</div>

</div>

The chef-solo executable is run as a command-line tool.