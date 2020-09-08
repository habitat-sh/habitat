An attribute can be defined in a cookbook (or a recipe) and then used to
override the default settings on a node. When a cookbook is loaded
during a Chef Infra Client run, these attributes are compared to the
attributes that are already present on the node. Attributes that are
defined in attribute files are first loaded according to cookbook order.
For each cookbook, attributes in the `default.rb` file are loaded first,
and then additional attribute files (if present) are loaded in lexical
sort order. When the cookbook attributes take precedence over the
default attributes, Chef Infra Client applies those new settings and
values during a Chef Infra Client run on the node.