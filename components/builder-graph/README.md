# Builder-Graph

`bldr-graph` is a tool for building and querying the reverse dependency graph of
Habitat packages. It is intended to be used primarily by Habitat developers.

The tool connects to the builder-scheduler Postgres DB in order to construct the
graph. The database can be optionally specified and passed in a config file.

## Features

* Interactive shell
* Query the forward dependencies of a package
* Query the reverse dependency graph of a given package
* Find the top packages that have the highest number of reverse dependencies
* Find the fully qualified package names from a given search phrase
* Print statistics about the reverse dependency graph
* Check new package dependencies for version conflicts

## Usage

To use it, do the following:

```
$ bldr-graph [<path to config file>]
```

Once the tool starts up and completes reading the graph, it will
present some basic stats and the open an interactive shell for commands.

Example run:

```
$ bldr-graph

Connecting to builder_scheduler
Building graph... please wait.
OK: 1224 nodes, 3537 edges (PT1.758762699S sec)

Available commands: help, stats, top, find, resolve, filter, rdeps, deps, check, exit

command> help
Commands:
  help                    Print this message
  stats                   Print graph statistics
  top     [<count>]       Print nodes with the most reverse dependencies
  filter  [<origin>]      Filter outputs to the specified origin
  resolve <name>          Find the most recent version of the package 'origin/name'
  find    <term> [<max>]  Find packages that match the search term, up to max items
  rdeps   <name> [<max>]  Print the reverse dependencies for the package, up to max
  deps    <name>|<ident>  Print the forward dependencies for the package
  check   <name>|<ident>  Validate the latest dependencies for the package
  exit                    Exit the application

command> stats
Node count: 1224
Edge count: 3537
Connected components: 114
Is cyclic: false
command>
```
