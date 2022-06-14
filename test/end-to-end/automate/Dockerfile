FROM ubuntu

# We need curl
RUN apt-get update && apt-get -y install curl

# Download latest release of automate
RUN curl https://packages.chef.io/files/current/latest/chef-automate-cli/chef-automate_linux_amd64.zip | gunzip - > chef-automate && chmod +x chef-automate

# Configure automate. Make sure its fqdn is set to localhost and not the build containers
# IP or hostname which will only work in the build environment
RUN ./chef-automate init-config --fqdn localhost

# Docker environment may not have a systemd so hint to automate not to use it
ENV CHEF_AUTOMATE_SKIP_SYSTEMD="true"

ENV HAB_LICENSE=accept-no-persist

# Get all the automate services deployed and running
RUN mkdir -p /hab/sup/default
RUN ./chef-automate deploy config.toml --skip-preflight --accept-terms-and-mlsa

# This is how containers will start automate
CMD hab sup run
