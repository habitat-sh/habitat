By default, a file or directory inherits rights from its parent
directory. Most of the time this is the preferred behavior, but
sometimes it may be necessary to take steps to more specifically control
rights. The `inherits` property can be used to specifically tell Chef
Infra Client to apply (or not apply) inherited rights from its parent
directory.

For example, the following example specifies the rights for a directory:

``` ruby
directory 'C:\mordor' do
  rights :read, 'MORDOR\Minions'
  rights :full_control, 'MORDOR\Sauron'
end
```

and then the following example specifies how to use inheritance to deny
access to the child directory:

``` ruby
directory 'C:\mordor\mount_doom' do
  rights :full_control, 'MORDOR\Sauron'
  inherits false # Sauron is the only person who should have any sort of access
end
```

If the `deny_rights` permission were to be used instead, something could
slip through unless all users and groups were denied.

Another example also shows how to specify rights for a directory:

``` ruby
directory 'C:\mordor' do
  rights :read, 'MORDOR\Minions'
  rights :full_control, 'MORDOR\Sauron'
  rights :write, 'SHIRE\Frodo' # Who put that there I didn't put that there
end
```

but then not use the `inherits` property to deny those rights on a child
directory:

``` ruby
directory 'C:\mordor\mount_doom' do
  deny_rights :read, 'MORDOR\Minions' # Oops, not specific enough
end
```

Because the `inherits` property is not specified, Chef Infra Client will
default it to `true`, which will ensure that security settings for
existing files remain unchanged.