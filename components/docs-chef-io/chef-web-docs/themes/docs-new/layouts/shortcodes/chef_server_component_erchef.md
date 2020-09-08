Erchef is a complete rewrite of the core API for the Chef Infra Server,
which allows it to be faster and more scalable than previous versions.
The API itself is still compatible with the original Ruby-based Chef
Infra Server, which means that cookbooks and recipes that were authored
for the Ruby-based Chef Infra Server will continue to work on the
Erlang-based Chef Infra Server. Chef Infra Client is still written in
Ruby.

<div class="admonition-note">

<p class="admonition-note-title">Note</p>

<div class="admonition-note-text">

Even though the Chef Infra Server is authored in Erlang, writing code in
Erlang is NOT a requirement for using Chef.



</div>

</div>