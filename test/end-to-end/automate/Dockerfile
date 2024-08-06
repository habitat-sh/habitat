FROM ubuntu:latest

# Update package lists and upgrade existing packages
RUN echo "--- Boostsrap env for automate install" && \
    apt-get update && \
    apt-get install -y \
        curl

# Download and install Chef Automate
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

# Deploy license
ARG A2_LICENSE
RUN ./chef-automate license apply $A2_LICENSE

# This is how containers will start automate
CMD hab sup run