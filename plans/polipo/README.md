This plan builds and configures the [polipo](https://www.irif.univ-paris-diderot.fr/~jch/software/polipo/) caching web proxy. After you deploy a dockerized polipo, you can use it as a caching proxy for Habitat by exporting it in your shell.

    http_proxy=http://192.168.142.180:8123; export http_proxy

It appears that not every Habitat command utilizes the `http_proxy` environment variable yet, but those that do produce decidedly faster builds.
