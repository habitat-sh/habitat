#!/usr/bin/env ruby

# Shameless stolen from and inspired by the Automate manifest generator:
# https://github.com/chef/automate/blob/master/.expeditor/create-manifest.rb

require 'json'
require 'logger'

class ManifestUtil
  def generate(filename, log)

    manifest = {}
    manifest["schema_version"] = "1"
    # manifest["build"] = major_version(pkg_ident)
    manifest["packages"] = { }

    packages = []
    
    f = File.open(filename, { mode: 'r', encoding: 'UTF-8' })
    f.each_line do | line |
      packages << line.chomp
    end


    packages.uniq.each do | pkg |
      pkg_ident = pkg.split(' ')[0]
      pkg_target = pkg.split(' ')[1]
      add_product(manifest, pkg_ident, pkg_target, log)
    end

    manifest
  end

  def add_product(manifest, pkg_ident, pkg_target, log)
    new_package =  { 
      pkg_target => [ pkg_ident ]
    }
    
    # Check to see if we have added a something with this pkg_target
    if manifest["packages"].has_key?(pkg_target)
      # add to existing array
      manifest["packages"][pkg_target] << pkg_ident
    else
      # create first element of the array since we haven't had this pkg_target yet
      manifest["packages"][pkg_target] = [ (pkg_ident) ]
    end
    manifest
  end

  def major_version(pkg_ident)
    v = pkg_ident.split("/")[2]
  end
end

def usage(log)
  log.error "Usage:"
  log.error "create_manifest.rb <filename> - generate a manifest from the input list."
end

# setup logger
log = Logger.new(STDOUT)
log.formatter = proc do |severity, datetime, progname, msg|
  "#{datetime} #{severity}: #{msg}\n"
end

# Some basic help
if ARGV.nil? || ARGV.length == 0
  usage(log)
  exit
end

if (ARGV[0].nil?)
    log.error "This option requires a pkg_ident, pkg_target, and output filename."
    usage(log)
end

filename = ARGV[0]

manifest = ManifestUtil.new.generate(filename, log)
File.write("manifest.json", JSON.pretty_generate(manifest), { mode: 'w', encoding: 'UTF-8' })