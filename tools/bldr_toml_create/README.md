## Builder TOML Creation Tool

This is a development tool that can be used to create a ```bldr.toml``` file for
a given repo. This tool is targeted to creating a toml file for the core plans
repo.

It takes as input parameters a local path to the core-plans repo, and a
destination directory for the ```bldr.toml``` file.

### Usage

You need to have a recent Ruby installed.

To run:
```
ruby toml_create.rb <core-plans-dir> <destination-dir>
```
