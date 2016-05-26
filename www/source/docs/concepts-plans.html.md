# Plans

A plan is a set of files that describe how to build a binary Habitat package. At the heart of the plan is a configurable bash script named `plan.sh`, containing instructions on how to download, compile, and install software.
Optionally included are a set of TOML variables and their defaults that can be used to generate configuration files via Handlebar.js templates. Lifecycle hooks for services can also be specified in the form of bash scripts.

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/concepts-packages">Packages and artifacts</a></li>
</ul>
