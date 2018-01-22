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

## Troubleshooting

Butterfly includes support for dropping trace files, which can then be
transformed into diagrams, which make it possible to trace the flow of traffic
between members.

To use it, do the following:

```
$ mkdir -p /tmp/habitat-swim-trace
$ env TRACE_SWIM=1 yourthing
```

For example, to get a trace of a particular integration test:

```
$ mkdir -p /tmp/habitat-swim-trace
$ env TRACE_SWIM=1 cargo test --test integration two_members_meshed
```

This will result in files populating in the `/tmp/habitat-swim-trace`
directory. To look at the stream together as plain text:

```
$ cat /tmp/habitat-swim-trace/*.swimtrace | sort
```

This will put all the trace files together, ordered by time.

You can turn this into a UML State Transition diagram by using
[PlantUML](http://plantuml.com/). [Download the jar
file](http://plantuml.com/download), and then do the following:

```
$ cat /tmp/habitat-swim-trace/*.swimtrace | sort | ruby ./bin/trace-sequence.rb > sequence.txt && java -DPLANTUML_LIMIT_SIZE=163840 -Xmx8024m -jar plantuml.jar -verbose sequence.txt
```

Where `plantuml.jar` is the path to `plantuml`, and `./bin/trace-sequence.rb`
is the path to `components/swim/bin/trace-sequence.rb` in this repository.

The results here can be overwhelming. Judicious use of Grep can help. See the
full list of event types in `trace.rs`.

## Why is it called Butterfly?

It's named after the swimming stroke. Because it's not just SWIM-ing - get it?

Also, Butterflies are lovely. Who doesn't want a Butterfly in their habitat?
