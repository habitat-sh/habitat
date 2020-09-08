If the `config.json` file specifies the following cookbooks are
published to a git repository located on an open source git server:

``` javascript
"delivery-truck":{
  "publish": {
    "git": "ssh://git@stash:2222/<project-name>/<repo-name>"
  }
}
```

This publishing option requires the `git` deploy key for that repository
to be available from a data bag on the Chef Infra Server that is part of
this Chef Automate configuration.