To search for cookbooks on a node, use the `recipes` attribute followed
by the `cookbook::recipe` pattern, escaping both of the `:` characters.
For example:

``` bash
knife search node 'recipes:cookbook_name\:\:recipe_name'
```