Use this option to define a `run_list` object. For example, a JSON file
similar to:

``` javascript
"run_list": [
  "recipe[base]",
  "recipe[foo]",
  "recipe[bar]",
  "role[webserver]"
],
```

may be used by running `chef-client -j path/to/file.json`.

In certain situations this option may be used to update `normal`
attributes.