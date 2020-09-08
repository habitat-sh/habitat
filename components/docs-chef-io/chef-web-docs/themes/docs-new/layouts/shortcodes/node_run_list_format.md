A run-list must be in one of the following formats: fully qualified,
cookbook, or default. Both roles and recipes must be in quotes, for
example:

```javascript
"role[NAME]"
```

or

```javascript
"recipe[COOKBOOK::RECIPE]"
```

Use a comma to separate roles and recipes when adding more than one item
the run-list:

``` javascript
"recipe[COOKBOOK::RECIPE],COOKBOOK::RECIPE,role[NAME]"
```