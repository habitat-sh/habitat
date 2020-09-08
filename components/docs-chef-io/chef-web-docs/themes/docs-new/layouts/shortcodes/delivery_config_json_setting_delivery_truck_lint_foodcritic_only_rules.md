If the `config.json` file specifies:

``` javascript
"delivery-truck": {
  "lint": {
    "foodcritic": {
      "only_rules": ["FC002"],
      "excludes": ["DIRECTORY", "DIRECTORY", ...]
    }
  }
}
```

then only the `FC002` Foodcritic rules is run.