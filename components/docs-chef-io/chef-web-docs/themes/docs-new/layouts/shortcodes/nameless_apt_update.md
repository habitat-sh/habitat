This resource can be **nameless**. Add the resource itself to your
recipe to get the default behavior:

``` ruby
apt_update
```

will behave the same as:

``` ruby
apt_update 'update'
```