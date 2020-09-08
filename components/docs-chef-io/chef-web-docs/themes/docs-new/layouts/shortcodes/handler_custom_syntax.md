The syntax for a handler can vary, depending on what the the situations
the handler is being asked to track, the type of handler being used, and
so on. All custom exception and report handlers are defined using Ruby
and must be a subclass of the `Chef::Handler` class.

``` ruby
require 'chef/log'

module ModuleName
  class HandlerName < Chef::Handler
    def report
      # Ruby code goes here
    end
  end
end
```

where:

-   `require` ensures that the logging functionality of Chef Infra
    Client is available to the handler
-   `ModuleName` is the name of the module as it exists within the
    `Chef` library
-   `HandlerName` is the name of the handler as it is used in a recipe
-   `report` is an interface that is used to define the custom handler

For example, the following shows a custom handler that sends an email
that contains the exception data when a Chef Infra Client run fails:

``` ruby
require 'net/smtp'

module OrgName
  class SendEmail < Chef::Handler
    def report
      if run_status.failed? then
        message  = "From: sender_name <sender@example.com>\n"
        message << "To: recipient_address <recipient@example.com>\n"
        message << "Subject: chef-client Run Failed\n"
        message << "Date: #{Time.now.rfc2822}\n\n"
        message << "Chef run failed on #{node.name}\n"
        message << "#{run_status.formatted_exception}\n"
        message << Array(backtrace).join('\n')
        Net::SMTP.start('your.smtp.server', 25) do |smtp|
          smtp.send_message message, 'sender@example', 'recipient@example'
        end
      end
    end
  end
end
```

and then is used in a recipe like:

``` ruby
send_email 'blah' do
  # recipe code
end
```