Attributes are always applied by Chef Infra Client in the following
order:

1.  A `default` attribute located in a cookbook attribute file
2.  A `default` attribute located in a recipe
3.  A `default` attribute located in an environment
4.  A `default` attribute located in a role
5.  A `force_default` attribute located in a cookbook attribute file
6.  A `force_default` attribute located in a recipe
7.  A `normal` attribute located in a JSON file passed via
    `chef-client -j`
8.  A `normal` attribute located in a cookbook attribute file
9.  A `normal` attribute located in a recipe
10. An `override` attribute located in a cookbook attribute file
11. An `override` attribute located in a recipe
12. An `override` attribute located in a role
13. An `override` attribute located in an environment
14. A `force_override` attribute located in a cookbook attribute file
15. A `force_override` attribute located in a recipe
16. An `automatic` attribute identified by Ohai at the start of a Chef
    Infra Client run

where the last attribute in the list is the one that is applied to the
node.

<div class="admonition-note">

<p class="admonition-note-title">Note</p>

<div class="admonition-note-text">

The attribute precedence order for roles and environments is reversed
for `default` and `override` attributes. The precedence order for
`default` attributes is environment, then role. The precedence order for
`override` attributes is role, then environment. Applying environment
`override` attributes after role `override` attributes allows the same
role to be used across multiple environments, yet ensuring that values
can be set that are specific to each environment (when required). For
example, the role for an application server may exist in all
environments, yet one environment may use a database server that is
different from other environments.



</div>

</div>

Attribute precedence, viewed from the same perspective as the overview
diagram, where the numbers in the diagram match the order of attribute
precedence:

![image](/images/overview_chef_attributes_precedence.png)

Attribute precedence, when viewed as a table:

![image](/images/overview_chef_attributes_table.png)