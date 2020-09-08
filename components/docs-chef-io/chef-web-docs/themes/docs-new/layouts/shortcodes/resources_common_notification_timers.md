A timer specifies the point during a Chef Infra Client run at which a
notification is run. The following timers are available:

`:before`

:   Specifies that the action on a notified resource should be run
    before processing the resource block in which the notification is
    located.

`:delayed`

:   Default. Specifies that a notification should be queued up, and then
    executed at the end of a Chef Infra Client run.

`:immediate`, `:immediately`

:   Specifies that a notification should be run immediately, per
    resource notified.