The Recipe DSL is a Ruby DSL that is primarily used to declare resources
from within a recipe. The Recipe DSL also helps ensure that recipes
interact with nodes (and node properties) in the desired manner. Most of
the methods in the Recipe DSL are used to find a specific parameter and
then tell Chef Infra Client what action(s) to take, based on whether
that parameter is present on a node.