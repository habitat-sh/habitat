Add `desired_state:` to set the desired state property for a resource.
This value may be `true` or `false`, and all properties default to true.

-   When `true`, the state of the property is determined by the state of
    the system
-   When `false`, the value of the property impacts how the resource
    executes, but it is not determined by the state of the system.

For example, if you were to write a resource to create volumes on a
cloud provider you would need define properties such as `volume_name`,
`volume_size`, and `volume_region`. The state of these properties would
determine if your resource needed to converge or not. For the resource
to function you would also need to define properties such as
`cloud_login` and `cloud_password`. These are necessary properties for
interacting with the cloud provider, but their state has no impact on
decision to converge the resource or not, so you would set
`desired_state` to `false` for these properties.

``` ruby
property :volume_name, String
property :volume_size, Integer
property :volume_region, String
property :cloud_login, String, desired_state: false
property :cloud_password, String, desired_state: false
```