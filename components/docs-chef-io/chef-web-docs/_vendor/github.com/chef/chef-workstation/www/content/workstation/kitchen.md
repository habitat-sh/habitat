+++
title = "Test Kitchen"
draft = false

aliases = ["/kitchen.html", "/kitchen/"]

[menu]
  [menu.workstation]
    title = "About Test Kitchen"
    identifier = "chef_workstation/chef_workstation_tools/test_kitchen/kitchen.md About Test Kitchen"
    parent = "chef_workstation/chef_workstation_tools/test_kitchen"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/kitchen.md)

{{% test_kitchen %}}

The key concepts in Test Kitchen are:

-   A platform is the operating system or target environment on which a
    cookbook is to be tested
-   A suite is the Chef Infra Client configuration, a Policyfile or
    run-list, and (optionally) node attributes
-   An instance is the combination of a specific platform and a specific
    suite, with each instance being assigned an auto-generated name
-   A driver is the lifecycle that implements the actions associated
    with a specific instance---create the instance, do what is needed to
    converge on that instance (such as installing Chef Infra Client,
    uploading cookbooks, starting a Chef Infra Client run, and so on),
    setup anything else needed for testing, verify one (or more) suites
    post-converge, and then destroy that instance
-   A provisioner is the component on which the Chef Infra Client code
    will be run, either using chef-zero or chef-solo via the `chef_zero`
    and `chef_solo` provisioners, respectively

## Bento

{{% bento %}}

## Drivers

{{% test_kitchen_drivers %}}

## Validation with InSpec

Test Kitchen will create a VM or cloud instance, install Chef Infra
Client to that system, and converge Chef Infra Client with your local
cookbook. Once this is complete, you will want to perform automated
validation against the infrastructure you have built to validate its
configuration. Test Kitchen allows you to run InSpec tests against your
converged cookbook for easy local validation of your infrastructure.

## kitchen (executable)

{{% ctl_kitchen_summary %}}

{{< note >}}

For more information about the `kitchen` command line tool, see
[kitchen](/workstation/ctl_kitchen/).

{{< /note >}}

## kitchen.yml

{{% test_kitchen_yml %}}

{{< note >}}

For more information about the kitchen.yml file, see
[kitchen.yml](/workstation/config_yml_kitchen/).

{{< /note >}}

### Syntax

{{% test_kitchen_yml_syntax %}}

### Work with Proxies

{{< readFile_shortcode file="test_kitchen_yml_syntax_proxy.md" >}}

## For more information ...

For more information about test-driven development and Test Kitchen:

-   [kitchen.ci](https://kitchen.ci/)
-   [Getting Started with Test
    Kitchen](https://kitchen.ci/docs/getting-started/introduction/)
