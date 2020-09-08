A data bag is a container of related data bag items, where each
individual data bag item is a JSON file. knife can load a data bag item
by specifying the name of the data bag to which the item belongs and
then the filename of the data bag item. The only structural requirement
of a data bag item is that it must have an `id`:

``` javascript
{
  /* This is a supported comment style */
  // This style is also supported
  "id": "ITEM_NAME",
  "key": "value"
}
```

where

-   `key` and `value` are the `key:value` pair for each additional
    attribute within the data bag item
-   `/* ... */` and `// ...` show two ways to add comments to the data
    bag item