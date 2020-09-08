Use the Handler DSL to attach a callback to an event. If the event
occurs during a Chef Infra Client run, the associated callback is
executed. For example:

-   Sending email if a Chef Infra Client run fails
-   Aggregating statistics about resources updated during a Chef Infra
    Client runs to StatsD