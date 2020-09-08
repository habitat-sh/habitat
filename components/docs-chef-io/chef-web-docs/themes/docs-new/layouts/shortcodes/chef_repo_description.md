The chef-repo is a directory on your workstation that stores everything
you need to define your infrastructure with Chef Infra:

-   Cookbooks (including recipes, attributes, custom resources,
    libraries, and templates)
-   Data bags
-   Policyfiles

The chef-repo directory should be synchronized with a version control
system, such as git. All of the data in the chef-repo should be treated
like source code.

You'll use the `chef` and `knife` commands to upload data to the Chef
Infra Server from the chef-repo directory. Once uploaded, Chef Infra
Client uses that data to manage the nodes registered with the Chef Infra
Server and to ensure that it applies the right cookbooks, policyfiles,
and settings to the right nodes in the right order.