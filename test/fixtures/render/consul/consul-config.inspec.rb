describe json('result/consul_config.json') do
  its('datacenter') { should eq 'IN_OVERRIDE_JSON' }
  its('data_dir') { should eq 'IN_DEFAULT_TOML' }
  its('log_level') { should eq 'IN_USER_TOML' }
  its('bind_addr') { should eq '9.9.9.9' }
  its('server') { should eq true }

  its(['retry_join', 0]) { should eq '1.1.1.1' }
  its(['retry_join', 1]) { should eq '2.2.2.2' }
  its(['retry_join', 2]) { should eq '3.3.3.3' }
  its(['ports','dns']) { should eq 6666 }
  its(['ports','server']) { should eq 9999 }
end
