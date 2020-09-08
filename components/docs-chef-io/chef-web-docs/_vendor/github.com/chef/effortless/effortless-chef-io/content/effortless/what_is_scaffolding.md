+++
title = "Effortless What is Scaffolding"
draft = false

[menu]
  [menu.effortless]
    title = "What is Scaffolding"
    identifier = "effortless/what_is_scaffolding.md Effortless What is Scaffolding"
    parent = "effortless"
    weight = 50
+++

# Chef Habitat Scaffolding

Chef Habitat scaffolding is a way to build a Chef Habitat plan that overrides some of the default functions that Chef Habitat uses during its build process. You can find out more about [scaffolding](https://www.habitat.sh/docs/glossary/#sts=Scaffolding) in the Chef Habitat documentation.

## Why The Effortless Pattern Uses Scaffolding

We use Chef Habitat scaffolding to provide the Effortless maintainers better management of the build and runtime steps needed to be successful with Chef Infra and Chef InSpec. This means a small plan file is the amount of code that you, the customer, need to maintain in order to build and run Chef Infra and Chef InSpec. Focus on writing Chef Infra cookbooks and Chef InSpec profiles, and not on how to build and run those things.

## How Scaffolding Works

By specifying the `pkg_scaffolding` variable in your plan, Chef Habitat will pull in the necessary package dependencies, run the build steps for Chef Infra and/or Chef InSpec, and provide you with a Chef Habitat artifact that contains your cookbooks or profiles and a way to run them on your systems.

Find the source code for these steps here for [Chef Infra](https://github.com/chef/effortless/tree/master/scaffolding-chef-infra/lib) and for [Chef InSpec](https://github.com/chef/effortless/tree/master/scaffolding-chef-inspec/lib).
