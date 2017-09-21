---
title: Plans
---

# Plans

A plan is a set of files that describe how to build a binary Habitat package. At the heart of the plan is a configurable bash script named `plan.sh`, containing instructions on how to download, compile, and install software. Default build phases can be overridden using callbacks. 
Optionally included are a set of TOML variables and their defaults that can be used to generate configuration files via Handlebar.js templates. Application lifecycle hooks that will be called by the Supervisor to take certain actions when an event occurs for services in the runtime can also be specified here.

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/concepts-packages">Packages</a></li>
</ul>
