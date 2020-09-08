+++
title = "Chef and Terraform"
draft = false

[menu]
  [menu.infra]
    title = "Chef and Terraform"
    identifier = "chef_infra/getting_started/terraform.md Chef and Terraform"
    parent = "chef_infra/getting_started"
    weight = 60
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/terraform.md)

[Terraform](https://www.terraform.io/) is an open-source infrastructure-as-code provisioning tool from [HashiCorp](https://www.hashicorp.com/). Terraform allows you to write code to define and provision infrastructure for the cloud, virtual machines, and on-premises machines. Terraform is not a configuration management tool, it is responsible for deploying, maintaining, and destroying the infrastructure that servers and applications run on. When Terraform creates cloud or virtual servers, it uses [Provisioners](https://www.terraform.io/docs/provisioners/index.html) to enable configuration management to manage them. When Terraform talks to APIs to define or configure resources, it uses [Providers](https://www.terraform.io/docs/providers/index.html) to request those resources.

## Chef Infra Provisioner

The [Terraform Chef Provisioner](https://www.terraform.io/docs/provisioners/chef.html) bootstraps Terraform, provisioned with Chef Infra via SSH or WinRM, and configures them to work with a [Chef Infra Server](/server_overview/). Standard bootstrap options such as Chef Infra versions, secrets, proxies, and assigning run lists via Policyfiles or Roles and Environments are all supported. The referenced documentation provides a complete list of supported options and an example of usage. HashiCorp provides support for the [Terraform Chef Provisioner](https://www.terraform.io/docs/provisioners/chef.html) and it is not officially supported by Chef Software.

### Terraform and Chef Solo

If you are using [Chef Solo](/chef_solo/), you will most likely want to use the [Terraform remote-exec Provisioner](https://www.terraform.io/docs/provisioners/remote-exec.html) rather than the Terraform Chef Provisioner. The remote-exec Provisioner may be used to run a script or an inline set of commands on the newly created machine. Please refer to the [Terraform remote-exec Provisioner documentation](https://www.terraform.io/docs/provisioners/remote-exec.html) for further options and examples.


#### Example remote-exec inline

```
resource "aws_instance" "web" {
  # ...

  provisioner "remote-exec" {
    inline = [
      "wget -O /tmp/chef.rpm https://MYSERVER/chef_installers/chef-15.8.23-1.el7.x86_64.rpm",
      "rpm -Uvh /tmp/chef.rpm",
      "wget -O /tmp/base.tgz https://MYSERVER/policyfiles/base.tgz",
      "tar -C /tmp -xzf /tmp/base.tgz",
      "PWD=/tmp/base chef-client -z",
    ]
  }
}
```

## Chef Infra Provider

The [Terraform Chef Provider](https://www.terraform.io/docs/providers/chef/index.html) allows you to manage Chef Infra Server resources (nodes, data bags, etc.) using the Chef Infra Server API. Policyfiles, cookbooks, clients, and ACLs are not currently managed with the Provider. The [Terraform Chef Provider documentation](https://www.terraform.io/docs/providers/chef/index.html) provides a complete list of supported options and an example of usage. HashiCorp provides support for the Terraform Chef Provider and it is not officially supported by Chef Software.

## Additional Terraform Integrations

* The [Habitat Provisioner](https://www.habitat.sh/docs/habitat-and-other-software/#habitat-and-provisioning) may be used to install and load the Chef Habitat Supervisor and configure the services to be managed by the supervisor.
* [Kitchen Terraform](https://newcontext-oss.github.io/kitchen-terraform/) is a community [Test Kitchen](/kitchen/) driver that allows for multi-node testing.
* [InSpec-Iggy](https://github.com/mattray/inspec-iggy/) is a community [InSpec](/inspec/) plugin that generates InSpec compliance controls and profiles from Terraform `tfstate` files and AWS CloudFormation templates.
