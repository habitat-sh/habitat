The `run_report_safely` method is used to run the report handler,
rescuing and logging errors that may arise as the handler runs and
ensuring that all handlers get a chance to run during a Chef Infra
Client run (even if some handlers fail during that run). In general,
this method should never be used as an interface in a custom handler
unless this default behavior simply must be overridden.

``` ruby
def run_report_safely(run_status)
  run_report_unsafe(run_status)
rescue Exception => e
  Chef::Log.error('Report handler #{self.class.name} raised #{e.inspect}')
  Array(e.backtrace).each { |line| Chef::Log.error(line) }
ensure
  @run_status = nil
end
```