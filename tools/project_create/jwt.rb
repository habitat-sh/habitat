#
# Usage: jwt <app id> <path to PEM file>
#
# Note: Do a 'gem install jwt' before using this
#

require 'openssl'
require 'jwt'  # https://rubygems.org/gems/jwt

if ARGV.length < 2
  puts "Usage: jwt <app id> <path to PEM file>"
  exit
end

app_id = ARGV[0]
pem_path = ARGV[1]

# Private key contents
private_pem = File.read(pem_path)
private_key = OpenSSL::PKey::RSA.new(private_pem)

# Generate the JWT
payload = {
  # issued at time
  iat: Time.now.to_i,
  # JWT expiration time (10 minute maximum)
  exp: Time.now.to_i + (10 * 60),
  # GitHub App's identifier
  iss: app_id
}

jwt = JWT.encode(payload, private_key, "RS256")
puts jwt
