Cookbook and custom resource names should contain only alphanumeric
characters. A hyphen (`-`) is a valid character and may be used in
cookbook and custom resource names, but it is discouraged. Chef Infra
Client will return an error if a hyphen is not converted to an
underscore (`_`) when referencing from a recipe the name of a custom
resource in which a hyphen is located.