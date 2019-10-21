#!/usr/bin/env ruby

# Shameless stolen from and inspired by the Automate manifest generator:
# https://github.com/chef/automate/blob/master/.expeditor/create-manifest.rb

require 'json'
require 'logger'

class ManifestUtil
  def generate(version, sha, filename, log)

    manifest = {
      "schema_version" => "1",
      "version" => version,
      "sha" => sha,
      "packages" => { }
    }

    input_lines = []

    f = File.open(filename, { mode: 'r', encoding: 'UTF-8' })
    f.each_line do | line |
      input_lines << line.chomp
    end

    input_lines.uniq.each do | pkg |
      pkg_ident, pkg_target = pkg.split
      add_product(manifest, pkg_ident, pkg_target, log)
    end

    manifest
  end

  def add_product(manifest, pkg_ident, pkg_target, log)

    # Check to see if we have added a something with this pkg_target
    if manifest["packages"].has_key?(pkg_target)
      # add to existing array
      manifest["packages"][pkg_target] << pkg_ident
    else
      # create first element of the array since we haven't had this pkg_target yet
      manifest["packages"][pkg_target] = [ pkg_ident ]
    end
    manifest
  end

end

def usage(log)
  log.error "Usage:"
  log.error "create_manifest.rb <filename> <version> <sha> - generate a manifest from the input list."
  log.error "Each line of the input file should be of the form: \"$FULLY_QUALIFIED_IDENT $PKG_TARGET\""
  log.error "<version> is the contents of the VERSION file from the Habitat repository"
  log.error "<sha> is a Git SHA for the code the managed artifacts are built from."
end

# setup logger
log = Logger.new(STDOUT)
log.formatter = proc do |severity, datetime, progname, msg|
  "#{datetime} #{severity}: #{msg}\n"
end

# Some basic help
if ARGV.nil? || ARGV.length != 3
  usage(log)
  exit
end

filename = ARGV[0]
version = ARGV[1]
sha = ARGV[2]

manifest = ManifestUtil.new.generate(version, sha, filename, log)
File.write("manifest.json", JSON.pretty_generate(manifest), { mode: 'w', encoding: 'UTF-8' })
