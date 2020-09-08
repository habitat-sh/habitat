Before installing the Chef Infra Server, ensure that each machine has
the following installed and configured properly:

-   **Hostnames** --- Ensure that all systems have properly configured
    hostnames. The hostname for the Chef Infra Server must be a FQDN,
    have fewer than 64 characters including the domain suffix, be
    lowercase, and resolvable. See [Hostnames,
    FQDNs](/install_server_pre.html#hostnames) for more information
-   **FQDNs** --- Ensure that all systems have a resolvable FQDN
-   **NTP** --- Ensure that every server is connected to NTP; the Chef
    Infra Server is sensitive to clock drift
-   **Mail Relay** --- The Chef Infra Server uses email to send
    notifications for various events; a local mail transfer agent should
    be installed and available to the Chef server
-   **cron** --- Periodic maintenance tasks are performed using cron
-   **git** --- git must be installed so that various internal services
    can confirm revisions
-   **libfreetype and libpng** --- These libraries are required
-   **Apache Qpid** --- This daemon must be disabled on CentOS and Red
    Hat systems
-   **Required users** --- If the environment in which the Chef Infra
    Server will run has restrictions on the creation of local user and
    group accounts, ensure that the correct users and groups exist
    before reconfiguring
-   **Firewalls and ports** --- If host-based firewalls (iptables, ufw,
    etc.) are being used, ensure that ports 80 and 443 are open. These
    ports are used by the **nginx** service

In addition:

-   **Browser** --- Firefox, Google Chrome, Safari, or Internet Explorer
    (versions 9 or better)
-   **Chef Infra Client communication with the Chef Infra Server** The
    Chef Infra Server must be able to communicate with every node that
    will be configured by Chef Infra Client and every workstation that
    will upload data to the Chef Infra