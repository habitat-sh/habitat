
###############################################
### Build a docker machine worthy of Mordor ###
###         (or, acceptance...)             ###
###############################################
load_delivery_chef_config

creds = data_bag_item(
  'delivery-secrets',
  'chef-bldr-acceptance'
)

ENV['AWS_ACCESS_KEY_ID']     = creds['aws_access_key_id']
ENV['AWS_SECRET_ACCESS_KEY'] = creds['aws_secret_access_key']

remote_file '/usr/local/bin/docker-machine' do
  source 'https://github.com/docker/machine/releases/download/v0.5.3/docker-machine_linux-amd64'
  mode 00755
end

execute 'bldr-docker-machine' do
  command <<-EOH.gsub(/^\s+/, '')
    docker-machine create -d amazonec2 \
      --amazonec2-vpc-id vpc-2229ff47 \
      --amazonec2-region us-west-2 \
      --amazonec2-instance-type m3.medium \
      --amazonec2-private-address-only \
      bldr-docker-machine
  EOH
  sensitive true
  environment('AWS_ACCESS_KEY_ID'     => creds['aws_access_key_id'],
              'AWS_SECRET_ACCESS_KEY' => creds['aws_secret_access_key'])
  not_if { BldrDockerMachine.available? }
end

chef_gem 'aws-sdk-v1'

ruby_block 'save-docker-machine-state' do
  block do
    # We need to save the secrets created by
    # 'execute[bldr-docker-machine]' above somewhere that all builder
    # nodes in Delivery can access.
    machine_home   = BldrDockerMachine.machine_home
    config         = BldrDockerMachine.load_config
    ca_pem         = IO.read(File.join(machine_home, 'ca.pem'))
    server_pem     = IO.read(File.join(machine_home, 'server.pem'))
    server_key_pem = IO.read(File.join(machine_home, 'server-key.pem'))
    cert_pem       = IO.read(File.join(machine_home, 'cert.pem'))
    key_pem        = IO.read(File.join(machine_home, 'key.pem'))
    id_rsa_public  = IO.read(File.join(machine_home, 'id_rsa.pub'))
    id_rsa_private = IO.read(File.join(machine_home, 'id_rsa'))

    dm_bag = Chef::DataBagItem.new
    dm_bag.data_bag('bldr-acceptance')
    dm_bag.raw_data = {
      'id'             => 'bldr-docker-machine',
      'ca_pem'         => ca_pem,
      'server_pem'     => server_pem,
      'server_key_pem' => server_key_pem,
      'cert_pem'       => cert_pem,
      'key_pem'        => key_pem,
      'id_rsa_public'  => id_rsa_public,
      'id_rsa_private' => id_rsa_private,
      'config'         => {
        'InstanceId'   => config['Driver']['InstanceId'],
        'IPAddress'    => config['Driver']['IPAddress'],
        'MachineName'  => config['Driver']['MachineName'],
        'SSHUser'      => config['Driver']['SSHUser'],
        'DriverName'   => config['DriverName'],
        'ConfigVersion' => config['ConfigVersion']
      }
    }

    dm_bag.save
  end

  action :nothing
  subscribes :create, 'execute[bldr-docker-machine]'
end

ruby_block 'tag-docker-machine' do
  block do
    config = BldrDockerMachine.load_config
    instance_id = config['Driver']['InstanceId']

    require 'aws-sdk-v1'

    ec2 = AWS::EC2.new(region: config['Driver']['Region'])
    instance = ec2.instances[instance_id]
    instance.tags['X-Project'] = 'bldr'
  end

  action :nothing
  subscribes :create, 'execute[bldr-docker-machine]'
end
