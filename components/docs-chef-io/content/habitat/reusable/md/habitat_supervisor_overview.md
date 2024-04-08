Chef Habitat Supervisor is a process manager that has two primary responsibilities:

- A Supervisor runs your app's services. It starts, stops, updates, and monitors the services according to your plan.
- Supervisors can talk to each other. You can connect Supervisors together into a network and instruct them to send information to each other and take actions based on that information.

In the Supervisor you can define topologies for you application, such as leader-follower or standalone, or more complex applications that include databases. The supervisor also allows you to inject tunables into your application. Allowing you to defer decisions about how your application behaves until runtime.