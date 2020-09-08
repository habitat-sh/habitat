The following open source handlers are available from the Chef
community:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 87%" />
</colgroup>
<thead>
<tr class="header">
<th>Handler</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><a href="https://github.com/timops/ohai-plugins/blob/master/win32_svc.rb">Airbrake</a></td>
<td>A handler that sends exceptions (only) to Airbrake, an application that collects data and aggregates it for review.</td>
</tr>
<tr class="even">
<td><a href="https://github.com/rottenbytes/chef/tree/master/async_handler">Asynchronous Resources</a></td>
<td>A handler that asynchronously pushes exception and report handler data to a STOMP queue, from which data can be processed into data storage.</td>
</tr>
<tr class="odd">
<td><a href="https://github.com/ampledata/chef-handler-campfire">Campfire</a></td>
<td>A handler that collects exception and report handler data and reports it to Campfire, a web-based group chat tool.</td>
</tr>
<tr class="even">
<td><a href="https://github.com/DataDog/chef-handler-datadog">Datadog</a></td>
<td>A handler that collects Chef Infra Client stats and sends them into a Datadog newsfeed.</td>
</tr>
<tr class="odd">
<td><a href="https://github.com/mmarschall/chef-handler-flowdock">Flowdock</a></td>
<td>A handler that collects exception and report handler data and sends it to users via the Flowdock API..</td>
</tr>
<tr class="even">
<td><a href="https://github.com/imeyer/chef-handler-graphite/wiki">Graphite</a></td>
<td>A handler that collects exception and report handler data and reports it to Graphite, a graphic rendering application.</td>
</tr>
<tr class="odd">
<td><a href="https://github.com/jellybob/chef-gelf/">Graylog2 GELF</a></td>
<td>A handler that provides exception and report handler status (including changes) to a Graylog2 server, so that the data can be viewed using Graylog Extended Log Format (GELF).</td>
</tr>
<tr class="even">
<td><a href="https://rubygems.org/gems/chef-handler-growl">Growl</a></td>
<td>A handler that collects exception and report handler data and then sends it as a Growl notification.</td>
</tr>
<tr class="odd">
<td><a href="https://github.com/mojotech/hipchat/blob/master/lib/hipchat/chef.rb">HipChat</a></td>
<td>A handler that collects exception handler data and sends it to HipChat, a hosted private chat service for companies and teams.</td>
</tr>
<tr class="even">
<td><a href="https://rubygems.org/gems/chef-irc-snitch">IRC Snitch</a></td>
<td>A handler that notifies administrators (via Internet Relay Chat (IRC)) when a Chef Infra Client run fails.</td>
</tr>
<tr class="odd">
<td><a href="https://github.com/marktheunissen/chef-handler-journald">Journald</a></td>
<td>A handler that logs an entry to the systemd journal with the Chef Infra Client run status, exception details, configurable priority, and custom details.</td>
</tr>
<tr class="even">
<td><a href="https://github.com/b1-systems/chef-handler-httpapi/">net/http</a></td>
<td>A handler that reports the status of a Chef run to any API via net/HTTP.</td>
</tr>
<tr class="odd">
<td><a href="https://rubygems.org/gems/chef-handler-mail">Simple Email</a></td>
<td>A handler that collects exception and report handler data and then uses pony to send email reports that are based on Erubis templates.</td>
</tr>
<tr class="even">
<td><a href="https://github.com/sendgrid-ops/chef-sendgrid_mail_handler">SendGrid Mail Handler</a></td>
<td>A chef handler that collects exception and report handler data and then uses SendGrid Ruby gem to send email reports that are based on Erubis templates.</td>
</tr>
<tr class="odd">
<td><a href="http://onddo.github.io/chef-handler-sns/">SNS</a></td>
<td>A handler that notifies exception and report handler data and sends it to a SNS topic.</td>
</tr>
<tr class="even">
<td><a href="https://github.com/rackspace-cookbooks/chef-slack_handler">Slack</a></td>
<td>A handler to send Chef Infra Client run notifications to a Slack channel.</td>
</tr>
<tr class="odd">
<td><a href="http://ampledata.org/splunk_storm_chef_handler.html">Splunk Storm</a></td>
<td>A handler that supports exceptions and reports for Splunk Storm.</td>
</tr>
<tr class="even">
<td><a href="https://github.com/jblaine/syslog_handler">Syslog</a></td>
<td>A handler that logs basic essential information, such as about the success or failure of a Chef Infra Client run.</td>
</tr>
<tr class="odd">
<td><a href="https://rubygems.org/gems/chef-handler-updated-resources">Updated Resources</a></td>
<td>A handler that provides a simple way to display resources that were updated during a Chef Infra Client run.</td>
</tr>
<tr class="even">
<td><a href="http://onddo.github.io/chef-handler-zookeeper/">ZooKeeper</a></td>
<td>A Chef report handler to send Chef run notifications to ZooKeeper.</td>
</tr>
</tbody>
</table>