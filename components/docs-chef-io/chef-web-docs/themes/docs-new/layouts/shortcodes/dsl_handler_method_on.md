Use the `on` method to associate an event type with a callback. The
callback defines what steps are taken if the event occurs during a Chef
Infra Client run and is defined using arbitrary Ruby code. The syntax is
as follows:

``` ruby
Chef.event_handler do
  on :event_type do
    # some Ruby
  end
end
```

where

-   `Chef.event_handler` declares a block of code within a recipe that
    is processed when the named event occurs during a Chef Infra Client
    run
-   `on` defines the block of code that will tell Chef Infra Client how
    to handle the event
-   `:event_type` is a valid exception event type, such as `:run_start`,
    `:run_failed`, `:converge_failed`, `:resource_failed`, or
    `:recipe_not_found`

For example:

``` bash
Chef.event_handler do
  on :converge_start do
    puts "Ohai! I have started a converge."
  end
end
```