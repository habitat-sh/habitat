Use the `on` method to create an event handler that sends email when a
Chef Infra Client run fails. This will require:

-   A way to tell Chef Infra Client how to send email
-   An event handler that describes what to do when the `:run_failed`
    event is triggered
-   A way to trigger the exception and test the behavior of the event
    handler