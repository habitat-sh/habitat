If the `config.json` file specifies:

``` javascript
"delivery-truck": {
  "lint": {
    "foodcritic": {
      "ignore_rules": ["FC009", "FC057", "FC058"],
      "excludes": ["DIRECTORY", "DIRECTORY", ...]
    }
  }
}
```

then all Foodcritic rules except `FC009`, `FC057`, and `FC058` rules are
run.