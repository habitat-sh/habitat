# Butterfly

Butterfly is the Habitat gossip protocol. It's an implementation of
SWIM+Inf+Susp for membership, and a ZeroMQ based newscast-inspired gossip
protocol.

## Goals

* Eventually consistent. Over a long enough time horizon, every living member
  will converge on the same state.
* Reasonably efficient. The protocol avoids any back-chatter; messages are
  sent but never confirmed.
* Reliable. As a building block, it should be safe and reliable to use.

## Why is it called Butterfly?

It's named after the swimming stroke. Because it's not just SWIM-ing - get it?

Also, Butterflies are lovely. Who doesn't want a Butterfly in their habitat?
