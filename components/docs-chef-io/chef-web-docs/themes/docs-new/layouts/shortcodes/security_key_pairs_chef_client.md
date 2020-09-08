Chef Infra Client authenticates with the Chef Infra Server using RSA
public key-pairs each time a Chef Infra Client needs access to data that
is stored on the Chef Infra Server. This prevents any node from
accessing data that it shouldn't and it ensures that only nodes that are
properly registered with the Chef Infra Server can be managed.