+++
title = "{{ .Name | humanize | title }}"
date = {{ .Date }}
draft = false
gh_repo = "habitat"

[menu]
  [menu.habitat]
    title = "{{ .Name | humanize | title }}"
    identifier = "habitat/{{ .Name }}.md {{ .Name | humanize | title }}"
    parent = "habitat"
    weight = 10
+++
