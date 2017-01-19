#!/usr/bin/env ruby
# Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.


# cat *.swimtrace | sort | ruby ~/src/habitat/components/swim/bin/trace-sequence.rb >! sequence.txt | java -DPLANTUML_LIMIT_SIZE=81920 -Xmx1024m  -jar ~/Downloads/plantuml.jar -verbose sequence.txt

output = [];
actors = {};

pattern = Regexp.new('(.*)\^(.*)\^(.*)\^(.*)\^(.*)\^(.*)\^(.*)\^(.*)\^(.*)\^(.*)\^(.*)\^(.*)');
$stdin.each_line do |line|
  if match = pattern.match(line)
    time = match[1]
    kind = match[2]
    thread_name = match[3]
    module_path = match[4]
    line_no = match[5]
    server_name = match[6]
    member_id = match[7]
    to_member_id = match[8]
    listening = match[9]
    to_addr = match[10]
    swim = match[11]
    rumor = match[12]
    actors[member_id] = true;
    case kind
    when /^ProbeConfirmed$/
      output.push "\"#{member_id}\" -[#red]-> \"#{to_member_id}\" : #{kind}"
    when /^ProbeSuspect$/
      output.push "\"#{member_id}\" -[#orange]-> \"#{to_member_id}\" : #{kind}"
    when /^Probe.+/
      output.push "\"#{member_id}\" -[#black]-> \"#{to_member_id}\" : #{kind}"
    when /.+Ping$/
      output.push "\"#{member_id}\" -[#blue]-> \"#{to_member_id}\" : #{kind} #{swim}"
    when /.+PingReq$/
      output.push "\"#{member_id}\" -[#yellow]-> \"#{to_member_id}\" : #{kind} #{swim}"
    when /.+Ack$/
      output.push "\"#{member_id}\" -[#green]-> \"#{to_member_id}\" : #{kind} #{swim}"
    when /.+Rumor$/
      output.push "\"#{member_id}\" -[#purple]-> \"#{to_member_id}\" : #{kind} #{rumor}"
    when /^MemberUpdate$/
      output.push "== #{member_id} sees #{rumor} =="
    when /^TestEvent$/
      output.push "== TEST #{rumor} TEST =="
    else
      output.push "\"#{member_id}\" -[#black]-> \"#{to_member_id}\" : #{kind}"
    end
  else
    puts "Failed to match #{line}"
  end
end
output.push "@enduml"
actors.keys.sort.reverse.each do |actor|
  output.unshift "participant \"#{actor}\""
end
output.unshift "@startuml"

puts output.join("\n")
