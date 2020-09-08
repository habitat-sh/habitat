The `run_report_unsafe` method is used to run the report handler without
any error handling. This method should never be used directly in any
handler, except during testing of that handler. For example:

``` ruby
def run_report_unsafe(run_status)
  @run_status = run_status
  report
end
```