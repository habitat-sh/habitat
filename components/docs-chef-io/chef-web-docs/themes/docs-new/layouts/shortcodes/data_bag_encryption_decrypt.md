An encrypted data bag item is decrypted with a knife command similar to:

``` bash
knife data bag show --secret-file /tmp/my_data_bag_key passwords mysql
```

that will return JSON output similar to:

``` javascript
{
  "id": "mysql",
  "pass": "thesecret123",
  "user": "fred"
}
```