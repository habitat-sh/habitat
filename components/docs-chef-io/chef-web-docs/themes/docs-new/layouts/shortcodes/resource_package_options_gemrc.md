Options can be specified in a .gemrc file. By default the `gem_package`
resource will use the Ruby interface to install gems which will ignore
the .gemrc file. The `gem_package` resource can be forced to use the
gems command instead (and to read the .gemrc file) by adding the
`gem_binary` attribute to a code block.