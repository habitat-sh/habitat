In chef-shell, it is possible to get verbose debugging using the tracing
feature in Interactive Ruby (IRb). chef-shell provides a shortcut for
turning tracing on and off. For example:

``` bash
chef > tracing on
tracing is on
=> nil
chef >
```

and:

``` bash
chef > tracing off
#0:(irb):2:Object:-: tracing off
#0:/opt/chef-workstation/embedded/lib/ruby/gems/2.6.0/gems/chef-15.8.23/lib/chef/shell/ext.rb:109:Shell::Extensions::ObjectCoreExtensions:>:       def off
#0:/opt/chef-workstation/embedded/lib/ruby/gems/2.6.0/gems/chef-15.8.23/lib/chef/shell/ext.rb:110:Shell::Extensions::ObjectCoreExtensions:-:         :off
#0:/opt/chef-workstation/embedded/lib/ruby/gems/2.6.0/gems/chef-15.8.23/lib/chef/shell/ext.rb:111:Shell::Extensions::ObjectCoreExtensions:<:       end
#0:/opt/chef-workstation/embedded/lib/ruby/gems/2.6.0/gems/chef-15.8.23/lib/chef/shell/ext.rb:272:main:>:       def tracing(on_or_off)
#0:/opt/chef-workstation/embedded/lib/ruby/gems/2.6.0/gems/chef-15.8.23/lib/chef/shell/ext.rb:273:main:-:         conf.use_tracer = on_or_off.on_off_to_bool
#0:/opt/chef-workstation/embedded/lib/ruby/gems/2.6.0/gems/chef-15.8.23/lib/chef/shell/ext.rb:162:Shell::Extensions::Symbol:>:       def on_off_to_bool
#0:/opt/chef-workstation/embedded/lib/ruby/gems/2.6.0/gems/chef-15.8.23/lib/chef/shell/ext.rb:163:Shell::Extensions::Symbol:-:         to_s.on_off_to_bool
#0:/opt/chef-workstation/embedded/lib/ruby/gems/2.6.0/gems/chef-15.8.23/lib/chef/shell/ext.rb:149:Shell::Extensions::String:>:       def on_off_to_bool
#0:/opt/chef-workstation/embedded/lib/ruby/gems/2.6.0/gems/chef-15.8.23/lib/chef/shell/ext.rb:150:Shell::Extensions::String:-:         case self
#0:/opt/chef-workstation/embedded/lib/ruby/gems/2.6.0/gems/chef-15.8.23/lib/chef/shell/ext.rb:154:Shell::Extensions::String:-:           false
#0:/opt/chef-workstation/embedded/lib/ruby/gems/2.6.0/gems/chef-15.8.23/lib/chef/shell/ext.rb:158:Shell::Extensions::String:<:       end
#0:/opt/chef-workstation/embedded/lib/ruby/gems/2.6.0/gems/chef-15.8.23/lib/chef/shell/ext.rb:164:Shell::Extensions::Symbol:<:       end
tracing is off
=> nil
chef >
```