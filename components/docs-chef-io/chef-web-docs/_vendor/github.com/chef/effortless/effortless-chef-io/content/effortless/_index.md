+++
title = "Effortless Overview"
draft = false

[menu]
  [menu.effortless]
    title = "Effortless Overview"
    identifier = "effortless/_index.md Overview"
    parent = "effortless"
    weight = 10
+++

# Welcome to the Chef Effortless Patterns

The Effortless Pattern is a way to better manage Chef Infra and Chef InSpec workloads using Chef Habitat, and to visualize your fleet using Chef Automate. We believe that each of these technologies working together can help you better manage your infrastructure.

## Who Should Use Effortless

If you use Chef Infra or Chef InSpec to manage your operating system configs, you should look at using Effortless. Effortless uses the power of Chef Habitat to help with you with Chef Infra and Chef InSpec dependency management. Chef Habitat pulls in the cookbooks and profiles that your cookbook and/or Chef InSpec profiles depend upon and packages them as a signed artifact. By packaging the latest and best practices for running Chef Infra and Chef InSpec on your systems, you do not have to worry about setting up the Chef Infra Client or the Chef InSpec client to run on your system.

## When Is Using Effortless Not The Best Approach

* If you use Chef Infra to deploy complex applications, you may not want to use the Effortless pattern.
  * Effortless does not support situations that require a Chef Infra Server. For example, if you use search in your cookbooks, or use `chef vault` for secrets management, then Effortless will not work for those cookbooks
  * If you have complex applications, you should deploy them with Chef Habitat as it has more features that better support complex applications

* If you have a bunch of nested cookbooks or Policyfiles in a complex [Chef Roles](https://docs.chef.io/roles/) and [Chef Environments](https://docs.chef.io/environments/) setup, you may not want to move to Effortless.
  * When you have a base cookbook and a bunch of Applications cookbooks dependent on that base cookbook, managing the build graph can become difficult because a change to the base cookbook will require a build to all the application cookbooks. Effortless in this situation can quickly become difficult to manage.

## Purpose

The purpose of the Effortless pattern is to reduce the amount of code and Chef knowledge a user needs to be successful with Chef products.

## Quick Links

* [Chef Infra](https://github.com/chef/chef) - Chef Infra automates infrastructure configuration and ensures every system configures correctly and consistently.
* [Chef InSpec](https://github.com/inspec/inspec) - Chef InSpec automates security tests, and ensures enforcement of consistent standards in every environment and at every stage of development.
* [Chef Habitat](https://github.com/habitat-sh/habitat) - Chef Habitat codifies how the application builds, how it runs, and its dependencies to free the application from underlying infrastructure and make updates easy.
* [Chef Automate](https://github.com/chef/automate) - Chef Automate provides an Enterprise dashboard and analytics tool to enable cross-team collaboration with actionable insights for configuration and compliance, and an auditable history of environment changes.
