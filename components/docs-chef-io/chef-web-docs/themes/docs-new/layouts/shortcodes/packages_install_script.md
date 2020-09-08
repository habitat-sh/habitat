The Chef Software install script can be used to install any Chef Software, including things like Chef Infra Client, Chef Infra Server, Chef Inspec. This script does the following:

-   Detects the platform, version, and architecture of the machine on
    which the installer is being executed
-   Fetches the appropriate package, for the requested product and
    version
-   Validates the package content by comparing SHA-256 checksums
-   Installs the package