chef-shell can be used to debug existing recipes. The recipe first needs
to be added to a run-list for the node, so that it is cached when
starting chef-shell and then used for debugging. chef-shell will report
which recipes are being cached when it is started:

``` bash
loading configuration: none (standalone session)
Session type: standalone
Loading.............done.

Welcome to the chef-shell 15.8.23
For usage see https://docs.chef.io/chef_shell.html

run `help' for help, `exit' or ^D to quit.

chef (15.8.23)>
```

To just load one recipe from the run-list, go into the recipe and use
the `include_recipe` command. For example:

``` bash
chef > recipe_mode
  chef:recipe > include_recipe "getting-started"
    => [#< Chef::Recipe:0x10256f9e8 @cookbook_name="getting-started",
  ... output truncated ...
```

To load all of the recipes from a run-list, use code similar to the
following:

``` ruby
node.run_list.expand(node.chef_environment).recipes.each do |r|
  include_recipe r
end
```

After the recipes that are to be debugged have been loaded, use the
`run_chef` command to run them.