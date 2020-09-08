Invoke the library helper in a recipe:

``` ruby
Chef.event_handler do
  on :run_failed do
    HandlerSendEmail::Helper.new.send_email_on_run_failure(
      Chef.run_context.node.name
    )
  end
end
```

-   Use `Chef.event_handler` to define the event handler
-   Use the `on` method to specify the event type

Within the `on` block, tell Chef Infra Client how to handle the event
when it's triggered.