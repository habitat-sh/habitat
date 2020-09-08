A data bag item is encrypted using a knife command similar to:

``` bash
knife data bag create passwords mysql --secret-file /tmp/my_data_bag_key
```

where "passwords" is the name of the data bag, "mysql" is the name of
the data bag item, and "/tmp/my_data_bag_key" is the path to the
location in which the file that contains the secret-key is located.
knife will ask for user credentials before the encrypted data bag item
is saved.