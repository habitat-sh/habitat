Community member `juliandunn` created a custom [report handler that logs
all of the cookbooks and cookbook
versions](https://github.com/juliandunn/cookbook_versions_handler) that
were used during a Chef Infra Client run, and then reports after the run
is complete. This handler requires the **chef_handler** resource (which
is available from the **chef_handler** cookbook).