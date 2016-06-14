# Habitat::Client

This is the Ruby client library for Habitat.

## Installation

Add this line to your application's Gemfile:

```ruby
gem 'habitat-client'
```

And then execute:

    $ bundle

Or install it yourself as:

    $ gem install habitat-client

## Usage

```ruby
# Create an instance object
hc = Habitat::Client.new

# Create the instance with a specific Habitat Depot
hc = Habitat::Client.new('http://localhost:9636/v1/depot')

# Upload a Habitat Artifact
hc.put_package('core-pandas-0.0.1-20160425190407.hart')

# Show a package
hc.show_package('core/pandas')

# Promote a package to the `stable` view
hc.promote_package('core/pandas', 'stable')
```
