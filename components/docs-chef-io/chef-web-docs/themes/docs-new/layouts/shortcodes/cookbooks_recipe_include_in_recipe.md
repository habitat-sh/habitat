A recipe can include one (or more) recipes from cookbooks by using the
`include_recipe` method. When a recipe is included, the resources found
in that recipe will be inserted (in the same exact order) at the point
where the `include_recipe` keyword is located.

The syntax for including a recipe is like this:

``` ruby
include_recipe 'recipe'
```

For example:

``` ruby
include_recipe 'apache2::mod_ssl'
```

Multiple recipes can be included within a recipe. For example:

``` ruby
include_recipe 'cookbook::setup'
include_recipe 'cookbook::install'
include_recipe 'cookbook::configure'
```

If a specific recipe is included more than once with the
`include_recipe` method or elsewhere in the run_list directly, only the
first instance is processed and subsequent inclusions are ignored.