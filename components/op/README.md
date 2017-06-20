# op

This is designed to be used by developers working on Habitat. This crate builds a command line tool called `op` and it provides two commands:

`op hash --file /path/to/file`

This will return the BLAKE2b hash checksum for said file, which is very useful if you need to upload files manually via curl, e.g. because the file is huge and `hab pkg upload` is timing out.

`op shard --origin myoriginname`

This will return the shard number for the origin provided, allowing you to know what to set the Postgres search path to in order to find what you need.
