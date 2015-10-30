# Wonder [![Build Status](https://travis-ci.org/reset/wonder.png?branch=master)](https://travis-ci.org/reset/wonder)

An [Erlang](http://www.erlang.org/doc/design_principles/gen_server_concepts.html)/[Elixir](http://elixir-lang.org/docs/stable/elixir/GenServer.html#content) inspired actor library for Rust

## Requirements

* stable/nightly Rust

## Quick Start

Define your actor

```rust
pub struct MyActor;
```

Implement the GenServer trait

```rust
impl GenServer for MyActor {
    type T = MyMessage;
    type S = ();
    type E = MyError;

    fn init(&self, _tx: &ActorSender<Self::T>, state: &mut Self::S) -> InitResult<Self::E> {
      // initialization implementation
      Ok(None)
    }

    // Implement overrides for default implementations for additional callbacks
}
```

Start your actor

```rust
use wonder::actor;

fn main() {
    let actor = actor::Builder::new(MyActor).start(()).unwrap();
}
```

## GenServer Trait

To create a GenServer actor you need to create a struct (perhaps a [unit-like struct](https://doc.rust-lang.org/book/structs.html#unit-like-structs)?) and implement the GenServer trait.

```
struct MyActor;
```

### Associated Types

To implement the GenServer trait you need to define 3 [associated types](https://doc.rust-lang.org/book/associated-types.html)

1. `T` - A public enum type for messages to be sent to and from the GenServer.
1. `S` - The state which the actor will own and maintain. `()` can be provided in the case of an actor who will manage no state.
1. `E` - A public enum type for errors to be returned by your GenServer.

```rust
struct MyState {
    pub initialized: bool,
}

#[derive(Debug)]
enum MyError {
    DirtyState,
}

#[derive(Debug)]
enum MyMessage {
    State(bool),
    GetState,
    SetState(bool),
}

impl GenServer for MyActor {
    type T = MyMessage;
    type S = MyState;
    // type S = (); // no state
    type E = MyError;

    // ... callbacks
}
```

> note: It is required for both the custom error and message enums to derive the Debug trait.

### Callbacks

The GenServer trait exposes 4 callbacks; one of which is required while the remaining three have default implementations making them optional to implement.

### `init/3 -> InitResult<E>`

> analogous to [GenServer:init/1](http://elixir-lang.org/docs/stable/elixir/GenServer.html#c:init/1)

Handles initialization of the newly created actor. As with proccess initialization in Elixir/Erlang, this is a blocking call. This function will be called once while the actor is starting.

The init function must returns an `InitResult` which is either `Ok(Option<u64>)` or `Err(E)` where E is your custom error type. The optional `u64` is the timeout value (in milliseconds) for the actor.

```rust
impl GenServer for MyActor {
    type T = MyMessage;
    type S = MyState;
    type E = MyError;

    fn init(&self, atx: &ActorSender<Self::T>, state: &mut Self::S) -> InitResult<Self::E> {
        // perform some initialization
        Ok(None)
    }
}
```

Parameters

- `atx` - Channel sender for the running actor. This is equivalent to sending to "self" in Elixir/Erlang.
- `state` - A mutable reference to the state that the running actor owns.

> note: `InitResult` is analogous to the return tuple for the `init/1` callback in Elixir; ie. `{:ok, state}` or `{:stop, reason}`

### `handle_call/5 -> HandleResult<T>` (optional)

> analogous to [GenServer:handle_call/3](http://elixir-lang.org/docs/stable/elixir/GenServer.html#c:handle_call/3)

Handles synchronous messages sent to the running actor.

```rust
impl GenServer for MyActor {
    type T = MyMessage;
    type S = MyState;
    type E = MyError;

    // ... other callbacks ...

    fn handle_call(&self, msg: Self::T, tx: &ActorSender<Self::T>,
        atx: &ActorSender<Self::T>, state: &mut Self::S) -> HandleResult<Self::T> {
        match msg {
            MyMessage::GetState => HandleResult::Reply(MyMessage::State(state.initialized), None),
            MyMessage::SetState(value) => {
                state.initialized = value;
                HandleResult::Reply(MyMessage::Ok, None)
            }
            _ => HandleResult::Stop(StopReason::Fatal(String::from("unhandled call")), None),
        }
    }
}
```

Parameters

- `tx` - Channel sender for the caller who started the actor. This is equivalent to sending to the caller's PID in Elixir/Erlang.
- `atx` - Same as `init/3`.
- `state` - Same as `init/3`.

> note: `HandleResult` is analogous to the return tuple for the `handle_call/3` callback in Elixir/Erlang.

### `handle_cast/4 -> HandleResult<T>` (optional)

> analogous to [GenServer:handle_cast/2](http://elixir-lang.org/docs/stable/elixir/GenServer.html#c:handle_cast/2)

Handles asynchronous messages sent to the running actor.

```rust
impl GenServer for MyActor {
    type T = MyMessage;
    type S = MyState;
    type E = MyError;

    // ... other callbacks ...

    fn handle_cast(&self, msg: Self::T, atx: &ActorSender<Self::T>, state: &mut Self::S) -> HandleResult<Self::T> {
        match msg {
            MyMessage::SetState(value) => {
                state.initialized = value;
                HandleResult::NoReply(None)
            },
            _ => HandleResult::NoReply(None)
        }
    }
}
```

Parameters

- `atx` - Same as `init/3`
- `state` - Same as `init/3`

> note: `HandleResult` is analogous to the return tuple for the `handle_cast/2` callback in Elixir/Erlang.

### `handle_timeout/3 -> HandleResult<T>` (optional)

> analogous to matching `handle_info(:timeout, _state)` in Elixir

Called when a timeout is set and the required amount of time has elapsed. A timeout can be set by including a timeout value in a `HandleResult` or `InitResult` returned by one of the GenServer callbacks.

The timeout can be used for various reason, but a great example is a pattern to perform late initialization. If your actor has a long running initialization period you can timeout immediately and perform initialization within the `handle_timeout` callback.

```rust
impl GenServer for MyActor {
    type T = MyMessage;
    type S = MyState;
    type E = MyError;

    fn init(&self, atx: &ActorSender<Self::T>, state: &mut Self::S) -> InitResult<Self::E> {
        Ok(Some(0))
    }

    fn handle_timeout(&self, atx: &ActorSender<Self::T>, state: &mut Self::S) -> HandleResult<Self::T> {
        // long running function for late initialization
        HandleResult::NoReply(None)
    }
}
```

Parameters

- `atx` - Same as `init/3`
- `state` - Same as `init/3`

> note: `HandleResult` is analogous to the return tuple for the `handle_info/2` callback in Elixir/Erlang.

## Authors

Jamie Winsor (<jamie@vialstudios.com>)
