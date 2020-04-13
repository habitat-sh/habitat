+++
title = "{{ replace .Name "-" " " | title }}"
draft = true

[menu]
  [menu.habitat]
    title = "{{ replace .Name "-" " " | title }}"
    identifier = "habitat/{{ .Name }}.md {{ replace .Name "-" " " | title }}"
    parent = "habitat/overview"
    weight = 10

+++

