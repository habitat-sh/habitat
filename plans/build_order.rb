#!/usr/bin/env ruby

require 'delegate'
require 'optparse'
require 'tsort'
require 'tempfile'

# Command line parser
class Cli
  def self.parse(argv) # rubocop:disable Metrics/MethodLength
    options = { with_base: true }
    parser = OptionParser.new do |opts|
      opts.banner = "Usage: #{File.basename(__FILE__)} [--without-base]"
      opts.on('--without-base', "Don't include base packages in result") do |_|
        options[:with_base] = false
      end
      opts.on('-h', '--help', 'Prints this help') do
        puts opts
        exit
      end
      opts.separator ''
      opts.separator 'Examples:'
      opts.separator "    find . -name plan.sh | #{__FILE__}"
      opts.separator "    find . -name plan.sh | #{__FILE__} --without-base"
    end
    parser.parse!(argv)
    options
  end
end

# Dependency tracker.
class Sortable < SimpleDelegator
  include TSort

  def add(ident, deps = [])
    __getobj__[ident] = deps
  end

  def tsort_each_node(&block)
    __getobj__.each_key(&block)
  end

  def tsort_each_child(node, &block)
    __getobj__.fetch(node).each(&block)
  end
end

options = Cli.parse(ARGV)

bash_prog = Tempfile.new('print_deps.sh')
bash_prog.write(<<'EOF')
#!/bin/bash
set -e
STUDIO_TYPE=stage1
FIRST_PASS=true

cd $(dirname $1)
source $(basename $1)
echo "${pkg_origin}/${pkg_name}"
echo "${pkg_build_deps[*]} ${pkg_deps[*]}"
exit 0
EOF
bash_prog.close

all_deps = Sortable.new({})
ARGF.each_line do |file|
  raw = `bash #{bash_prog.path} #{file}`.chomp
  ident, _, deps_str = raw.partition(/\n/)
  if ident.start_with?('core/')
    all_deps.add(ident, deps_str.split(' ')
      .map { |d| d.split('/').first(2).join('/') })
  end
end

all_deps.keys.each do |ident|
  all_deps[ident].each do |dep|
    all_deps.add(dep) unless all_deps.key?(dep)
  end
end

sorted_deps = all_deps.tsort

unless options[:with_base]
  prog = "#{File.dirname(__FILE__)}/build-base-plans.sh"
  raw = `env PRINT_IDENTS_ONLY=true #{prog}`.chomp
  base_deps = raw.split(/\n/)
  sorted_deps.delete_if { |dep| base_deps.include?(dep) }
end

puts sorted_deps
