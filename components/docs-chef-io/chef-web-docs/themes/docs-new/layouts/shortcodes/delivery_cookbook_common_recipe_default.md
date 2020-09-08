Use the `default.rb` recipe to configure a project on a build node. This
recipe is run by Chef Infra Client as the root user and is a standard
default recipe, i.e. Chef Infra Client may use this recipe to configure
this project on any node, whether or not it's part of a Chef Automate
pipeline.