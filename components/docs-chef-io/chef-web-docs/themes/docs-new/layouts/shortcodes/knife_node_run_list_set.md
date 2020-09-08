Use the `run_list set` argument to set the run-list for a node. A recipe
must be in one of the following formats: fully qualified, cookbook, or
default. Both roles and recipes must be in quotes, for example:
`"role[ROLE_NAME]"` or `"recipe[COOKBOOK::RECIPE_NAME]"`. Use a comma to
separate roles and recipes when setting more than one, like this:
`"recipe[COOKBOOK::RECIPE_NAME],COOKBOOK::RECIPE_NAME,role[ROLE_NAME]"`.