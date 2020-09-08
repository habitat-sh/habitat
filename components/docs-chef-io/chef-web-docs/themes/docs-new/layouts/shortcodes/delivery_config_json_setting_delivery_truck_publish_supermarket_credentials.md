Publish cookbooks to Chef Supermarket, but with custom credentials:

``` javascript
"delivery-truck":{
  "publish": {
    "supermarket": "https://supermarket.chef.io",
    "supermarket-custom-credentials": "true"
  }
}
```

This `publish` option requires the `supermarket_user` and
`supermarket_key` credentials to be available from the
`delivery-secrets` data bag on the Chef Infra Server that is part of
this Chef Automate configuration. For more information on the
`delivery-secrets` data bag, see [Handling
Secrets](https://github.com/chef-cookbooks/delivery-sugar#handling-secrets-alpha)
in the `delivery-sugar` cookbook README file.