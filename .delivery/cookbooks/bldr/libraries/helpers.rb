module BldrDocker
  class << self
    include Chef::Mixin::ShellOut
  end

  def self.devshell_name
    'habitat/devshell:latest'
  end

  def self.image_exists?(repository)
    !shell_out!(%{docker images -q #{repository}}).stdout.empty?
  end

  def self.fresh_image?(repository)
    if image_exists?(repository)
      require 'time'

      output = shell_out!(
        %{docker inspect --format='{{.Created}}' #{repository}}
      ).stdout
      created = Time.parse(output).utc
      now = Time.new.utc

      # Has this image been created in the last 24 hours?
      (now - created) < (60 * 60 * 24)
    else
      false
    end
  end
end

module BldrDockerMachine
  def self.available?(port = 2376)
    require 'socket'

    begin
      an_socket = TCPSocket.new(machine_ip, port)
    rescue
      Chef::Log.debug('Could not establish connection to docker machine')
      return false
    end

    an_socket.close

    true
  end

  def self.load_config
    require 'json'

    begin
      config = Chef::DataBagItem.load('bldr-acceptance', 'bldr-docker-machine')['config']
    rescue Net::HTTPServerException
      if ::File.exist?(::File.join(machine_home, 'config.json'))
        config = ::JSON.parse(IO.read(::File.join(machine_home, 'config.json')))
      else
        config = {}
      end
    end

    config ? config : {}
  end

  def self.dbuild_machine_dir
    require 'etc'

    ::File.join(
      Etc.getpwnam('dbuild').dir,
      '.docker/machine/machines',
      'bldr-docker-machine'
    )
  end

  def self.machine_home
    ::File.join(ENV['HOME'], '.docker/machine/machines', 'bldr-docker-machine')
  end

  def self.machine_ip
    if load_config.keys.include?('Driver') && load_config.keys.include?('IPAddress')
      ip = load_config['Driver']['IPAddress']
    elsif load_config.keys.include?('IPAddress')
      ip = load_config['IPAddress']
    end

    ip
  end
end
