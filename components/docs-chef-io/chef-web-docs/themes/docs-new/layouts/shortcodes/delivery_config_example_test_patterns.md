The following example shows how to configure Chef Automate to ignore
and/or run certain Foodcritic rules, and to exclude running tests that
are located in the specified cookbook directories:

``` javascript
{
  "version": "2",
  "build_cookbook": {
    "name": "delivery-truck",
    "git": "https://github.com/chef-cookbooks/delivery-truck.git"
  },
  "delivery-truck": {
    "lint": {
      "foodcritic": {
        "ignore_rules": ["FC009", "FC057", "FC058"],
        "only_rules": ["FC002"],
        "excludes": ["spec", "test"],
        "fail_tags": ["any"]
      }
    }
  }
}
```

where:

-   `ignore_rules` is set to ignore Foodcritic rules `FC009`, `FC057`,
    `FC058`
-   `only_rules` is set to run only Foodcritic rule `FC002`; omit this
    setting to specify all rules not specified by `ignore_rules`
-   `excludes` prevents Foodcritic rules from running if they are
    present in a cookbook's `/spec` and/or `/test` directories
-   `fail_tags` states which rules should cause the run to fail; omit
    this setting to specify `correctness`