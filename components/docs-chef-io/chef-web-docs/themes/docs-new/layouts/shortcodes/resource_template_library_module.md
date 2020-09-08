A template helper module can be defined in a library. This is useful
when extensions need to be reused across recipes or to make it easier to
manage code that would otherwise be defined inline on a per-recipe
basis.

``` ruby
template '/path/to/template.erb' do
  helpers(MyHelperModule)
end
```