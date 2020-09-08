Superuser accounts may not be managed by users who belong to the
`server-admins` group. For example, Alice attempts to delete the
`pivotal` superuser account:

``` bash
knife user delete pivotal -c ~/.chef/alice.rb
```

and the following error is returned:

``` bash
ERROR: You authenticated successfully to <chef_server_url> as user1
       but you are not authorized for this action
Response: Missing read permission
```

Alice's action is unauthorized even with membership in the
`server-admins` group.