A `Policyfile.rb` is a Ruby file in which run-list and cookbook
locations are specified. The syntax is as follows:

``` ruby
name "name"
run_list "ITEM", "ITEM", ...
default_source :SOURCE_TYPE, *args
cookbook "NAME" [, "VERSION_CONSTRAINT"] [, SOURCE_OPTIONS]
```