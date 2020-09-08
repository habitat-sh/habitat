Use the `knife exec` subcommand to execute Ruby scripts in the context
of a fully configured Chef Infra Client. Use this subcommand to run
scripts that will only access Chef Infra Server one time (or otherwise
very infrequently) or any time that an operation does not warrant full
usage of the knife subcommand library.