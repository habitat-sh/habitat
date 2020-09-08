This resource can be **nameless**. Add the resource itself to your
recipe to get the default behavior:

``` ruby
build_essential
```

will behave the same as:

``` ruby
build_essential 'install tools'
```