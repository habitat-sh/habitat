#!/usr/bin/env ruby

#############################################################################
#
# Script to auto-create bldr.tom file for the core plan repo
#
# Usage: ruby toml_create.rb <core-plans-dir>
#
#############################################################################

require 'erb'
require 'net/http'
require 'uri'
require 'json'

if ARGV.length < 2
  puts "Usage: toml_create <core-plans-dir> <destination path>"
  exit
end

source_dir = File.expand_path(ARGV[0])
dest_dir = File.expand_path(ARGV[1])

template = File.read(File.join(File.dirname(__FILE__), 'toml.erb'))
renderer = ERB.new(template)

destfile = File.open(File.join(dest_dir, '.bldr.toml'), 'w')

Dir.chdir source_dir
Dir.open '.' do |root|
  root.each do |f|
      if f.index(".") != 0 && File.directory?(f)
        plan = f.to_s
        plan_path = File.join(f, 'plan.sh')

        if File.exists?(plan_path)
          puts "Adding #{plan}"
          snippet = renderer.result(binding)
          destfile.write(snippet)
        else
          puts "WARNING: plan.sh not found at #{plan_path} - skipping"
        end
      end
  end
end

destfile.close()
