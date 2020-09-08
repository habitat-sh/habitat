Policy maps business and operational requirements, process, and workflow
to settings and objects stored on the Chef Infra Server:

-   Roles define server types, such as "web server" or "database server"
-   Environments define process, such as "dev", "staging", or
    "production"
-   Certain types of data---passwords, user account data, and other
    sensitive items---can be placed in data bags, which are located in a
    secure sub-area on the Chef Infra Server that can only be accessed
    by nodes that authenticate to the Chef Infra Server with the correct
    SSL certificates
-   The cookbooks (and cookbook versions) in which organization-specific
    configuration policies are maintained