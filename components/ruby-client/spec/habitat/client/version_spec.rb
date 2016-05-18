require 'spec_helper'

describe 'Habitat::Client::VERSION' do
  it 'has a version number' do
    expect(Habitat::Client::VERSION).not_to be nil
  end
end
