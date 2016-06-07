# Supervisor

The supervisor is a process manager that has two primary responsibilities. First, it starts and monitors the child app service defined in the package. Second, it receives and acts upon configuration changes from other supervisors to which it is connected. A service will be reconfigured through hooks if its configuration has changed.

## The Supervisor Ring

Supervisors typically run in a network, which we refer to as a *ring* (although it is more like a peer-to-peer network rather than a circular ring). The ring can be very large; it could contain hundreds or thousands of supervisors. The membership list of this ring is maintained independently by each supervisor and is known as the *census*.

### Census

The census is the core of the service discovery mechanism in Habitat. It keeps track of every supervisor in the ring, and handles reading, writing, and serializing it with the discovery backend.

Each supervisor in the system is a *census entry*; taken together, they form a *census*. Operations to discover or mutate the state of the census happen through algorithms that arrive at the same conclusion given the same inputs.

An example is leader election; it's handled here by having a consistent (and simple) algorithm for selecting a leader deterministically for the group. We rely on the eventual consistency of every supervisor's census entry to elect a new leader in a reasonable amount of time.

## Supervisor HTTP API

The Habitat supervisor provides a HTTP API to expose cluster metadata, statistics, and general diagnostic information useful for monitoring and support in the form of a JSON document. It also provides detailed information about the Habitat package that it is supervising, including metadata such as the build and runtime dependencies and their versions.

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/concepts-director">Director</a></li>
</ul>
