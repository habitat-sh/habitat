The recommended order in which registry key-specific methods should be
used within a recipe is: `key_exists?`, `value_exists?`, `data_exists?`,
`get_values`, `has_subkeys?`, and then `get_subkeys`.