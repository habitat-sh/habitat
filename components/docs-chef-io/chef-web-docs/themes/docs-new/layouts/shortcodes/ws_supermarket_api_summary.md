The Supermarket API is used to provide access to cookbooks, tools, and
users on the [Chef Supermarket](https://supermarket.chef.io). All of the
cookbooks, tools, and users on the Supermarket are accessible through a
RESTful API by accessing `supermarket.chef.io/api/v1/` via the supported
endpoints. In most cases, knife is the best way to interact with the
Supermarket; however in some cases, direct use of the Supermarket API is
necessary.