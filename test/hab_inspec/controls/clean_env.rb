hab_env_vars = %w(HAB_AUTH_TOKEN
                  HAB_CACHE_KEY_PATH
                  HAB_DEPOT_URL
                  HAB_ORG
                  HAB_ORIGIN
                  HAB_ORIGIN_KEYS
                  HAB_RING
                  HAB_RING_KEY
                  HAB_STUDIOS_HOME
                  HAB_STUDIO_ROOT
                  HAB_USER
                  no_proxy)


hab_ports = [9631, 9632, 9636]

hab_ports.each do |p|
    describe port(p) do
        it { should_not be_listening }
    end
end

hab_env_vars.each do |e|
    describe os_env(e) do
        its('content') { should eq nil }
    end
end

describe file("results") do
    it { should_not exist }
end

describe user('hab') do
    it { should exist }
end




