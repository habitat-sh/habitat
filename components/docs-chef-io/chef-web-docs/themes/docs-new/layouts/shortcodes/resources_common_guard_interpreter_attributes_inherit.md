The `guard_interpreter` property is set to `:default` by default for the
**bash**, **csh**, **perl**, **python**, and **ruby** resources. When
the `guard_interpreter` property is set to `:default`, `not_if` or
`only_if` guard statements **do not inherit** properties that are
defined by the **script**-based resource.

<div class="admonition-warning">

<p class="admonition-warning-title">Warning</p>

<div class="admonition-warning-text">

The **batch** and **powershell_script** resources inherit properties by
default. The `guard_interpreter` property is set to `:batch` or
`:powershell_script` automatically when using a `not_if` or `only_if`
guard statement within a **batch** or **powershell_script** resource,
respectively.



</div>

</div>

For example, the `not_if` guard statement in the following resource
example **does not inherit** the `environment` property:

``` ruby
bash 'javatooling' do
  environment 'JAVA_HOME' => '/usr/lib/java/jdk1.7/home'
  code 'java-based-daemon-ctl.sh -start'
  not_if 'java-based-daemon-ctl.sh -test-started'
end
```

and requires adding the `environment` property to the `not_if` guard
statement so that it may use the `JAVA_HOME` path as part of its
evaluation:

``` ruby
bash 'javatooling' do
  environment 'JAVA_HOME' => '/usr/lib/java/jdk1.7/home'
  code 'java-based-daemon-ctl.sh -start'
  not_if 'java-based-daemon-ctl.sh -test-started', :environment => 'JAVA_HOME' => '/usr/lib/java/jdk1.7/home'
end
```

To inherit properties, add the `guard_interpreter` property to the
resource block and set it to the appropriate value:

-   `:bash` for **bash**
-   `:csh` for **csh**
-   `:perl` for **perl**
-   `:python` for **python**
-   `:ruby` for **ruby**

For example, using the same example as from above, but this time adding
the `guard_interpreter` property and setting it to `:bash`:

``` ruby
bash 'javatooling' do
  guard_interpreter :bash
  environment 'JAVA_HOME' => '/usr/lib/java/jdk1.7/home'
  code 'java-based-daemon-ctl.sh -start'
  not_if 'java-based-daemon-ctl.sh -test-started'
end
```

The `not_if` statement now inherits the `environment` property and will
use the `JAVA_HOME` path as part of its evaluation.