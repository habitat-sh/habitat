# Spider

Spider is a tool for building and querying the reverse dependency graph of
Habitat packages. It is intended to be used primarily by Habitat developers.

The tool scans the specified path in the file system to read metadata from .hart
files to construct the graph. Graph construction can take a while for a large
corpus of files.

## Features

* Interactive shell
* Query the reverse dependency graph of a given package
* Find the top packages that have the highest number of reverse dependencies
* Find the fully qualified package names from a given search phrase
* Print statistics about the reverse dependency graph

## Usage

To use it, do the following:

```
$ hab-spider <path to packages>
```

Once the tool starts up and completes reading the input files, it will
present some basic stats and the open an interactive shell for commands.

Example run (on a small dataset):

```
hab-spider pkgs
Crawling packages... please wait.
OK: 65 nodes, 63 edges (PT4.848970454S sec)

Available commands: HELP, STATS, TOP, FIND, RDEPS, EXIT

spider> help
HELP           - print this message
STATS          - print graph statistics
TOP [<count>]  - print nodes with the most reverse dependencies
FIND  <term>   - find packages that match the search term
RDEPS <name>   - print the reverse dependencies for the package
EXIT           - exit the application

spider> stats
Node count: 65
Edge count: 63
Connected components: 5
Is cyclic: false

spider> find cacerts
OK: 1 items (PT0.000083179S sec)

core/cacerts/2016.04.20/20160612081125

spider> rdeps core/cacerts/2016.04.20/20160612081125
OK: 3 items (PT0.000849052S sec)

core/git/2.10.0/20160914215433
chefconf/habitat-demo/0.0.1/20160709
core/jfrog-cli/1.3.1/20160729210516
```
