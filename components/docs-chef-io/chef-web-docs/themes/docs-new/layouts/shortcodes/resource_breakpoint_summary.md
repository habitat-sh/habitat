Use the **breakpoint** resource to add breakpoints to recipes. Run the
chef-shell in Chef Infra Client mode, and then use those breakpoints to
debug recipes. Breakpoints are ignored by Chef Infra Client during an
actual Chef Infra Client run. That said, breakpoints are typically used
to debug recipes only when running them in a non-production environment,
after which they are removed from those recipes before the parent
cookbook is uploaded to the Chef Infra Server.