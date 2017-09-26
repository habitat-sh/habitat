#############################################################################
#
# Script to auto-create habitat core plan projects in a specified depot
#
# Usage: ruby project_create.rb <core-plans-dir> <projects-url> [<auth-token>]
#
# The projects-url should be in this form:
# http://app.acceptance.habitat.sh/v1/projects
#
# If <auth-token> is not specified, the script will default to looking for
# the HAB_AUTH_TOKEN environment variable.
#
#############################################################################

require 'erb'
require 'net/http'
require 'uri'
require 'json'

if ARGV.length < 2
  puts "Usage: project_create <core-plans-dir> <projects-url> [<auth-token>]"
  exit
end

source_dir = ARGV[0]
url = ARGV[1]

if ARGV.length > 2
  auth_token = ARGV[2]
else
  auth_token = ENV['HAB_AUTH_TOKEN']
end

template = File.read('project.erb')
renderer = ERB.new(template)

uri = URI.parse(url)
http = Net::HTTP.new(uri.host, uri.port)
http.use_ssl = (uri.scheme == "https")

Dir.chdir source_dir
Dir.open '.' do |root|
  root.each do |f|
      if f.index(".") != 0 && File.directory?(f)
        plan = f.to_s
        plan_path = File.join(f, 'plan.sh')

        if File.exists?(plan_path)
          puts "Creating project for #{plan}"
          req = Net::HTTP::Post.new(uri, {"Authorization" => "Bearer #{auth_token}"})
          req.body = renderer.result(binding)
          res = http.request(req)
          puts "Response: #{res}"
        else
          puts "WARNING: plan.sh not found at #{plan_path} - skipping"
        end
      end
  end
end
