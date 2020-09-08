+++
title = "Platform Overview"
draft = false

aliases = ["/platform_overview.html"]

[menu]
  [menu.overview]
    title = "Platform Overview"
    identifier = "overview/platform_overview.md Platform Overview"
    parent = "overview"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/platform_overview.md)

Chef is an automation company. Ever since it was founded in 2008, we've
been bringing together developers and system administrators with our
namesake product, Chef Infra. Over the years, what we mean by automation
has expanded. Today, Chef has a complete automation solution for both
infrastructure and applications that takes you all the way from
development to production. Here's the complete Chef solution.

<img src="/images/automate_architecture.svg" class="align-center" width="500" alt="image" />

## Chef Infra

{{% chef %}}

### Using Chef Workstation

Chef Workstation allows you to author cookbooks and administer your
infrastructure. Chef Workstation runs on the computer you use everyday,
whether it's Linux, macOS, or Windows.

Chef Workstation ships with Cookstyle, ChefSpec, Chef InSpec, and Test
Kitchen testing tools. With them, you can make sure your Chef Infra code
does what you intended before you deploy it to environments used by
others, such as staging or production.

When you write your code, you use resources to describe your
infrastructure. A resource corresponds to some piece of infrastructure,
such as a file, a template, or a package. Each resource declares what
state a part of the system should be in, but not how to get there. Chef
Infra handles these complexities for you. Chef Infra provides many
resources that are ready for you to use. You can also utilize resources
shipped in community cookbooks, or write your own resources specific to
your infrastructure.

A Chef Infra recipe is a file that groups related resources, such as
everything needed to configure a web server, database server, or a load
balancer. A Chef Infra cookbook provides structure to your recipes and,
in general, helps you stay organized.

The Chef Workstation includes other command line tools for interacting
with Chef Infra. These include knife for interacting with the Chef Infra
Server, and chef for interacting with your local chef code repository
(chef-repo).

### Uploading your code to Chef Infra Server

Once you're done developing and testing code on your local workstation,
you can upload it to the Chef Infra Server. The Chef Infra Server acts
as a hub for configuration data. It stores cookbooks, the policies that
are applied to the systems in your infrastructure and metadata that
describes each system. The knife command lets you communicate with the
Chef Infra Server from your workstation. For example, you use it to
upload your cookbooks.

### Configuring nodes with Chef Infra Client

Chef Infra is constructed so that most of the computational effort
occurs on the nodes rather than on the Chef Infra Server. A node
represents any system you manage and is typically a virtual machine,
container instance, or physical server. Basically, it's any compute
resource in your infrastructure that's managed by Chef Infra. All nodes
have Chef Infra Client installed on them, and Chef Infra Client is
available for multiple platforms including Linux, macOS, Windows, AIX,
and Solaris.

Periodically, Chef Infra Client contacts the Chef Infra Server to
retrieve the latest cookbooks. If (and only if) the current state of the
node doesn't conform to what the cookbook says it should be, Chef Infra
Client executes the cookbook instructions. This iterative process
ensures that the network as a whole converges to the state envisioned by
business policy.

## Chef Habitat

Chef Habitat offers a new approach to deploying applications called
application automation. Application automation means that the automation
is packaged with the application and travels with it, no matter where
that application is deployed. The unit of deployment becomes the
application and its associated automation. The runtime environment,
whether it is a container, bare metal, or PaaS does not in any way
define the application.

Chef Habitat is comprised of a packaging format and a supervisor. The
format defines Chef Habitat packages, which are isolated, immutable, and
auditable. The Chef Habitat supervisor knows how to take the packages
and run them. It's aware of the package's peer relationships, its
upgrade strategy and security policies. To learn everything about Chef
Habitat, go to the Chef Habitat web site at
[https://www.habitat.sh](https://www.habitat.sh/).

## Chef InSpec

Chef InSpec is an open-source testing framework with a human- and
machine-readable language for specifying compliance, security and policy
requirements. When compliance is expressed as code, you can integrate it
into your deployment pipeline and automatically test for adherence to
security policies.

Chef InSpec code can run in multiple platforms. You can execute the same
set of tests locally, with remote commands that use SSH or WinRM, or
with external mechanisms such as the Docker API.

With Chef InSpec, you can do more than ensure that your physical servers
are in compliance. You can, for example, assess data in a database or
inspect the configuration of virtual resources by using their API.

To get a sense of how the Chef InSpec language works, here are some
examples. This Chef InSpec rule ensures that insecure services and
protocols, such as telnet, are not used.

``` ruby
describe package('telnetd') do
 it { should_not be_installed }
end

describe inetd_conf do
 its("telnet") { should eq nil }
end
```

## Chef Automate

Chef Automate provides a full suite of enterprise capabilities for node
visibility and compliance. Chef Automate integrates with the open-source
products Chef Infra Client, Chef InSpec and Chef Habitat. Chef Automate
comes with comprehensive 24x7 support services for the entire platform,
including open source components.

Chef Automate gives you a full-stack continuous compliance and security,
as well as visibility into your applications and infrastructure.

### Nodes

Chef Automate gives you a data warehouse that accepts input from Chef
Server, Chef Habitat, and Chef Automate workflow and compliance. It
provides views into operational and workflow events. There is a query
language available through the UI and customizable dashboards.

Here is an example of the Chef Automate dashboard.

<img src="/images/visibility1.png" class="align-center" width="700" alt="image" />

### Compliance

Chef Automate creates customizable reports that identify compliance
issues, security risks, and outdated software. You can write your own
compliance rules in Chef InSpec, or you can get started quickly by using
built-in profiles, which are predefined rule sets for a variety of
security frameworks, such as Center for Internet Security (CIS)
benchmarks, included as part of Chef Automate.

For information on the integrated reporting capabilities in Chef
Automate, see [Compliance Overview](/automate/reports/).

### High availability

Chef Automate includes a high-availability Chef Infra Server with fault
tolerance, immediately consistent search results, and accurate real-time
data about your infrastructure. Chef Automate also provides a graphical
management console for the Chef Infra Server.

## More Resources

If you're interested in getting hands-on experience, go to
<https://learn.chef.io/> for tutorials, information about formal
training classes and community resources. The Chef Habitat web site at
<https://www.habitat.sh/> has Habitat tutorials, along with
documentation and other resources.
