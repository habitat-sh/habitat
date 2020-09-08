A resource may specify multiple packages and/or versions for platforms
that use Apt, Chocolatey, DNF, Homebrew, Pacman, or Zypper package managers.
Specifying multiple packages and/or versions allows a single transaction
to:

-   Download the specified packages and versions via a single HTTP
    transaction
-   Update or install multiple packages with a single resource during a
    Chef Infra Client run

For example, installing multiple packages:

``` ruby
package %w(package1 package2)
```

Installing multiple packages with versions:

``` ruby
package %w(package1 package2) do
  version [ '1.3.4-2', '4.3.6-1']
end
```

Upgrading multiple packages:

``` ruby
package %w(package1 package2)  do
  action :upgrade
end
```

Removing multiple packages:

``` ruby
package %w(package1 package2)  do
  action :remove
end
```

Purging multiple packages:

``` ruby
package %w(package1 package2)  do
  action :purge
end
```

Notifications, via an implicit name:

``` ruby
package %w(package1 package2)  do
  action :nothing
end

log 'call a notification' do
  notifies :install, 'package[package1, package2]', :immediately
end
```

<div class="admonition-note">

<p class="admonition-note-title">Note</p>

<div class="admonition-note-text">

Notifications and subscriptions do not need to be updated when packages
and versions are added or removed from the `package_name` or `version`
properties.



</div>

</div>