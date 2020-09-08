+++
title = "AWS Marketplace"
draft = false

aliases = ["/aws_marketplace.html"]

[menu]
  [menu.infra]
    title = "AWS Marketplace"
    identifier = "chef_infra/setup/integrations/aws_marketplace.md AWS Marketplace"
    parent = "chef_infra/setup/integrations"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/aws_marketplace.md)

Chef provides Amazon Machine Images (AMIs) for Chef Automate and Chef
Infra Server that can be launched from the [AWS
Marketplace](https://aws.amazon.com/marketplace/seller-profile/ref=srh_res_product_vendor?ie=UTF8&id=e7b7691e-634a-4d35-b729-a8b576175e8c).
Hourly metered billing and Bring Your Own License (BYOL) options are
available.

## Metered AMI

The Chef Automate Amazon Machine Image (AMI) is preinstalled with Chef
Automate and Chef Infra Server on a single instance. When using the
metered billing AMI, an hourly aggregate of your Chef Automate usage is
calculated and billed through your Amazon Web Services (AWS) account.
Follow the steps in the sections below to use the Chef Automate metered
billing AMI:

### Accept software terms

{{% accept_aws_marketplace_terms %}}

### Create S3 bucket and access role

If you wish to use Chef Automate's built-in S3 backup support, or if you
want to bring your own license, complete the following steps:

1.  Navigate to the [S3
    Console](https://s3.console.aws.amazon.com/s3/home) and create an S3
    bucket in the region where you intend to launch the Chef Automate
    AMI.
2.  Copy the S3 bucket ARN.
3.  Navigate to the [IAM Role section in the AWS
    console](https://console.aws.amazon.com/iam/home#roles).
4.  Create an access policy for your bucket that allows listing,
    getting, putting, deleting and multi-part uploads to your bucket
    ARN. You can use the following example with your bucket ARN in the
    Resource arrays:


``` json
{
  "Statement": [
    {
      "Action": [
        "s3:ListBucket",
        "s3:GetBucketLocation",
        "s3:ListBucketMultipartUploads",
        "s3:ListBucketVersions"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:s3:::yourbucket"
      ]
    },
    {
      "Action": [
        "s3:GetObject",
        "s3:PutObject",
        "s3:DeleteObject",
        "s3:AbortMultipartUpload",
        "s3:ListMultipartUploadParts"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:s3:::yourbucket/*"
      ]
    }
  ],
  "Version": "2012-10-17"
}
```

1.  Create an IAM role for your instance.
2.  Attach the S3 bucket access policy to the role.

### Launch the Metered AMI

1.  Navigate back to the Chef Automate [product
    page](https://aws.amazon.com/marketplace/pp/B01AMIH01Q) and continue
    to the launch wizard.
2.  Click the 'Launch with EC2 Console' button next to the desired
    region.
3.  Configure the Amazon EC2 instance type, Amazon Virtual Private Cloud
    (VPC) settings, SSH key pair, IAM Role and assign [a public IP
    address](http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-instance-addressing.html#concepts-public-addresses).
4.  Increase the root volume size to a minimum of 30GB. You might
    consider even larger if you have hundreds of nodes or need to
    maintain months of node visibility data.
5.  Launch the Amazon Machine Image (AMI).
6.  [Install Chef
    Workstation](#install-chef-workstation).

## Bring Your Own License (BYOL) AMI

The Chef Automate Amazon Machine Image (AMI) is preinstalled with Chef
Automate and Chef Infra Server on a single instance. The BYOL image
includes a 30 day trial license, but it can also be configured to use an
existing Chef Automate license that you have procured from Chef. Follow
the steps in the sections below to use the Chef Automate metered billing
AMI:

### Accept software terms

{{% accept_aws_marketplace_terms %}}

### Create S3 bucket and access role

If you wish to use Chef Automate's built-in S3 backup support, or if you
want to bring your own license, complete the following steps:

1.  Navigate to the [S3
    Console](https://s3.console.aws.amazon.com/s3/home) and create an S3
    bucket in the region where you intend to launch the Chef Automate
    AMI.

2.  Select your bucket in the console and upload your Chef Automate
    `delivery.license` file. Ensure that you've restricted access to the
    file, and that it is not publicly readable. If you do not have a
    license, skip this step.

    {{< note spaces=4 >}}

    Placing your license file in S3 is not a requirement for using the
    BYOL functionality, the instance just needs a fully-qualified URL to
    the license file. For the sake of these instructions we're using S3
    to safely store the file and make it accessible to the Chef Automate
    instance.

    {{< /note >}}

3.  Copy the S3 bucket ARN.

4.  Navigate to the [IAM Role section in the AWS
    console](https://console.aws.amazon.com/iam/home#roles).

5.  Create an access policy for your bucket that allows listing,
    getting, putting, deleting and multi-part uploads to your bucket
    ARN. You can use the following example with your bucket ARN in the
    Resource arrays:

<!-- -->

``` json
{
  "Statement": [
    {
      "Action": [
        "s3:ListBucket",
        "s3:GetBucketLocation",
        "s3:ListBucketMultipartUploads",
        "s3:ListBucketVersions"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:s3:::yourbucket"
      ]
    },
    {
      "Action": [
        "s3:GetObject",
        "s3:PutObject",
        "s3:DeleteObject",
        "s3:AbortMultipartUpload",
        "s3:ListMultipartUploadParts"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:s3:::yourbucket/*"
      ]
    }
  ],
  "Version": "2012-10-17"
}
```

1.  Create an IAM role for your instance.
2.  Attach the S3 bucket access policy to the role.

### Launch the BYOL AMI

1.  Navigate back to the Chef Automate [product
    page](https://aws.amazon.com/marketplace/pp/B01AMIH01Q) and continue
    to the launch wizard.

2.  If you're using your own license, create and copy a pre-signed link
    with the AWS command line tools and save it. For example:

    ``` bash
    aws s3 presign yourbucket/delivery.license
    ```

3.  Configure all fields in the CloudFormation template. Use the
    pre-signed license URL for the `LicenseUrl` field.

4.  Associate the IAM role for backup access.

5.  Run the CloudFormation template to create the Chef Automate
    instance.

## Install Chef Workstation

While the Amazon Machine Images (AMI) for Chef Automate is provisioning,
download and install Chef Workstation. Chef Workstation is a collection
of tools and libraries that are packaged together to make it easy to
develop cookbooks and resources for a Chef / Chef Automate environment.
You'll need this to interact with Chef Automate and Chef Infra Server
from the command line.

## Configure Chef Automate

After the instance has been provisioned and initial configuration has
completed (usually 10 to 13 minutes) finish configuring Chef Automate
and Chef Infra Server.

1.  Access the intial configuration page by loading `/biscotti/setup`
    route. Build the URL by prepending `https://` and appending
    `/biscotti/setup` to the IP address or public hostname that was
    automatically assigned to the instance when the Amazon Machine
    Images (AMI) was launched. For example,
    `https://<fqdn>/biscotti/setup`. If you used the BYOL image, the
    CloudFormation stack will have the setup URL in the `Outputs`
    section.

    {{< note spaces=4 >}}

    {{% notes_chef_aws_ssl %}}

    {{< /note >}}

2.  Use the AWS console or command line tools to determine the Instance
    ID of your Chef Automate instance. The instance ID is required for
    authorization to access the setup page.

3.  Fill out the setup form and submit it.

4.  Follow the link and log into the Chef Automate webui.

## Configure the workstation

1.  Download and extract the `starter_kit.zip` file to a directory on
    the workstation. Open a command prompt and change into the
    `chef-repo` directory extracted from the starter kit. For example:

    ``` bash
    cd ~/Downloads
    unzip starter_kit.zip
    cd starter_kit/chef-repo
    ```

2.  {{% install_aws_chef_server_knife_client_list %}}

## Configure backups

Follow the Workflow
[instructions](/delivery_server_backup/#s3-backups) for configuring
backups.

## Troubleshooting

### Required ports

The following are recommended security group rules for Chef Automate
from the AWS Marketplace:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 87%" />
</colgroup>
<thead>
<tr class="header">
<th>Port</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>443</td>
<td>HTTPS for Chef Automate webui</td>
</tr>
<tr class="even">
<td>8989</td>
<td>Git access for the delivery-cli and workflow</td>
</tr>
<tr class="odd">
<td>22</td>
<td>SSH</td>
</tr>
</tbody>
</table>

### Change the hostname

To update the hostname, do the following:

1.  Run `sudo -i` to gain administrator privileges.

2.  Run `chef-marketplace-ctl hostname` to view the current hostname.

3.  Configure the `api_fqdn` in `/etc/chef-marketplace/marketplace.rb`

    ``` none
    echo 'api_fqdn "<new.fully.qualified.hostname.com>"' | sudo tee -a /etc/chef-marketplace/marketplace.rb
    ```

4.  Run `chef-marketplace-ctl reconfigure` to update Chef Automate and
    Chef Infra Server configuration.

5.  Run `chef-server-ctl stop` to stop Chef Infra Server.

6.  Run `automate-ctl stop` to stop Chef Automate.

7.  Run
    `chef-marketplace-ctl hostname <new.fully.qualified.hostname.com>`
    to update the hostname.

8.  Run `automate-ctl reconfigure` to ensure Chef Automate has been
    correctly configured with the new hostname.

9.  Run `chef-server-ctl reconfigure` to ensure Chef Infra Server has
    been correctly configured with the new hostname.

10. Run `automate-ctl restart` to restart Chef Automate

11. Run `chef-server-ctl restart` to restart Chef server

### Change instance size

To edit the Amazon Machine Images (AMI) instance size, do the following:

1.  Login using SSH to access the Chef Automate instance. Use the SSH
    key pair and the IP address or public hostname that was
    automatically assigned when the Amazon Machine Images (AMI) was
    launched. The default user is `ec2-user`. For example:

    ``` bash
    ssh -i /path/to/ssh_key.pem ec2-user@<instance IP address>
    ```

2.  Stop the Chef Infra Server services:

    ``` bash
    sudo chef-server-ctl stop
    ```

3.  Stop then Chef Automate services:

    ``` bash
    sudo automate-ctl stop
    ```

4.  Navigate to the Amazon Web Services (AWS) instance in the AWS
    Management Console.

5.  From the **Actions** dropdown, select **Instance State**, and then
    **Stop**.

6.  After the instance transitions to **Stopped**, edit the instance
    size. From the **Actions** dropdown, select **Instance Settings**,
    and then **Change Instance Type**.

7.  From the dropdown, select the desired instance size, and then click
    **Apply**.

8.  From the **Actions** dropdown, select **Instance State**, and then
    click **Start**.

9.  After the instance has started it will have a **new public IP
    address and public DNS**.

10. Use SSH to log into the new instance. Use the SSH key pair and new
    IP address:

    ``` bash
    ssh -i /path/to/ssh_key.pem ec2-user@<instance IP address>
    ```

11. Follow the [instructions for changing the
    hostname](#change-automate-hostname)

12. Verify that you can login to Chef Automate webui by navigating to
    `https://<YOUR NEW PUBLIC DNS>/e/default`.

    {{< note spaces=4 >}}

    {{% notes_chef_aws_ssl %}}

    {{< /note >}}

13. Open a command prompt and change into your `chef-repo` directory.

14. {{% install_update_aws_knife_rb %}}

15. Open `.chef/pivotal.rb` in a text editor and modify the
    `chef_server_url` and `chef_server_root` with your new public DNS.
    For example:

    ``` bash
    vim ~/chef-repo/.chef/pivotal.rb
    ```

    will open a `pivotal.rb` file similar to:

    ``` ruby
    node_name        "pivotal"
    chef_server_url  "<YOUR NEW PUBLIC DNS>"
    chef_server_root "<YOUR NEW PUBLIC DNS>"
    client_key       ::File.join(::File.dirname(__FILE__), "pivotal.pem")
    ```

16. {{% install_aws_chef_server_knife_ssl_fetch %}}

17. {{% install_aws_chef_server_knife_client_list %}}

18. Update the `/etc/chef/client.rb` on all of your nodes to use the new
    public DNS. For example:

    ``` bash
    knife ssh name:* 'sudo sed -ie "s/chef_server_url.*/chef_server_url 'https://ec2-52-6-31-230.compute-1.amazonaws.com/organizations/your_org'/"' /etc/chef/client.rb
    ```

    Replace `ec2-52-6-31-230.compute-1.amazonaws.com` with your new
    public DNS name and `your_org` with your organization name.

### Upgrade Chef Automate

The Chef Automate Amazon Machine Images (AMI) can perform in-place
upgrades of all of the pre-bundled software. This makes it easy to stay
up-to-date with the latest version of Chef Automate, the Chef Infra
Server and Chef Marketplace, while not requiring data to be migrated to
the latest published Amazon Machine Images (AMI).

There are three options: upgrade Chef Automate, upgrade Chef Infra
Server, upgrade Chef Marketplace; upgrade everything.

To upgrade, do one of the following:

-   Upgrade the Chef Automate package by using the following command:

    ``` bash
    sudo chef-marketplace-ctl upgrade --automate
    ```

    {{< note spaces=4 >}}

    Chef Automate and Chef Infra Server services will be unavailable
    while the software is updated.

    {{< /note >}}

-   Upgrade the Chef Infra Server package by using the following
    command:

    ``` bash
    sudo chef-marketplace-ctl upgrade --server
    ```

    {{< note spaces=4 >}}

    Chef Infra Server services will be unavailable while the software is
    updated.

    {{< /note >}}

-   Upgrade the Chef Marketplace package by using the following command:

    ``` bash
    sudo chef-marketplace-ctl upgrade --marketplace
    ```

-   Upgrade all the installed packages by using the following command:

    ``` bash
    sudo chef-marketplace-ctl upgrade -y
    ```

### Migrate to Chef Automate on AWS

The process of migrating from an existing Chef Infra Server installation
to the Amazon Machine Images (AMI) differs depending on which software
version is being used and the location in which it is deployed. In all
scenarios, data is first migrated to the latest Chef Infra Server
schema, after which it is migrated to the Amazon Machine Images (AMI).

-   Verify that the latest version of the Chef Infra Server is installed
    by using the platform package manager:
    `rpm -qa | grep chef-server-core` and compare the result to the
    latest version available on the [downloads
    site](https://downloads.chef.io/). If this is not the latest
    version, download the package, and then
    [upgrade](/upgrade_server/#from-chef-server-12) to the latest
    version.

After verifying that your existing Chef Infra Server installation is up
to date, do the following to migrate to the Amazon Machine Images (AMI)
instance:

1.  Backup the data on the Chef Infra Server using `knife ec backup`.
    This method will export all of your existing Chef Infra Server data
    as JSON. We'll then re-import the same data into a new Chef Automate
    cluster. We use the JSON based backup and restore procedure because
    the Chef Infra Server data on the Chef Automate Marketplace AMI is
    stored in shared databases so copying of binary files won't work.

    ``` bash
    mkdir -p /tmp/chef-backup
    /opt/opscode/embedded/bin/knife ec backup /tmp/chef-backup --with-user-sql --with-key-sql
    tar -czvf chef-backup.tgz -C /tmp/chef-backup
    ```

1.  Copy the resulting tarball to your Amazon Machine Images (AMI)
    instance:

    ``` bash
    scp /tmp/chef-backup.tgz ec2-user@<MARKETPLACE AMI IP ADDRESS>:/tmp/
    ```

1.  Login to the Amazon Machine Images (AMI) and ensure that it is
    running the latest version of the Chef Infra Server:

    ``` bash
    chef-marketplace-ctl upgrade -y
    ```

1.  Reconfigure Chef Automate and the Chef Infra Server:

    ``` bash
    sudo automate-ctl reconfigure
    sudo chef-server-ctl reconfigure
    ```

1.  Restore the backup:

    ``` bash
    mkdir -p /tmp/chef-backup
    mv /tmp/chef-backup.tgz /tmp/chef-backup
    cd /tmp/chef-backup
    tar -ztf chef-backup.tgz
    /opt/opscode/embedded/bin/knife ec restore /tmp/chef-backup --with-user-sql --with-key-sql
    ```

1.  {{% install_update_aws_knife_rb %}}

1.  {{% install_aws_chef_server_knife_ssl_fetch %}}

1.  {{% install_aws_chef_server_knife_client_list %}}

1.  Update the `/etc/chef/client.rb` on all of your nodes to use the new
    public DNS. For example:

    ``` none
    knife ssh name:* 'sudo sed -ie "s/chef_server_url.*/chef_server_url 'https://ec2-52-6-31-230.compute-1.amazonaws.com/organizations/your_org'/" /etc/chef/client.rb
    ```

    Replace `ec2-52-6-31-230.compute-1.amazonaws.com` with your new
    public DNS name and `your_org` with your organization name.
