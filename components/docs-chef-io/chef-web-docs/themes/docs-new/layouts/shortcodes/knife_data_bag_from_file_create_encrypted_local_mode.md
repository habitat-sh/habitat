To generate an encrypted data bag item in a JSON file for use when Chef
Infra Client is run in local mode (via the `--local-mode` option),
enter:

``` bash
knife data bag from file my_data_bag /path/to/data_bag_item.json -z --secret-file /path/to/encrypted_data_bag_secret
```

this will create an encrypted JSON file in:

    data_bags/my_data_bag/data_bag_item.json