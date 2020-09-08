Use `.run_action(:some_action)` at the end of a resource block to run
the specified action during the compile phase. For example:

``` ruby
build_essential 'Install compilers' do
  action :nothing
end.run_action(:install)
```

where `action` is set to `:nothing` to ensure the `run_action` is run
during the compile phase and not later during the execution phase.

This can be simplified by using the `compile_time` flag in Chef Infra
Client 16 and later versions:

``` ruby
build_essential 'Install compilers' do
  compile_time true
end
```

That flag both forces the resource to run at compile time and sets the
converge action to `:nothing`.

The following examples show when (and when not) to use `run_action`.

**Using Custom Resources preferred to forcing to compile time**

Compile time execution is often used to install gems before requiring
them in recipe code.

This is a poor pattern since gems may depend on native gems which
may require installing compilers at compile time.

``` ruby
build_essential 'Install compilers' do
  compile_time true
end

chef_gem 'aws-dsk' do
  compile_time true
end

require 'aws-sdk'
```

A better strategy is to move the code, which requires the gem, into
a custom resource. Since all the actions of custom resources run
at converge time, this defers requiring 
the gem until later in the overall Chef Infra Client execution. Unified
mode can also be used in the resource to eliminate compile/converge
mode issues entirely:

``` ruby
unified_mode true

action :run do
  build_essential 'Install compilers'

  chef_gem 'aws-sdk'

  require 'aws-sdk'
end
```

**Download and parse a configuration file**

A common use case is to download a configuration file, parse it, and then
use the values in templates and to control other configuration.

An important distinction to make is that the downloaded configuration file
only exists in a temporary state to be used by the Chef Infra Client. It will
not be used directly by the system or applications that are managed by the
Chef Infra Client.

To download and parse a JSON file and render it in a template, it makes sense
to download the file during compile time:

``` ruby
  # the remote_file is being downloaded to a temporary file
  remote_file "#{Chef::Config[:file_cache_path]}/users.json" do
    source "https://jsonplaceholder.typicode.com/users"
    compile_time true
  end

  # this parsing needs to happen after the remote_file is downloaded, but will
  # be executed at compile time.
  array = JSON.parse(IO.read("#{Chef::Config[:file_cache_path]}/users.json")

  # the `array.last["phone"]` expression here will also be evaluated at compile
  # time and must be lazied via wrapping the expresssion in `lazy {}`
  file "/tmp/phone_number.txt" do
    content array.last["phone"]
  end
```

This is considerably cleaner than the alternative of lazy evaluating both the parsing of the
JSON and the rendering of the data into the file template, which will happen if
the `remote_file` resource is not run at compile time:

``` ruby
  # the execution of this is now deferred
  remote_file "#{Chef::Config[:file_cache_path]}/users.json" do
    source "https://jsonplaceholder.typicode.com/users"
  end

  # it is necessary due to lexical scoping issues to create this variable here
  array = nil

  # the parsing of the JSON is now deferred due to the ruby_block
  ruby_block "parse JSON" do
    block do
      array = JSON.parse(IO.read("#{Chef::Config[:file_cache_path]}/users.json")
    end
  end

  # the argument to the content property must now also be deferred
  file "/tmp/phone_number.txt" do
    content lazy { array.last["phone"] }
  end
```

This is an example of code that overuses deferred execution, uses more "lazy" evaluation, and is
considerably harder to understand and write correctly.

**Notifications will not work**

Resources that are executed during the compile phase cannot notify other
resources. For example:

``` ruby
execute 'ifconfig'

package 'vim-enhanced' do
  compile_time true
  notifies :run, 'execute[ifconfig]', :immediately
end
```

A better approach in this type of situation is to install the package
before the resource collection is built to ensure that it is available
to other resources later on.

The best approach to this problem is to use `unified mode` which eliminates
the compile time and converge time distinction, while allowing notifications
to work correctly.

**Resources that are forced to compile time by default**

The `ohai_hint` and `hostname` resources run at compile time by default.

This is due to the fact that later resources may consume the node attributes which
are set by those resources leading to excessive use of `lazy` in subsequent
resources (and similar issues to the `remote_file` example above).

The `chef_gem` resource used to execute at compile time by default, but now we
recommend that users move code that executes at compile time to custom resources.
