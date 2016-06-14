# coding: utf-8
lib = File.expand_path('../lib', __FILE__)
$LOAD_PATH.unshift(lib) unless $LOAD_PATH.include?(lib)
require 'habitat/client/version'

Gem::Specification.new do |spec|
  spec.name          = 'habitat-client'
  spec.version       = Habitat::Client::VERSION
  spec.authors       = ['Joshua Timberman']
  spec.email         = ['humans@habitat.sh']

  spec.summary       = 'Habitat Depot Client Library'
  spec.homepage      = 'https://www.habitat.sh'

  spec.files         = `git ls-files -z`.split("\x0").reject { |f| f.match(%r{^(test|spec|features)/}) }
  spec.executables   = spec.files.grep(%r{^exe/}) { |f| File.basename(f) }
  spec.require_paths = ['lib']

  spec.add_dependency 'rbnacl', '~> 3.3'
  spec.add_dependency 'faraday', '~> 0.9.0'
  spec.add_dependency 'mixlib-shellout', '~> 2.2'

  spec.add_development_dependency 'pry'
  spec.add_development_dependency 'pry-coolline'
  spec.add_development_dependency 'bundler', '~> 1.11'
  spec.add_development_dependency 'rake', '~> 10.0'
  spec.add_development_dependency 'rspec', '~> 3.0'
end
