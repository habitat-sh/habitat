The RubyGems package provider attempts to use the RubyGems API to
install gems without spawning a new process, whenever possible. A gems
command to install will be spawned under the following conditions:

-   When a `gem_binary` property is specified (as a hash, a string, or
    by a .gemrc file), Chef Infra Client will run that command to
    examine its environment settings and then again to install the gem.
-   When install options are specified as a string, Chef Infra Client
    will span a gems command with those options when installing the gem.
-   The Chef installer will search the `PATH` for a gem command rather
    than defaulting to the current gem environment. As part of
    `enforce_path_sanity`, the `bin` directories area added to the
    `PATH`, which means when there are no other proceeding RubyGems, the
    installation will still be operated against it.