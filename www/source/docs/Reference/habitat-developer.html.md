---
title: Habitat Developer Reference
---

# Habitat Developer Reference

This section contains information pertinent to the developer who is working on the [Habitat code](https://github.com/habitat-sh/habitat) itself. More up-to-date information, including instructions on how to set up your development environment to work on Habitat, can be obtained by visiting the GitHub repository.

## Contributing to Habitat Documentation

Habitat's website and documentation (which you are reading now) is generated using [Middleman](https://middlemanapp.com/) and is located in the `www` directory of the Habitat source code. To work on the documentation, you will need to have a working [Ruby](https://ruby-lang.org) installation and Bundler. We recommend Ruby 2.3.1 or greater.

To install Middleman, follow these instructions:

1. Change to the `www` directory and type:

       bundle install --path=vendor

To build the documentation, either before or after you make your change, follow these instructions:

1. Change to the `www` directory and type:

       bundle exec middleman build

2. The documentation is built into the `source` directory. You can instruct Middleman to serve the site by typing:

       bundle exec middleman serve

3. Middleman will start a small webserver on your computer and indicate what URL you should load in your browser to preview it.

       == View your site at "http://mylaptop.example.com:4567", "http://192.168.1.101:4567"

4. You can continue to make changes to the documentation files and Middleman will reload them live.
5. Press `Ctrl-C` to terminate the webserver when you are finished working with Middleman.
