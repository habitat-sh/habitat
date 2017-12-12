# builder-sessionsrv

Generates, validates, and invalidates authenticated sessions for builder clients

# Adding new migrations

```
cargo install diesel-cli
diesel migration generate --migration-dir ./src/migrations <migration_name>
```
