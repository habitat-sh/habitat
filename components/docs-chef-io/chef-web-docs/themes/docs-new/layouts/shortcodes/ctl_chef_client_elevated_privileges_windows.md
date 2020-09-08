On Microsoft Windows, running without elevated privileges (when they are
necessary) is an issue that fails silently. It will appear that Chef
Infra Client completed its run successfully, but the changes will not
have been made. When this occurs, do one of the following to run Chef
Infra Client as the administrator:

-   Log in to the administrator account. (This is not the same as an
    account in the administrator's security group.)

-   Run Chef Infra Client process from the administrator account while
    being logged into another account. Run the following command:

    ``` bash
    runas /user:Administrator "cmd /C chef-client"
    ```

    This will prompt for the administrator account password.

-   Open a command prompt by right-clicking on the command prompt
    application, and then selecting **Run as administrator**. After the
    command window opens, Chef Infra Client can be run as the
    administrator