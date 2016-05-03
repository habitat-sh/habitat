---
title: Habitat Rust modules
---

# Habitat modules
To create the API documentation for the Habitat Rust modules, perform the following steps:

1. Open a terminal window and connect your shell to the Docker VM you created when you setup Docker on your host machine. The following example is for connecting to the Docker machine named `default`.

       eval "$(docker-machine env default)"

2. Run `make docs` to build the internal documentation for the Habitat supervisor.
2. Run `make serve-docs` to run a small web server that exposes the documentation on port 9633. You can then read the docs at http://DOCKER_HOST:9633/.
