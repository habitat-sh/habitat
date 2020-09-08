The following example shows how to prevent concurrent Chef Infra Client
runs from both holding a lock on etcd:

``` ruby
lock_key = "#{node.chef_environment}/#{node.name}"

Chef.event_handler do
  on :converge_start do |run_context|
    Etcd.lock_acquire(lock_key)
  end
end

Chef.event_handler do
  on :converge_complete do
    Etcd.lock_release(lock_key)
  end
end
```