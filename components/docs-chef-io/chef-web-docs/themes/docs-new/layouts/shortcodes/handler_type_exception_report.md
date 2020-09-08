Exception and report handlers are used to trigger certain behaviors in
response to specific situations, typically identified during a Chef
Infra Client run.

-   An exception handler is used to trigger behaviors when a defined
    aspect of a Chef Infra Client run fails.
-   A report handler is used to trigger behaviors when a defined aspect
    of a Chef Infra Client run is successful.

Both types of handlers can be used to gather data about a Chef Infra
Client run and can provide rich levels of data about all types of usage,
which can be used later for trending and analysis across the entire
organization.

Exception and report handlers are made available to a Chef Infra Client
run in one of the following ways:

-   By adding the **chef_handler** resource to a recipe, and then
    adding that recipe to the run-list for a node. (The
    **chef_handler** resource is available from the **chef_handler**
    cookbook.)
-   By adding the handler to one of the following settings in the node's
    client.rb file: `exception_handlers` and/or `report_handlers`