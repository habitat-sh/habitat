A data bag named `sample` contains four data bag items: `abc`, `bar`,
`baz`, and `quz`. All of the items in-between `bar` and `foo`,
inclusive, can be searched for using an inclusive search pattern.

To search using an inclusive range, enter the following:

``` bash
knife search sample "id:[bar TO foo]"
```

where square brackets (`[ ]`) are used to define the range.