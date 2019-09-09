# Locking

In order to support its high degree of parallelism at runtime, Habitat uses
a number of different lock types. To avoid deadlock, we rely on conventions
around how these locks are acquired, held and released.

## Documenting behavior

Functions which acquire locks should have the details described in their doc
comment. Additionally, a suffix is applied to the function name to indicate
the locks that are acquired either directly or indirectly. The suffix is an
abbreviation of the lock name (see [Lock Ordering](#lock-ordering) section)
and an `r` or `w` to indicate a read or write lock, if applicable. If a
function takes both a read and write lock `w` suffices. The ordering of the
suffixes and doc comments should reflect the lock ordering.

When adding code which takes a lock, it's important add the appropriate doc
comment and suffix to the name of the function, and then propagate that
information up the call chain. Without this, it can be easy to violate the
lock ordering because a lock is acquired through a series of function calls.

For example, a caller which wishes to insert a new service rumor may not be
aware of the implementation details involved and mistakenly hold a
`MemberList` lock while doing the insert. This could lead to deadlock, so
to make this clear the function for inserting a service is named
`insert_service_rsw_mlw` to make it clear that both `RumorStore::list` and
`MemberList::entries` locks may be acquired during the execution of
`insert_service_rsw_mlw`.

## Lock Ordering

Whenever a thread needs to hold multiple locks concurrently, they
must be acquired in the conventional order and released in the reverse order:

1. `RumorStore::list` (`rs`)
1. `MemberList::initial_members` (`iml`)
1. `MemberList::entries` (`ml`)
1. `GatewayState::inner` (`gs`)
1. `Server::member` (`sm`)
1. `Server::block_list` (`sbl`)

Any function which is documented to acquire a lock should not be called with
any lock that occurs later in the lock order held. For example, since
`insert_service_config_rsw` calls functions which acquire the
`RumorStore::list` lock, calling it with the `MemberList::entries` lock held
violates the lock order and may lead to deadlock.

It's not necessary to acquire all intermediate locks. For example, a thread
may take `RumorStore::list`, followed by `MemberList::entries`, without
taking `MemberList::initial_members`. However, once `MemberList::entries`
is taken, subsequent logic must not take `MemberList::initial_members`,
unless `MemberList::entries` is released first.

Additional lock types will be added as work on https://github.com/habitat-sh/habitat/issues/6435
progresses.

See https://en.wikipedia.org/wiki/Dining_philosophers_problem

## Recursion

Recursive locking adds complexity, risks deadlock with the `std::sync` locks
(since the behavior is undefined) and precludes fairness. No locks should be
acquired recursively. Though recursive locking does exist in the codebase,
https://github.com/habitat-sh/habitat/issues/6435 tracks the process of
eliminating it.

Any function which is documented to acquire a lock should not be called with
that same lock already held.

## `std::sync` vs `parking_lot`

The interfaces for `Mutex` and `RwLock` are very similar between the stdlib
and the `parking_lot` crate, but the latter has better performance and more
features, including defined recursion semantics and the option to try to
acquire a lock with a timeout.

In general we prefer to move to the `parking_lot` implementations, but this
is a gradual process and should be undertaken with care.
