+++
title = "Quick Start"
draft = false

aliases = ["/quick_start.html"]

[menu]
  [menu.infra]
    title = "Quick Start"
    identifier = "chef_infra/getting_started/quick_start.md Quick Start"
    parent = "chef_infra/getting_started"
    weight = 20
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/quick_start.md)

For the quickest way to get started using Chef Infra:

1.  Install Chef Workstation:
    <https://downloads.chef.io/chef-workstation/>.

2.  Generate a cookbook:

    ``` bash
    chef generate cookbook first_cookbook
    ```

    where `first_cookbook` is an arbitrary cookbook name.

3.  Navigate to the `first_cookbook` directory.

4.  Update the `cookbooks/first_cookbook/recipes/default.rb` recipe in
    the generated cookbook to contain:

    ``` ruby
    file "#{ENV['HOME']}/test.txt" do
      content 'This file was created by Chef Infra!'
    end
    ```

5.  Run Chef Infra Client using the `default.rb` recipe:

    ``` bash
    chef-client --local-mode --override-runlist first_cookbook
    ```

This will create a file named `test.txt` at the home path on your
machine. Open that file and it will say
`This file was created by Chef Infra!`.

-   Delete the file, run Chef Infra Client again, and Chef Infra will
    put the file back.
-   Change the string in the file, run Chef Infra Client again, and Chef
    Infra will make the string in the file the same as the string in the
    recipe.
-   Change the string in the recipe, run Chef Infra Client again, and
    Chef Infra will update that string to be the same as the one in the
    recipe.

There's a lot more that Chef Infra can do, obviously, but that was super
easy!

-   See <https://learn.chef.io/> for more detailed setup scenarios.
-   Keep reading for more information about setting up a workstation,
    configuring Test Kitchen to run virtual environments, setting up a
    more detailed cookbook, resources, and more.
