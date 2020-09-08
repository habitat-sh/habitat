If an explicit `gem_binary` parameter is not being used with the
`gem_package` resource, it is preferable to provide the install options
as a hash. This approach allows the provider to install the gem without
needing to spawn an external gem process.

The following RubyGems options are available for inclusion within a hash
and are passed to the RubyGems DependencyInstaller:

-   `:env_shebang`
-   `:force`
-   `:format_executable`
-   `:ignore_dependencies`
-   `:prerelease`
-   `:security_policy`
-   `:wrappers`

For more information about these options, see the RubyGems
documentation:
<http://rubygems.rubyforge.org/rubygems-update/Gem/DependencyInstaller.html>.