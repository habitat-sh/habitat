A node is any physical, virtual, or cloud device that is configured and
maintained by an instance of Chef Infra Client. Bootstrapping installs
Chef Infra Client on a target system so that it can run as a client and
sets the node up to communicate with a Chef Infra Server. There are two
ways to do this:

-   Run the `knife bootstrap` command from a workstation.
-   Perform an unattended install to bootstrap from the node itself,
    without requiring SSH or WinRM connectivity.