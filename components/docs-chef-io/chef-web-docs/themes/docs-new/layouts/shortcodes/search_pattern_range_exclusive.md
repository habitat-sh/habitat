A data bag named `sample` contains four data bag items: `abc`, `bar`,
`baz`, and `quz`. All of the items that are exclusive to `bar` and `foo`
can be searched for using an exclusive search pattern.

To search using an exclusive range, enter the following:

``` bash
knife search sample "id:{bar TO foo}"
```

where curly braces (`{ }`) are used to define the range.