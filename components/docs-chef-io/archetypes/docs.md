+++
title = "{{ .Name | humanize | title }}"

date = {{ .Date }}
draft = false

[menu]
  [menu.automate]
    title = "{{ .Name | humanize | title }}"
    identifier = "automate/{{ .Name }}.md {{ .Name | humanize | title }}"
    parent = "automate"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/{{ .Name }}.md)
