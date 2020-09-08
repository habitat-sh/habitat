+++
title = "Chef Automate for Microsoft Azure"
draft = false
robots = "noindex"


aliases = ["/automate_azure.html"]

[menu]
  [menu.legacy]
    title = "Chef Automate for Microsoft Azure"
    identifier = "legacy/workflow/automate_azure.md Chef Automate for Microsoft Azure"
    parent = "legacy/workflow"
    weight = 60
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/automate_azure.md)

{{% cloud_azure_portal %}}

## Chef Automate

Chef provides a fully functional Chef Automate server that can be
launched from the Azure Marketplace. A single VM running Chef Automate
and Chef Infra Server will be provisioned and configured for you. The
only requirement is that you provide your own Chef Automate license at
the time of launch; otherwise, Chef Automate will run under a 30-day
free trial. If you would like to continue using the image after 30 days,
please contact <amp@chef.io> to obtain a new license.

1.  Sign in to the [Azure portal](https://portal.azure.com/) and
    authenticate using your Microsoft Azure account credentials.

2.  Click the **New** icon in the upper-left corner of the portal and
    search the Azure Marketplace for **Chef Automate**.

3.  Locate the Chef Automate product and click the **create** button to
    launch it through the Resource Manager.

4.  Complete each configuration step, agree to the software and
    marketplace terms and create the Chef Automate VM.

    {{< note spaces=4 >}}

    Remember the DNS label of the Chef Automate VM. It will be required
    to access the Chef Automate UI and Chef Infra Server.

    {{< /note >}}

5.  While the Chef Automate VM is being provisioned, download and
    install [Chef Workstation](/workstation/install_workstation/). Chef
    Workstation is a collection of tools ---Test Kitchen, ChefSpec,
    knife, delivery-cli, chef, chef-vault, Cookstyle, and more--- and
    libraries that are all packaged together to get your started with
    the Chef Automate workflow. You'll need this to interact with Chef
    Automate and Chef Infra Server from the command line.

6.  After the VM has been provisioned and the Resource Manager has
    completed (usually 10 to 13 minutes), finish configuring Chef
    Automate and Chef Infra Server. Access the initial configuration
    page by loading the `/biscotti/setup` route. Build the URL by
    prepending `https://` and appending `/biscotti/setup` to the DNS
    label that you chose when the VM was launched. For example,
    `https://<dns_label>.<location>.cloudapp.azure.com/biscotti/setup`
    or
    `https://chef-automate-01.eastus.cloudapp.azure.com/biscotti/setup`.

    {{< note spaces=4 >}}

    In order to use TLS/SSL for the Chef Automate Web UI, the VM will
    automatically create and use a self-signed SSL certificate. Modern
    web browsers typically warn about self-signed certificates during
    login; however, in this case, you can ignore the warning and accept
    the certificate.

    {{< /note >}}

7.  Fill out the setup form and submit it.

8.  Follow the link and log into the Chef Automate web UI.

9.  Extract the `starter_kit.zip` file to a directory on the
    workstation. Open a command prompt and change into the `chef-repo`
    directory extracted from the starter kit. For example:

    ``` bash
    cd ~/Downloads
    unzip starter_kit.zip
    cd starter_kit/chef-repo
    ```

10. Run `knife client list` to test the connection to the Chef Infra
    Server. The command should return `<orgname>-validator`, where
    `<orgname>` is the name of the organization that was created
    previously.

11. Optionally, bootstrap a node using knife.

    ``` bash
    cd ~/Downloads/starter_kit/chef-repo
    knife bootstrap chef-automate-01.eastus.cloudapp.azure.com --connection-user azure --sudo
    ```

### Migrate to Chef Automate on Microsoft Azure

The process of migrating from an existing Chef Infra Server installation
to the Chef Automate Azure VM image differs depending on which software
version is being used and the location in which it is deployed. In all
scenarios, data is first migrated to the latest Chef Infra Server
schema, after which it is migrated to the Chef Automate Azure VM image.

-   Verify that the latest version of the Chef Infra Server is installed
    by using the platform package manager. For example, in a terminal
    run `rpm -qa | grep chef-server-core` on CentOS/RHEL or
    `dpkg -l | grep chef-server-core` on Ubuntu/Debian. Compare the
    result to the latest version available on the [downloads
    site](https://downloads.chef.io/). If you are not using the latest
    version, download the package and then
    [upgrade](/upgrade_server/#from-chef-server-12) to the latest
    version.

After verifying that your existing Chef Infra Server installation is up
to date, do the following to migrate to the Chef Automate Azure VM:

1.  Backup the data on the Chef Infra Server using `knife ec backup`.
    This backup method will export all Chef Infra Server data into
    nested JSON files that can be used to import into the Chef Automate
    Azure VM. We must use the JSON-based backup and restore procedure
    because the Chef Infra Server data on the Chef Automate VM image is
    stored in a combined configuration with Chef Automate, therefore,
    using file based backups from your existing Chef Infra Server is not
    supported.

    {{< note spaces=4 >}}

    The Chef Infra Server services must be online for the entire
    duration of the backup.

    {{< /note >}}

    ``` bash
    mkdir -p /tmp/chef-backup
    /opt/opscode/embedded/bin/knife ec backup /tmp/chef-backup --with-user-sql --with-key-sql
    tar -czvf chef-backup.tgz -C /tmp/chef-backup
    ```

1.  Using the Admin Username and FQDN that you choose when provisioning
    the Chef Automate Azure VM from the Azure portal, copy the resulting
    tarball to your Azure VM:

    ``` bash
    scp /tmp/chef-backup.tgz <Admin Username>@<FQDN>:/tmp/
    ```

    {{< note spaces=4 >}}

    You can find the FQDN of the Automate VM by checking the deployment
    outputs in the Azure portal. Navigate to the resource group, click
    on the deployment history, select the main template and location the
    FQDN in the outputs section.

    {{< /note >}}

1.  Login to your Chef Automate VM and ensure that it is running the
    latest version of the Chef Infra Server:

    ``` bash
    chef-marketplace-ctl upgrade --server
    ```

1.  Reconfigure Chef Automate and the Chef Infra Server

    ``` bash
    sudo automate-ctl reconfigure
    sudo chef-server-ctl reconfigure
    ```

1.  Restore the backup

    ``` bash
    mkdir -p /tmp/chef-backup
    mv /tmp/chef-backup.tgz /tmp/chef-backup
    cd /tmp/chef-backup
    tar -ztf chef-backup.tgz
    /opt/opscode/embedded/bin/knife ec restore /tmp/chef-backup --with-user-sql --with-key-sql
    ```

1.  Update your workstation knife configuration. Open `.chef/config.rb`
    in a text editor and modify the `chef_server_url` with your Azure VM
    FQDN. For example:

    ``` bash
    vim ~/chef-repo/.chef/config.rb
    ```

    will open a `config.rb` file similar to:

    ``` ruby
    current_dir = ::File.dirname(__FILE__)
    log_level                :info
    log_location             $stdout
    node_name                'your_username'
    client_key               "#{current_dir}/your_username.pem"
    validation_client_name   'your_orgname-validator'
    validation_key           "#{current_dir}/your_orgname-validator.pem"
    chef_server_url          'https://<FQDN>/organizations/your_org'
    cookbook_path            ["#{current_dir}/../cookbooks"]
    ```

1.  {{% install_aws_chef_server_knife_ssl_fetch %}}

1.  {{% install_aws_chef_server_knife_client_list %}}

1.  Update the `/etc/chef/client.rb` on all of your nodes to use the new
    FQDN. For example:

    ``` none
    knife ssh name:* 'sudo sed -ie "s/chef_server_url.*/chef_server_url 'https://<FQDN>/organizations/your_org'/" /etc/chef/client.rb
    ```
