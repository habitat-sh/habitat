If the `config.json` file specifies:

``` javascript
"delivery-truck": {
  "lint": {
    "foodcritic": {
      "ignore_rules": ["RULE", "RULE", ...],
      "only_rules": ["RULE", "RULE", ...],
      "excludes": ["spec", "test"]
    }
  }
}
```

then Foodcritic rules are not run against tests that are located in the
specified directories, in this case the `/spec` and `/test` directories.