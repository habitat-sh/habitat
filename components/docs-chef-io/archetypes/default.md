+++
title = "{{ .Name | humanize | title }}"

date = {{ .Date }}
draft = false

[menu]
  [menu.habitat]
    title = "{{ .Name | humanize | title }}"
    identifier = "habitat/{{ .Name }}.md {{ .Name | humanize | title }}"
    parent = "habitat"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/habitat-sh/habitat/blob/master/components/docs-chef-io/content/habitat/{{ .Name }}.md)
