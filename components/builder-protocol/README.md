# builder-protocol

Definitions for network protocol messages and serialization implementations for builder servers

# Adding a new message or protocol

* If you are adding a message to an existing protocol, find that protocol in the [protocols directory](/protocols)
* If you are adding a brand new protocol, add the new protocol as your_protocol.proto in the [protocols directory](/protocols)
* Add new messages to the .proto file.
  * i.e.
  ```
  message Origin {
    optional uint64 id = 1;
    optional string name = 2;
    optional uint64 owner_id = 3;
    optional string private_key_name = 4;
  }

  message OriginCreate {
    optional string name = 1;
    optional uint64 owner_id = 2;
    optional string owner_name = 3;
  }
  ```
* You will also need to implement some traits on your new messages.  Do this in src/your_protocol.rs
  * items need at least a Persistable trait implemented
  * actions need at least a Routable trait implemented
  * i.e.
  ```
  impl Persistable for Origin {
      type Key = u64;

      fn primary_key(&self) -> Self::Key {
          self.get_id()
      }

      fn set_primary_key(&mut self, value: Self::Key) {
          self.set_id(value);
      }
  }


  impl Routable for OriginCreate {
      type H = InstaId;

      fn route_key(&self) -> Option<Self::H> {
          Some(InstaId(self.get_owner_id()))
      }
  }
  ```

* Finally, run this command to autogenerate the rest of what is needed for your protocol

```bash
$ cargo build --features protocols
```

