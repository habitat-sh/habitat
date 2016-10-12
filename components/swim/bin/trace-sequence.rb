#!/usr/bin/env ruby
# Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

$stdin.each_line do |line|
  line =~ /(.+)!\*!(.+)!\*!(.+)!\*!(.+)!\*!(.+)!\*!(.+)!\*!(.+)!\*!(.+)!\*!(.+)!\*!(.+)/
  time_string = $1
  module_path = $2
  line = $3
  server_name = $4
  server_id = $5
  listening = $6
  thread_name = $7
  msg_type = $8
  to_addr = $9
  payload = $10

  actors[listening] = true;
  case msg_type
  when /^probe-marked-confirmed$/
    output.push "\"#{listening}\" -[#red]-> \"#{to_addr}\" : #{msg_type}"
  when /^probe-marked-suspect$/
    output.push "\"#{listening}\" -[#orange]-> \"#{to_addr}\" : #{msg_type}"
  when /^probe.+/
    output.push "\"#{listening}\" -[#black]-> \"#{to_addr}\" : #{msg_type}"
  when /.+ping$/
    output.push "\"#{listening}\" -[#blue]-> \"#{to_addr}\" : #{msg_type}"
  when /.+ack$/
    output.push "\"#{listening}\" -[#green]-> \"#{to_addr}\" : #{msg_type}"
  else
    output.push "\"#{listening}\" -[#black]-> \"#{to_addr}\" : #{msg_type}"
  end
end
output.push "@enduml"
actors.keys.sort.reverse.each do |actor|
  output.unshift "participant \"#{actor}\""
end
output.unshift "@startuml"

puts output.join("\n")
