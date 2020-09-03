# Managing migrations for Builder services

All builder migrations are run with [Diesel](http://diesel.rs). This document describes how to create and manage those migrations.

## Generating new migrations

Every time you need to make a change to the Builder schema you will be required to generate a new migration

For the service `builder-SERVICE` you will need to run:

* `cd components/builder-SERVICE/src`
* `diesel migration generate <your migration name>`

The migration name should describe what you are doing. Ex:

* create-posts
* add-user-select-v4
* remove-user-select-43

This will generate something like

```
Creating migrations/20160815133237_create_posts/up.sql
Creating migrations/20160815133237_create_posts/down.sql
```

You can then edit `up.sql` to create your migration steps.
You should ignore, but not delete, `down.sql` as we don't use it since we rely on transactions for our rollback logic.

## Testing your changes

You will need to compile your service and restart it to test your changes. You should see:

`Running Migration <your-migration-name>`