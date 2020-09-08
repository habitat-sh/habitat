+++
title = "Node Integrations"

date = 2018-05-22T18:01:36-07:00
draft = false
[menu]
  [menu.automate]
    title = "Node Integrations"
    parent = "automate/settings"
    identifier = "automate/settings/node_integrations.md Node Integrations"
    weight = 40
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/node_integrations.md)

Set up Chef Automate to detect and monitor the nodes in your AWS EC2 and Azure accounts by providing your credentials and creating a node manager. Chef Automate creates a node reference for each instance in your account. Associate your EC2 and Azure instances with ssh and WinRM credentials using tags--the values support wildcard matching--in your node manager. Run scan jobs with your node manager reference and you're suddenly running an `inspec exec` across your instances. Every two hours Chef Automate queries your AWS or Azure account to see the current state of all your nodes to determine if they are running, stopped, or terminated, and then updates Chef Automate accordingly. If the node manager finds an instance that used to be running and reachable but which no longer is (the node stopped, terminated, or is in a transition state), the node manager updates the status of that node in Chef Automate accordingly.

Access the _Node Integrations_ page from the **Settings** tab.

![Node Integrations](/images/automate/node-integrations.png)

## Add an AWS EC2 Node Manager

Set up Chef Automate to detect and scan the nodes in your AWS EC2 account by providing your AWS Credentials and creating an _AWS EC2 Node Manager_ from the **Node Credentials** page in the **Settings** tab. Chef Automate requires your information to detect the nodes in your AWS EC2 account. Chef Automate creates a node reference for each EC2 instance in your account and collects all of the tags associated with each instance.

Inspec 2+ supports running scan jobs against your AWS account configuration, such as CloudWatch or IAM, [see more here](https://docs.chef.io/inspec/resources/#aws). Set up Chef Automate to run these scan jobs by providing your AWS Credentials and creating an _AWS API Node Manager_ in the **Node Integrations** page in the **Settings**  tab.

To create an AWS EC2 Node Manager, you need the following information:

1. A name for your manager
1. Your AWS credentials (access key ID and secret access key)
1. The default region to target (if `us-east-1` is not desired)

![Chef Automate Create AWS-EC2 Manager](/images/automate/node-integrations-full.png)

You can use associate your `ssh` or `WinRM` credentials with your EC2 instances using tag keys or values, using the option at the bottom of the screen. This feature supports wildcard matching, which is useful for grouping nodes. Chef Automate detects your nodes immediately after any update to the Node Manager, and maintains a current list of your node status. The following example uses tag with the key 'Name' and the value 'vj-' to associate those nodes with the 'ssh ec22' credential.

Filter instances for scanning by specifying either regions or tags by their keys and values.

![Chef Automate Instance Credentials](/images/automate/instance-credentials.png)

### AWS EC2 Node Discovery

The service makes these API calls:

* `STS-GetCallerIdentity`
* `EC2-DescribeRegions`
* `EC2-DescribeInstances`
* `EC2-DescribeInstanceStatus`
* `IAM-ListAccountAliases`

Chef Automate's Node Manager discovers EC2 instances using polling and scan jobs.

Polling
: Chef Automate's Node Manager calls out to the AWS `DescribeInstanceStatus` API every two hours and discovers the state of all the instances in the account. If the node manager finds any instances that aren't in its database, it adds them. This sometimes results in "bare bones info" and stopped instances in the database. The node manager updates node information in the database after an instance returns to a running state and a scan job has run on the node.

Scan Jobs
: Whenever a scan job is triggered, the node manager queries the AWS API for all nodes. Any scan reports created for nodes that are not already in the database results in creating a new node in the database.

### Create a Scan Job Targeting Your AWS Account Configuration

![Chef Automate Create AWS-API Scan Job](/images/automate/create-aws-api-scanjob.png)

### AWS API Scanning Endpoints

The service makes calls to these APIs:

* `STS-GetCallerIdentity`
* `SEC2-DescribeRegions`
* `IAM-ListAccountAliases`
* `IAM-GetAccountSummary`
* `IAM-ListUsers`

Permissions: You'll need at least a global read permission; `arn:aws:iam::aws:policy/ReadOnlyAccess`

## AWS Credential-less Scanning with Chef Automate

For users running Chef Automate 2 in EC2, we invite you to try out our "AWS-EC2 Credential-less Scanning"!
Please note that credential-less scanning is not supported for AWS GovCloud.

### Ensure Minimum Permissions

Ensure the policy attached to the role used by the instance you have Chef Automate running on has at least these permissions:

```bash
{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Effect": "Allow",
            "Action": [
                "ec2:DescribeInstances",
                "ssm:*",
                "ec2:DescribeRegions",
                "sts:GetCallerIdentity",
                "ec2:DescribeInstanceStatus",
                "iam:ListAccountAliases",
                "iam:GetAccountSummary",
                "iam:ListUsers",
            ],
            "Resource": "*"
        }
    ]
}
```

{{< note >}}
`"ssm:*"` uses a wildcard match on the AWS EC2 Systems Manager (SSM); You may wish to use a more restrictive policy.
{{< /note >}}

### Install AWS EC2 Systems Manager on Instances

Please follow the instructions on [AWS](https://docs.aws.amazon.com/systems-manager/latest/userguide/ssm-agent.html)

### Enable AWS EC2 Systems Manager on Instances

In order to use the SSM scan job functionality, your instances must have access to `AmazonEC2RoleforSSM`, or `arn:aws:iam::aws:policy/service-role/AmazonEC2RoleforSSM`.

### Adding an AWS EC2 Node Manager Using IAM Credentials

When running in EC2, AWS has the ability to use the IAM role associated with your instance to create and use temporary credentials for accessing the AWS API. If you enable this feature, then you won't need to provide credentials for your AWS account. You will only be required to provide a name for your node manager. Chef Automate creates a node reference for each EC2 instance in your account, collecting all tags associated with each instance. Chef Automate calls the Amazon System Manager (SSM) to describe instance information and to get ping status for the SSM agent on all instances. A detect job is *not* run on the instances; all instances with an SSM ping status of "Online" will be marked as reachable.

### Create a Scan Job Targeting Your AWS EC2 Instances using AWS SSM

The `ssm` Scan Job:

1. Installs the latest stable InSpec from `packages.chef.io`
1. Executes InSpec locally, providing InSpec with the `fqdn` of Chef Automate and a data collector token, so each instance reports directly back to Chef Automate

Your Automate instance must be reachable (open to incoming traffic) from the instances being scanned in order for the SSM scanning to work. You can filter the instances to be scanned by specifying tag key/value matches or regions.

![Chef Automate Create AWS-EC2 Scan Job](/images/automate/create-aws-ec2-scanjob.png)

### AWS Credential-less Scanning

The service makes these API calls:

* `STS-GetCallerIdentity`
* `EC2-DescribeRegions`
* `EC2-DescribeInstances`
* `EC2-DescribeInstanceStatus`
* `SSM-DescribeInstanceInformation`
* `SSM-SendCommand`
* `SSM-ListCommands`

## Azure VM Scanning with Chef Automate

Set up Chef Automate to detect and scan the nodes in your Azure account by providing your Azure Credentials and creating an _Azure VM Node Manager_. To add an Azure VM Node Manager, navigate to the [_Node Integrations_]({{< relref "node_integrations.md" >}}) page in the Settings tab, select `Create Integration`, and you should see _Azure_ as one of your node management service options.

{{< note >}}
We do not support Azure Government Cloud.
{{< /note >}}

### Adding an Azure VM Node Manager

When creating an Azure VM Node Manager, you will be required to provide:

1. A name for your manager
1. Your Azure credentials (client ID, client secret, and tenant ID)

This information is required to detect the nodes in your Azure account. Chef Automate creates a nodes reference for each VM in your account, reading in all tags associated with each instance. Chef Automate detects your nodes immediately after any update to the Node Manager in order to maintain a current list of your node status. The following example uses a tag with the key 'Name' and the value 'vj-' to associate those nodes with the 'ssh ec2' credential.

![Chef Automate Create Azure-VM Manager](/images/automate/create-azure-vm-mgr.png)

Chef Automate uses Azure's RunCommand functionality to run scan jobs on instances without needing `ssh` and `WinRM` credentials. In order for this functionality to work, the Automate instance must be reachable (open to incoming traffic) from the instances being scanned.

You also have the option of using the traditional `ssh` and `WinRM` scanning by providing such credentials for the VMs. At the bottom of the screen there is an option to associate `ssh` and `WinRM` credentials with your VMs using tag keys and values (supports wildcard match) to group nodes. If the tag key matches, the value of the tag is evaluated against the user provided value:

```txt
- foo* -> tag value has a prefix of "foo" == match
- *foo -> tag value has a suffix of "foo" == match
- *foo* -> tag value contains substring "foo" == match
- foo -> tag value is exactly "foo" == match
```

Chef Automate detects your nodes immediately after any update to the Node Manager, keeping a current view of your nodes' reachability.

![Chef Automate Instance Credentials](/images/automate/instance-credentials.png)

### Create a Scan Job Targeting Your Azure VMs

Filter the regions for the scan job by specifying regions to include or exclude.

Filter instances for scanning by specifying either regions or tags by their keys and values.

![Chef Automate Create Azure-VM Scan Job](/images/automate/create-azure-vm-scanjob.png)

## Use Case: Azure Account Scanning with Chef Automate

Inspec 2+ supports running scan jobs against your Azure account configuration, such as network security groups and ad users. See [Azure resources](https://docs.chef.io/inspec/resources/#azure) for more information.
Set up Chef Automate to run these scan jobs by providing your Azure credentials and creating an _Azure API Node Manager_.

### Adding an Azure API Node Manager

When creating an Azure API Node Manager, you will be required to provide:

1. A name for your manager
2. Your Azure credentials (client ID, client secret, and tenant ID)

This information is required to detect all subscriptions available to your Azure account. Chef Automate creates a nodes reference for each subscription in your account.

### Create a Scan Job Targeting Your Azure Account Configuration

From the **Scan Jobs** tab, select the "Create new job" button.

Filter the regions for the scan job by specifying regions to include or exclude.

![Chef Automate Create Azure-API Scan Job](/images/automate/create-azure-api-scanjob.png)

## Google Cloud Platform Account Scanning with Chef Automate

Run scans against your GCP account infrastructure using Chef Automate. Set up Chef Automate to detect and scan the nodes in your Google Cloud Platform (GCP) account by providing your GCP Credentials and creating a _GCP Node Manager_. To create a GCP Node Manager, navigate to _Node Integrations_, select `Create Integration`, and you should see _Google Cloud_ as one of your node management service options. See the Chef InSpec documentation for more infomation about [GCP resources](https://docs.chef.io/inspec/resources/#gcp).

To run a GCP scan in Chef Automate:

1. Add a GCP-API Node Manager using a service account json credential
1. The service adds a node reference in the database for the project tied to the service account credential
1. Execute a profile against the project reference with a scan job

This information is required to detect all subscriptions available to your Azure account. Chef Automate creates a nodes reference for each subscription in your account.

Note: The service account json credential requires the following fields:
`type`, `project_id`, `client_id`, `private_key_id`, `private_key`, `client_email`, `auth_uri`, `token_uri`, `auth_provider_x509_cert_url`, `client_x509_cert_url`

![Chef Automate Create GCP-API Integration](/images/automate/add-gcp-api-integration.png)

Set up Chef Automate to detect and monitor the nodes in your AWS EC2 and Azure accounts by providing your credentials in the [Node Credentials]({{< relref "node_credentials.md" >}}) page in the Settings tab and creating a node manager. Chef Automate creates a node reference for each instance in your account. Associate your EC2 and Azure instances with ssh and WinRM credentials using tags--the values support wildcard match--in your node manager. Run scan jobs with your node manager reference and you're suddenly running an `inspec exec` across your instances. Every two hours Chef Automate queries your AWS or Azure account to see the current state of all your nodes, if they are running, stopped, or terminated, and then updates Chef Automate accordingly. If the node manager finds an instance that used to be running and reachable, but which no is--if the node is stopped, terminated, or a transition state--it updates the status of that node in Chef Automate accordingly.

### Create a Scan Job Targeting Your GCP Account Configuration

From the **Scan Jobs** tab, select the "Create new job" button.

Filter the regions for the scan job by specifying regions to include or exclude.

![Chef Automate Create GCP-API Scan Job](/images/automate/add-gcp-api-scanjob.png)
