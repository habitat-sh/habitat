To edit an item named "charlie" that is contained in a data bag named
"admins", enter:

``` bash
knife data bag edit admins charlie
```

to open the \$EDITOR. Once opened, you can update the data before saving
it to the Chef Infra Server. For example, by changing:

``` javascript
{
   "id": "charlie"
}
```

to:

``` javascript
{
   "id": "charlie",
   "uid": 1005,
   "gid": "ops",
   "shell": "/bin/zsh",
   "comment": "Crazy Charlie"
}
```