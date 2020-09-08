A user who belongs to the `admins` group must be removed from the group
before they may be removed from an organization. To remove a user from
the `admins` group, run the following:

``` bash
EDITOR=vi knife edit /groups/admins.json
```

make the required changes, and then save the file.