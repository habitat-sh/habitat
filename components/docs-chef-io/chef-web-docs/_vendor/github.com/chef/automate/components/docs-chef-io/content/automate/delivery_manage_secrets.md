+++
title = "Secrets"

draft = false
[menu]
  [menu.automate]
    title = "Secrets"
    parent = "automate/workflow"
    identifier = "automate/workflow/delivery_manage_secrets.md Secrets"
    weight = 30
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/delivery_manage_secrets.md)

Workflow is a legacy feature for Chef Automate, which was designed for managing changes to both infrastructure and application code, giving your operations and development teams a common platform for developing, testing, and deploying cookbooks, applications, and more.

{{< warning >}}
Workflow is available in Chef Automate for existing users. If you are not already using Workflow, but are interested in the solution it offers, please contact your sales or success representative for support with continuous integration pipelines.
{{< /warning >}}

This topic describes how and why to use secrets in a **build-cookbook**:

* This code is used in the **build-cookbook**
* Secrets are managed at the project, organization and/or enterprise level
* There are two mechanisms to manage secrets in Chef Automate:
    * Using an encrypted data bag
    * Using chef vaults

For more information on security, see [How to be a Secure Chef](https://learn.chef.io/tracks/administering-chef-installation/).

## Using Encrypted Data Bags

This section describes how to use encrypted data bag items in Chef Automate.

### Create an Encrypted Data Bag

Create an encrypted data bag item that is nested inside the `delivery-secrets` data bag. Give the encrypted data bag item a name following the pattern:

```ruby
<ENT>-<ORG>-<PROJECT>
```

For example, if the project is in the `chef` enterprise and is in the `ORG` organization with a name of `chef-web-www`, the encrypted data bag item would have the following name:

```ruby
chef-ORG-chef-web-www
```

The encrypted data bag item should use the same encrypted data bag private key that is distributed with the build nodes.

If the project item does not exist, **delivery-sugar** will try to load the secrets from the project's parent organization. It will look for an item called:

```ruby
chef-ORG
```

This is useful if you would like to share secrets across projects within the same organization.

### Use an Encrypted Data Bag

To use an encrypted data bag item, do the following:

1. Ensure that **metadata.rb** for the **build-cookbook** shows that it depends on the [delivery-sugar cookbook](https://github.com/chef-cookbooks/delivery-sugar)
2. Update the Berksfile to point to GitHub for the cookbook. The line in your Berksfile should be similar to:

    ```ruby
    cookbook 'delivery-sugar', github: 'chef-cookbooks/delivery-sugar'
    ```

From there, begin using the secrets by calling the `get_project_secrets` method. For example:

```ruby
if push_repo_to_github?
  secrets = get_project_secrets
  github_repo = node['delivery']['config']['delivery-truck']['publish']['github']

  delivery_github github_repo do
    deploy_key secrets['github']
    branch node['delivery']['change']['pipeline']
    remote_url "git@github.com:#{github_repo}.git"
    repo_path node['delivery']['workspace']['repo']
    cache_path node['delivery']['workspace']['cache']
    action :push
  end
```

This example is part of the **publish.rb** recipe in the [delivery-truck
cookbook](https://github.com/chef-cookbooks/delivery-truck/blob/master/recipes/publish.rb#L91-L103).

## Using a Chef Vault

This section describes how to use Chef vault in Chef Automate.

### Create a Chef Vault

In order to use Chef vaults you must follow hierarchical naming standard for your Chef vaults under the workflow-vaults data bag:

```ruby
<ENT>
<ENT>-<ORG>
<ENT>-<ORG>-<PROJECT>
```

For example, if the `chef` enterprise has a `cookbooks` organization with a `mysql` project, it would have the naming schema:

```ruby
chef
chef-cookbook
chef-cookbook-mysql
```

During the creation of a Chef vault, the data in these vaults are merged into a single Ruby hash. Any duplicate key names will be merged as follows:

* `<ENT>-<ORG>-<PROJECT>` will overwrite `<ENT>-<ORG>` and `<ENT>`.
* `<ENT>-<ORG>` will overwrite `<ENT>`.

## Using Secrets in a Chef Vault

To access your secret data from the vault items, ensure that **metadata.rb** for the **build-cookbook** shows that it depends on the [delivery-sugar cookbook](https://github.com/chef-cookbooks/delivery-sugar). From there, begin using the secrets by calling the `get_chef_vault_data` method.

For example:

```ruby
vault = get_workflow_vault_data
puts vault['my_key']
```
