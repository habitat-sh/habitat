``` bash
knife bootstrap 192.0.2.0 -P vanilla -x root -r 'recipe[apt],recipe[xfs],recipe[vim]' --fips
```

which shows something similar to:

``` none
OpenSSL FIPS 140 mode enabled
...
192.0.2.0 Chef Infra Client finished, 12/12 resources updated in 78.942455583 seconds
```