+++
title = "Example bldr.env"
description = "An example Chef Habitat Builder configuration file."

[menu]
  [menu.habitat]
    title = "Example bldr.env"
    identifier = "habitat/builder-on-prem/builder-example"
    parent = "habitat"

+++

An example Chef Habitat Builder configuration file.

```shell
#!/bin/bash

# The endpoint and port for your Postgresql instance
# Change only if needed
export POSTGRES_HOST=localhost
export POSTGRES_PORT=5432

# The endpoint, key and secret for your Minio instance (see README)
# Change these before the first install if needed
export MINIO_ENDPOINT=http://localhost:9000
export MINIO_BUCKET=habitat-builder-artifact-store.local
export MINIO_ACCESS_KEY=depot
export MINIO_SECRET_KEY=password

# If you'd like to use Artifactory instead of Minio, uncomment
# and set the following variables appropriately.
# IMPORTANT: See the README for more info
# export ARTIFACTORY_ENABLED=true
# export ARTIFACTORY_API_URL=http://localhost:8081
# export ARTIFACTORY_API_KEY=foo
# export ARTIFACTORY_REPO=habitat-builder-artifact-store

# Modify these as needed for the on-premise OAuth2 provider.
# The variables below are configured for GitHub by default,
# but appropriate values for Bitbucket, GitLab, Azure AD and Okta
# are also included as comments.

# Whether SSL is enabled for the on-prem depot
export APP_SSL_ENABLED=false

# The URL for this instance of the on-prem depot
# IMPORTANT: If SSL is enabled, APP_URL should start be https
export APP_URL=http://localhost

# The OAUTH_PROVIDER value can be "github", "gitlab", "bitbucket", "azure-ad",
# "okta" or "chef-automate"
export OAUTH_PROVIDER=github
# export OAUTH_PROVIDER=bitbucket
# export OAUTH_PROVIDER=gitlab
# export OAUTH_PROVIDER=azure-ad
# export OAUTH_PROVIDER=okta
# export OAUTH_PROVIDER=chef-automate

# The OAUTH_USERINFO_URL is the API endpoint that will be used for user info
export OAUTH_USERINFO_URL=https://api.github.com/user
# export OAUTH_USERINFO_URL=https://api.bitbucket.org/1.0/user
# export OAUTH_USERINFO_URL=https://gitlab.com/oauth/userinfo
# export OAUTH_USERINFO_URL=https://login.microsoftonline.com/<tenant-id>/openid/userinfo
# export OAUTH_USERINFO_URL=https://<your.okta.domain>.com/oauth2/v1/userinfo
# export OAUTH_USERINFO_URL=https://<your.automate.domain>/session/userinfo

# The OAUTH_AUTHORIZE_URL is the *fully qualified* OAuth2 authorization endpoint
export OAUTH_AUTHORIZE_URL=https://github.com/login/oauth/authorize
# export OAUTH_AUTHORIZE_URL=https://bitbucket.org/site/oauth2/authorize
# export OAUTH_AUTHORIZE_URL=https://gitlab.com/oauth/authorize
# export OAUTH_AUTHORIZE_URL=https://login.microsoftonline.com/<tenant-id>/oauth2/authorize
# export OAUTH_AUTHORIZE_URL=https://<your.okta.domain>.com/oauth2/v1/authorize
# export OAUTH_AUTHORIZE_URL=https://<your.automate.domain>/session/new

# The OAUTH_SIGNUP_URL is the link used to register users with the OAUTH provider
export OAUTH_SIGNUP_URL=https://github.com/join
# export OAUTH_SIGNUP_URL=https://bitbucket.org/account/signup/
# export OAUTH_SIGNUP_URL=https://gitlab.com/users/sign_in#register-pane

# The OAUTH_TOKEN_URL is the *fully qualified* OAuth2 token endpoint
export OAUTH_TOKEN_URL=https://github.com/login/oauth/access_token
# export OAUTH_TOKEN_URL=https://bitbucket.org/site/oauth2/access_token
# export OAUTH_TOKEN_URL=https://gitlab.com/oauth/token
# export OAUTH_TOKEN_URL=https://login.microsoftonline.com/tenant-id/oauth2/token
# export OAUTH_TOKEN_URL=https://your.okta.domain.com/oauth2/v1/token
# export OAUTH_TOKEN_URL=https://<your.automate.domain>/session/token

# The OAUTH_REDIRECT_URL is the registered OAuth2 redirect
# IMPORTANT: If SSL is enabled, the redirect URL should be https
export OAUTH_REDIRECT_URL=http://localhost/

# The OAUTH_CLIENT_ID is the registered OAuth2 client id
export OAUTH_CLIENT_ID=0123456789abcdef0123

# The OAUTH_CLIENT_SECRET is the registered OAuth2 client secret
export OAUTH_CLIENT_SECRET=0123456789abcdef0123456789abcdef01234567

# Modify these only if there is a specific need, otherwise leave as is
export BLDR_CHANNEL=on-prem-stable
export BLDR_ORIGIN=habitat
export HAB_BLDR_URL=https://bldr.habitat.sh

# Help us make Habitat better! Opt into analytics by changing the ANALYTICS_ENABLED
# setting below to true, then optionally provide your company name. (Analytics is
# disabled by default. See our privacy policy at https://www.habitat.sh/legal/privacy-policy/.)
export ANALYTICS_ENABLED=false
export ANALYTICS_COMPANY_NAME=""
```
