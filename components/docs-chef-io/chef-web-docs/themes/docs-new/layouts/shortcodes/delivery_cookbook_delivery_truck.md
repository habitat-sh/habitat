`delivery-truck` is a cookbook for Chef Automate that should be a
dependency of every recipe in a `build-cookbook`, which is effectively a
project-specific wrapper cookbook for the `delivery-truck` cookbook. The
`delivery-truck` cookbook defines a set of recipes that correspond to
the phases and stages in the Chef Automate pipeline and help ensure good
default `build-cookbook` behavior. Chef recommends including the
`delivery-truck` cookbook in all recipes in a `build-cookbook`.