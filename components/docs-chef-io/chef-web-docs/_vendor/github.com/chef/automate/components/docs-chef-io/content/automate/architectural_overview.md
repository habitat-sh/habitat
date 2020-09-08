+++
title = "Architecture"

draft = false
[menu]
  [menu.automate]
    title = "Architecture"
    parent = "automate/reference"
    identifier = "automate/reference/architecture.md Architecture"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/architectural_overview.md)

## Automate 2 Architecture

![Automate 2 Architecture](/images/automate/a2-architecture.png)

## Component overview

### Automate Gateway

The Automate Gateway serves as the application layer of Chef Automate's architecture. All public facing requests go through the gateway and authentication/authorization takes place here.

### Deployment Service

This service collects the initial service configuration from the user. It does everything required to set up Chef Automate initially. The deployment service manages configuration patches, as well.

### Configuration Management Service

This service serves all configuration management related information to the API and user interface, including Chef Infra Server action data and Chef Infra Client run data.

### Ingest Service

This service is the primary ingress event handler for configuration management related events such as Chef Infra Client runs and Chef Infra Server actions. It also manages the data related to these domains, such as cleanup, migration and index initialization.

### Compliance Service

This service handles InSpec and scan job-related data, including event ingestion and reporting.

### Notification Service

This service is responsible for sending notifications based on configured rules in response to events.

### License Control Service

This service provides policy information to the rest of the system derived from the license file. It also provides telemetry configuration.

### AuthZ Service

This service provides the API to determine which actions a requestor is allowed to take on in Chef Automate.

### AuthN Service

This service provides the API to verify a requestor is allowed to interact with Chef Automate.

### Teams Service

This service is an API for defining local teams that are used as part of the authorization model for Chef Automate.

### Users Service

This service is used to manage users local to Chef Automate (as opposed to users defined in an external identity provider).

### Session Service

This service stands between the browser and Dex. It acts as an [OpenID Connect](http://openid.net/connect/) client to Dex, and uses the [Authorization Code Grant Flow](https://auth0.com/docs/api-auth/tutorials/authorization-code-grant).

### Secrets Service

Service securely stores credentials for other services.

### Elasticsearch Sidecar Service

This service runs alongside Elasticsearch. It provides common Elasticsearch functionality such as monitoring disk usage and handling index purges.

### Dex

[Dex](https://github.com/dexidp/dex) is a federated OpenID Connect (OIDC) provider that allows Automate to integrate with external identity providers via LDAP, SAML or OpenID Connect.
