For example, if a kitchen.yml file contains the following:

``` javascript
- name: centos-7
- name: centos-8
- name: fedora-latest
- name: ubuntu-1604
- name: ubuntu-1804
```

then a regular expression like `(04|7)` would run Test Kitchen against
`centos-7`, `ubuntu-1604`, and `ubuntu-1804`. A regular expression like
`(ubuntu)` would run Test Kitchen against `ubuntu-1604` and
`ubuntu-1804`. A regular expression like `(fedora)` would run Test
Kitchen against only `fedora-latest`. Default: `all`.