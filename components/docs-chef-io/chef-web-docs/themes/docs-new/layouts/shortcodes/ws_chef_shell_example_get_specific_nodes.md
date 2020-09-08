To get a list of nodes using a recipe named `postfix` use
`search(:node,"recipe:postfix")`. To get a list of nodes using a
sub-recipe named `delivery`, use chef-shell. For example:

``` ruby
search(:node, 'recipes:postfix\:\:delivery')
```

<div class="admonition-note">

<p class="admonition-note-title">Note</p>

<div class="admonition-note-text">

Single (' ') vs. double (" ") is important. This is because a backslash
() needs to be included in the string, instead of having Ruby interpret
it as an escape.



</div>

</div>