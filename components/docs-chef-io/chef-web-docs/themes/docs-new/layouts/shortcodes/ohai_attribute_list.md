Ohai collects a list of automatic attributes at the start of each Chef
Infra Client run. This list will vary from organization to organization,
by server type, and by the platform that runs those servers. All the
attributes collected by Ohai are unmodifiable by Chef Infra Client. Run
the `ohai` command on a system to see which automatic attributes Ohai
has collected for a particular node.