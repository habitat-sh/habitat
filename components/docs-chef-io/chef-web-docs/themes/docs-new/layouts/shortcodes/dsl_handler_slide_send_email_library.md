Use a library to define the code that sends email when a Chef Infra
Client run fails. Name the file `helper.rb` and add it to a cookbook's
`/libraries` directory:

``` ruby
require 'net/smtp'

module HandlerSendEmail
  class Helper

    def send_email_on_run_failure(node_name)

      message = "From: Chef <chef@chef.io>\n"
      message << "To: Grant <grantmc@chef.io>\n"
      message << "Subject: Chef run failed\n"
      message << "Date: #{Time.now.rfc2822}\n\n"
      message << "Chef run failed on #{node_name}\n"
      Net::SMTP.start('localhost', 25) do |smtp|
        smtp.send_message message, 'chef@chef.io', 'grantmc@chef.io'
      end
    end
  end
end
```