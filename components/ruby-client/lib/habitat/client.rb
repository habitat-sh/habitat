#
# Copyright:: Copyright (c) 2016 Chef Software Inc.
# License:: Apache License, Version 2.0
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#

require_relative 'client/version'
require_relative 'exceptions'
require 'faraday'
require 'json'

module Habitat
  # Habitat Client
  #
  # This class is an API client to the Habitat Depot. It uses Faraday
  # under the covers to give us a nice HTTP interface.
  #
  class Client
    attr_reader :depot, :connection

    # Creates the Habitat client connection object. The public
    # interface should be used for the common interactions with the
    # API, but more complex operations can be done using the
    # +connection+ attribute through Faraday.
    #
    # === Attributes
    #
    # * +depot+ - URL to the Habitat Depot, defaults to the public
    #             service run by the Habitat organization
    # * +connection+ - A Faraday object representing the HTTP connection
    #
    # === Examples
    #
    #    hc = Habitat::Client.new
    #    hc = Habitat::Client.new('https://depot.habitat.sh')
    #    hc.connection.get('/pkgs')

    def initialize(depot = 'http://willem.habitat.sh:9636/v1/depot')
      @depot = depot
      @connection = Faraday.new(url: @depot) do |f|
        f.request :multipart
        f.request :url_encoded
        f.adapter :net_http
      end
    end

    # Downloads the specified key. By default, it will download to the
    # current directory as a file named by the +X-Filename+ HTTP
    # header.
    def fetch_key(_key, _path = '.')
      raise 'Downloading keys is not yet implemented'
    end

    # Uploads the specified key from the given path location.
    def put_key(_key, _path)
      raise 'Uploading keys is not yet implemented'
    end

    # Downloads the specified package. It will default to the latest
    # version if not specified, or the latest release of a version if
    # the release is not specified.
    #
    # The file will be downloaded to the current directory as a file
    # named by the +X-Filename+ HTTP header.
    #
    # === Arguments
    #
    # * +pkg+ - A package string, like +core/zlib+
    def fetch_package(pkg, path = '.')
      download(fetch_package_path(pkg), path)
    end

    # Show details about the specified package using a package
    # identifier string. If the version or release are not specified,
    # +latest+ is assumed.
    #
    # === Examples
    #
    #    hc = Habitat::Client.new
    #    hc.show_package('core/zlib')
    #    hc.show_package('core/zlib/1.2.8')
    #    hc.show_package('core/zlib/1.2.8/20160222155343')
    def show_package(pkg)
      JSON.parse(@connection.get(show_package_path(pkg)).body)
    end

    # Uploads a package from the specified filepath.
    #
    # === Arguments
    #
    # * +file+ - The file to upload
    #
    # === Examples
    def put_package(file)
      upload(upload_package_path(file), file)
    end

    # Promotes a package to the specified view, for example
    # +core/zlib+ to +staging+ or +production+
    #
    # === Examples
    #
    #    hc = Habitat::Client.new
    #    hc.promote_package('core/pandas/0.0.1/20160419213120', 'staging')
    def promote_package(pkg, view)
      promote(promote_artifact_path(pkg, view))
    end

    private

    # Returns the PackageIdent as a string with the version and/or
    # release qualified if not specified. For example, if +pkg+ is
    # +'core/zlib'+, it will return +'core/zlib/latest'+.
    def package_ident(pkg)
      PackageIdent.new(*pkg.split('/')).to_s
    end

    # If the PackageIdent has four parts, it's fully qualified.
    def fully_qualified?(pkg)
      parts = package_ident(pkg).split('/')
      parts.count == 4 && parts.last != 'latest'
    end

    # Returns a URL path for retrieving the PackageIdent specified.
    def show_package_path(pkg)
      ['pkgs', package_ident(pkg)].join('/')
    end

    # Returns a URL path for retrieving the PackageIdent specified
    # using the download path.
    def fetch_package_path(pkg)
      resolved_path = resolve_latest(package_ident(pkg))
      ['pkgs', resolved_path, 'download'].join('/')
    end

    # Returns a URL path for retrieving the specified key
    def fetch_key_path(key)
      ['keys', key].join('/')
    end

    # Returns a URL path for uploading the specified package artifact
    def upload_package_path(path)
      ['pkgs', derive_pkgid_from_file(path)].join('/')
    end

    # Resolves the latest version of the package returned by +show_package+
    def resolve_latest(pkg)
      latest = show_package(pkg)
      %w(origin name version release).map { |i| latest['ident'][i] }.join('/')
    end

    def validate_pkg_path!(pkg)
      # rubocop:disable GuardClause
      unless fully_qualified?(pkg)
        raise <<-EOH.gsub(/^\s+/, '')
          You must specify a fully qualified package path, such as:

              core/pandas/0.0.1/20160419213120

          You specified `#{pkg}' in `#{caller_locations(1, 1)[0].label}'
        EOH
      end
    end

    # Opens a Habitat Artifact and reads the IDENT metadata to get the
    # fully qualified package identifier.
    #
    def derive_pkgid_from_file(file)
      require 'mixlib/shellout'
      tail_n = 'tail -n +6'
      xzcat = 'xzcat --decompress'
      tar_toc = 'tar -tf -'
      tar_stdout = 'tar -xOf -'
      subcommand = "#{tail_n} #{file} | #{xzcat} | #{tar_toc} | grep '/IDENT$'"
      command = "#{tail_n} #{file} | #{xzcat} | #{tar_stdout} $(#{subcommand})"
      pkgid = Mixlib::ShellOut.new(command)
      begin
        pkgid.run_command.stdout.chomp
      rescue
        raise "Could not derive a version from #{file}, aborting!"
      end
    end

    # Returns a hex encoded blake2b hash from the given data.
    def blake2b_checksum(data, size = 32)
      require 'rbnacl'
      require 'digest'
      Digest.hexencode(RbNaCl::Hash.blake2b(data, digest_size: size)).chomp
    end

    # Returns the URL path for promoting an artifact
    def promote_artifact_path(pkg, view)
      validate_pkg_path!(pkg)
      ['views', view, 'pkgs', pkg, 'promote'].join('/')
    end

    def promote(url)
      response = @connection.post(url)
      if response.status == 200
        return true
      else
        raise Habitat::PromotionError,
              "Depot returned #{response.status} on promote"
      end
    end

    # Downloads the file from the depot.
    #
    def download(url, path = '.')
      response = @connection.get(url)
      if response.status == 200
        ::File.open(
          ::File.join(
            path,
            response.headers['x-filename']
          ), 'wb') do |fp|
          fp.write(response.body)
        end
      else
        raise Habitat::DownloadError,
              "Depot returned #{response.status} on download"
      end
    end

    # Uploads a file to the depot.
    #
    # === Attributes
    # * +url+ - the URL path on the depot server
    # * +path+ - the file path to upload
    #
    def upload(url, path)
      payload = { file: Faraday::UploadIO.new(path, 'application/octet-stream') }
      response = @connection.post do |req|
        req.url url
        req.params['checksum'] = blake2b_checksum(IO.read(path))
        req.body = payload
      end

      if response.status == 200
        return true
      else
        raise Habitat::UploadError,
              "Depot returned #{response.status} on uploading '#{path}'"
      end
    end
  end

  # Habitat Package Ident
  #
  # This class builds a Habitat Package Identifier object using the
  # four components of a Habitat package: origin, package name (pkg),
  # version, and release. It subclasses +Struct+ with arguments that
  # have default values - 'latest' for +version+ and +release+. It
  # also implements a method to return the package identifier as a +/+
  # separated string for use in the Habitat Depot API
  # rubocop:disable StructInheritance
  class PackageIdent < Struct.new(:origin, :pkg, :version, :release)
    # Creates the package identifier using the origin and package
    # name. Optionally will also use the version and release, or sets
    # them to latest if not specified.
    #
    # === Attributes
    #
    # * +origin+ - The package origin
    # * +pkg+ - The package name
    # * +version+ - The version of the package
    # * +release+ - The timestamp release of the package
    #
    # === Examples
    #
    # Use the latest core/zlib package:
    #
    #    Habitat::PackageIdent.new('core', 'zlib')
    #
    # Use version 1.2.8 of the core/zlib package:
    #
    #    Habitat::PackageIdent.new('core', 'zlib', '1.2.8')
    #
    # Use a specific release of version 1.2.8 of the core/zlib
    # package:
    #
    #     Habitat::PackageIdent.new('core', 'zlib', '1.2.8', '20160222155343')
    #
    # Pass an array as an argument:
    #
    #    Habitat::PackageIdent.new(*['core', 'zlib'])
    #
    # For example, from a +#split+ string:
    #
    #    Habitat::PackageIdent.new(*'core/zlib'.split('/'))

    def initialize(origin, pkg, version = 'latest', release = 'latest')
      super
    end

    # Returns a string from a +Habitat::PackageIdent+ object separated by
    # +/+ (forward slash).
    #
    # === Examples
    #
    #     zlib = Habitat::PackageIdent.new('core', 'zlib')
    #     zlib.to_s #=> "core/zlib/latest"

    def to_s
      parts = if self[:version] == 'latest'
                [self[:origin], self[:pkg], self[:version]]
              else
                [self[:origin], self[:pkg], self[:version], self[:release]]
              end
      parts.join('/')
    end
  end
end

# So users don't have to remember +Habitat+ vs +Hab+
module Hab
  include Habitat
end
