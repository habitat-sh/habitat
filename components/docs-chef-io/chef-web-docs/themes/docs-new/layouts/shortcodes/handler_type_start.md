A start handler is not loaded into a Chef Infra Client run from a
recipe, but is instead listed in the client.rb file using the
`start_handlers` attribute. The start handler must be installed on the
node and be available to Chef Infra Client prior to the start of a Chef
Infra Client run. Use the **chef-client** cookbook to install the start
handler.

Start handlers are made available to a Chef Infra Client run in one of
the following ways:

-   By adding a start handler to the **chef-client** cookbook, which
    installs the handler on the node so that it is available to Chef
    Infra Client at the start of a Chef Infra Client run
-   By adding the handler to one of the following settings in the node's
    client.rb file: `start_handlers`