A run-list defines all of the information necessary for Chef to
configure a node into the desired state. A run-list is:

-   An ordered list of roles and/or recipes that are run in the exact
    order defined in the run-list; if a recipe appears more than once in
    the run-list, Chef Infra Client will not run it twice
-   Always specific to the node on which it runs; nodes may have a
    run-list that is identical to the run-list used by other nodes
-   Stored as part of the node object on the Chef server
-   Maintained using knife and then uploaded from the workstation to the
    Chef Infra Server, or maintained using Chef Automate