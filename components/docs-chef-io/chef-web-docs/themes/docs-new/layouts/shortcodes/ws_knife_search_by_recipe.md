To search for recipes that are used by a node, use the `recipes`
attribute to search for the recipe names, enter something like:

``` bash
knife search node 'recipes:recipe_name'
```

or:

``` bash
knife search node '*:*' -a recipes | grep 'recipe_name'
```