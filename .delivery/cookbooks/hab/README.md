# The Habitat Build Cookbook

This project is testable with [Test Kitchen](http://kitchen.ci) and assumes
you have a [ChefDK](https://downloads.chef.io/chef-dk/) installation.

There is a Test Kitchen Instance per phase which approximates setting up a new
Delivery builder node and running the appropriate phase:

```sh
> kitchen list
Instance                Driver   Provisioner  Verifier  Transport  Last Action
unit-ubuntu-1504        Vagrant  ChefSolo     Busser    Ssh        <Not Created>
lint-ubuntu-1504        Vagrant  ChefSolo     Busser    Ssh        <Not Created>
functional-ubuntu-1504  Vagrant  ChefSolo     Busser    Ssh        <Not Created>
```

To setup an Instance for development or investigation, run:
`kitchen converge <PHASE>`.

To login: `kitchen login <PHASE>`.

And finally, to destroy: `kitchen destroy <PHASE>`.
