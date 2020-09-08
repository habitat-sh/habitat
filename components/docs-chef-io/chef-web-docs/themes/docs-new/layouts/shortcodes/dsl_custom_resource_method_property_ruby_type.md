The property ruby_type is a positional parameter. Use to ensure a
property value is of a particular ruby class, such as `true`, `false`,
`nil`, `String`, `Array`, `Hash`, `Integer`, `Symbol`. Use an array of
ruby classes to allow a value to be of more than one type. For example:

``` ruby
property :aaaa, String
```

``` ruby
property :bbbb, Integer
```

``` ruby
property :cccc, Hash
```

``` ruby
property :dddd, [true, false]
```

``` ruby
property :eeee, [String, nil]
```

``` ruby
property :ffff, [Class, String, Symbol]
```

``` ruby
property :gggg, [Array, Hash]
```