---
title: Director
---

# Director

The Habitat director is a supervisor for a group of supervisor (`hab-sup`) processes running on one machine. The director automatically restarts child processes upon failure detection and each child service runs in it's own supervisor process.

Services are listed a config.toml file. This file defines the start order, service group, organization, and any CLI arguments for each service.

  > Note: The start order does not guarantee processes are fully initialized and started before the next one in the list is started.

The director can be run inside of a supervisor as well. As with any other service, this allows the director to be updated with new configuration changes at runtime, which enable it to dynamically deploy different child service configurations and topologies.

When running inside of a supervisor, the director will use the standard gossip and HTTP API ports of 9638 and 9631, respectively. However, child supervisor processes will use the same IP address of the director. The default for every service is to use ports 9638 and 9631, so to stop port collision from happening when the child `hab-sup` processes start up, the director defines ring and HTTP API port numbers for all children in sequential order starting with port 9000 for gossip connections and port 8000 for HTTP API connections. It's advisable to keep the director's port numbers at their default values to avoid port collisions.

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/concepts-plans">Plans</a></li>
</ul>
