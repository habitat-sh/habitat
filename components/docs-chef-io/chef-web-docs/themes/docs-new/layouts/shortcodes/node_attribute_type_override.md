An `override` attribute is automatically reset at the start of every
Chef Infra Client run and has a higher attribute precedence than
`default`, `force_default`, and `normal` attributes. An `override`
attribute is most often specified in a recipe, but can be specified in
an attribute file, for a role, and/or for an environment. A cookbook
should be authored so that it uses `override` attributes only when
required.