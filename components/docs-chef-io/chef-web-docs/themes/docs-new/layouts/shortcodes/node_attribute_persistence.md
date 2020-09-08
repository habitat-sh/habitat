All attributes except for normal attributes are reset at the beginning
of a Chef Infra Client run. Attributes set via `chef-client -j` with a
JSON file have normal precedence and are persisted between Chef Infra
Client runs. Chef Infra Client rebuilds these attributes using automatic
attributes collected by Ohai at the beginning of each Chef Infra Client
run, and then uses default and override attributes that are specified in
cookbooks, roles, environments, and Policyfiles. All attributes are then
merged and applied to the node according to attribute precedence. The
attributes that were applied to the node are saved to the Chef Infra
Server as part of the node object at the conclusion of each Chef Infra
Client run.