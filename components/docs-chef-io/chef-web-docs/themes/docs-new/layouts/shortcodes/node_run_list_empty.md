Use an empty run-list to determine if a failed Chef Infra Client run has
anything to do with the recipes that are defined within that run-list.
This is a quick way to discover if the underlying cause of a Chef Infra
Client run failure is a configuration issue. If a failure persists even
if the run-list is empty, check the following:

-   Configuration settings in the config.rb file
-   Permissions for the user to both the Chef Infra Server and to the
    node on which a Chef Infra Client run is to take place