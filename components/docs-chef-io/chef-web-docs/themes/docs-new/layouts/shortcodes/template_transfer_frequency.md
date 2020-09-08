The Chef Infra Client caches a template when it is first requested. On
each subsequent request for that template, the Chef Infra Client
compares that request to the template located on the Chef Infra Server.
If the templates are the same, no transfer occurs.