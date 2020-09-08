The supermarket.rb file contains all of the non-default configuration
settings used by the Chef Supermarket. The default settings are built-in
to the Chef Supermarket configuration, and should only be added to the
supermarket.rb file to apply non-default values. These configuration
settings are processed when the `supermarket-ctl reconfigure` command is
run. The supermarket.rb file is a Ruby file, which means that
conditional statements can be used in the configuration file.