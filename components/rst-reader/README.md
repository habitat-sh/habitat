## RST Reader

This is a tool for reading the binary RST file that Habitat supervisors write
to disk periodically.

### Motivation
When in the course of human events it becomes necessary for one or more persons
to debug the internal workings of a supervisor participating in a butterfly
network, having a way to view the contents of an RST file is very helpful. The
RST file is a serialized representation of butterfly's internal state. It
contains all of the rumors that butterfly is currently storing.

### Usage
`rst-reader` takes a path to an RST file as a required argument, e.g.

```
rst-reader /hab/sup/default/data/fe15223b3f014ce19cc9710ad3d6929a.rst
```

If you're not interested in doing a directory listing and copy/pasting a file
name every time you run this, you can use a trick like this:

```
rst-reader $(find /hab/sup/default/data -iname "*.rst")
```

It can sometimes be helpful to pass this invocation to `watch`, so that you can
notice changes to the file over time:

```
watch -n 1 'rst-reader $(find /hab/sup/default/data -iname "*.rst")'
```

If you are only interested in rumor counts and not their contents, you can pass
`-s` for a summary.

```
rst-reader -s $(find /hab/sup/default/data -iname "*.rst")
```
