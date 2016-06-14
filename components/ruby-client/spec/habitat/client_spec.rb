require 'spec_helper'

describe Habitat::Client do
  let(:hc) { Habitat::Client.new }
  let(:hc_custom_depot) { Habitat::Client.new('http://localhost:9636') }

  it 'includes the Habitat module in Hab' do
    expect(hc).to be_a(Hab::Client)
  end

  it 'has a default depot' do
    expect(hc.depot).to eq 'https://willem.habitat.sh/v1/depot'
  end

  it 'can set custom depot' do
    expect(hc_custom_depot.depot).to eq 'http://localhost:9636'
  end

  it 'sets a connection up with Faraday' do
    expect(hc.connection).to be_a(Faraday::Connection)
  end

  it 'raises when using unimplemented methods' do
    expect { hc.fetch_key('empty') }.to raise_exception RuntimeError
    expect { hc.put_key('empty', 'file.key') }.to raise_exception RuntimeError
  end
end

describe Habitat::PackageIdent do
  it 'sets version to latest if unspecified' do
    hpi = Habitat::PackageIdent.new('core', 'rspec')
    expect(hpi.version).to eq 'latest'
  end

  it 'sets release to latest if unspecified' do
    hpi = Habitat::PackageIdent.new('core', 'rspec', '3.4.4')
    expect(hpi.release).to eq 'latest'
  end

  describe '#to_s' do
    it 'ends in latest with no version specified' do
      hpi = Habitat::PackageIdent.new('core', 'rspec')
      expect(hpi.to_s).to match('core/rspec/latest')
    end

    it 'ends in latest with version, but no release specified' do
      hpi = Habitat::PackageIdent.new('core', 'rspec', '3.4.4')
      expect(hpi.to_s).to match('core/rspec/3.4.4/latest')
    end

    it 'is fully qualified with all parts specified' do
      hpi = Habitat::PackageIdent.new('core', 'rspec', '3.4.4', '201604181646')
      expect(hpi.to_s).to match('core/rspec/3.4.4/201604181646')
    end
  end
end
