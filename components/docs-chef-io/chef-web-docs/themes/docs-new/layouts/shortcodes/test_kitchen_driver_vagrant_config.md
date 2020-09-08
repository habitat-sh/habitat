The `kitchen-vagrant` driver can predict the box name for Vagrant and
the download URL that have been published by Chef. For example:

``` ruby
platforms:
- name: ubuntu-16.04
- name: ubuntu-18.04
- name: centos-7
- name: centos-8
- name: debian-10
```

which will generate a configuration file similar to:

``` ruby
platforms:
- name: ubuntu-16.04
  driver:
    box: bento/ubuntu-16.04
- name: ubuntu-18.04
  driver:
    box: bento/ubuntu-18.04
# ...
```