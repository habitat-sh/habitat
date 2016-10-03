
hab_ports = [9631, 9632, 9636]

hab_ports.each do |p|
    describe port(p) do
        it { should_not be_listening }
    end
end

describe file("results") do
    it { should_not exist }
end

describe file("/hab/svc/simple_service") do
    it { should_not exist }
end

describe user('hab') do
    it { should exist }
end




