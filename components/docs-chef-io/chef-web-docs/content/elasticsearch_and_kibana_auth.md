+++
title = "Authentication for Elasticsearch and Kibana"
draft = false
robots = "noindex"


aliases = ["/elasticsearch_and_kibana_auth.html"]

[menu]
  [menu.legacy]
    title = "Elasticsearch and Kibana Auth"
    identifier = "legacy/workflow/managing_workflow/elasticsearch_and_kibana_auth.md Elasticsearch and Kibana Auth"
    parent = "legacy/workflow/managing_workflow"
    weight = 140
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/elasticsearch_and_kibana_auth.md)

{{% chef_automate_mark %}}

{{% EOL_a1 %}}

Node data in Chef Automate is stored in
[Elasticsearch](https://www.elastic.co/products/elasticsearch) and
viewable in the Chef Automate UI as well as
[Kibana](https://www.elastic.co/products/kibana). Access to Chef
Automate's Elasticsearch and Kibana is protected by the same
authentication used by the Chef Automate user interface. Elasticsearch
authentication is enabled by default.

{{% kibana_note %}}

## How It Works

-   User logs into the Chef Automate UI normally.
-   Chef Automate stores information about the user's session in browser
    local storage as well as a browser cookie.
-   If authentication is enabled for Elasticsearch or Kibana, Chef Automate's
    web server will look for the session cookie and validate the session is valid and active.
-   If the session is valid and active, the request is permitted.
-   If the session is invalid, or if no session information is present, the
    server returns a `401 Unauthorized` message.

## Accessing Elasticsearch with Authentication - Node Visibility UI

The Automate node visibility UI performs a number of queries to
Elasticsearch in order to present the node visibility data. The Chef
Automate server will validate each of the Elasticsearch requests with
the session cookie information as described in the **How It Works**
section above.

## Accessing Elasticsearch with Authentication - API/CLI

If you wish to access Elasticsearch via your Chef Automate server via a
CLI tool (such as `curl`) or an API client (such as
[elasticsearch-ruby](https://github.com/elastic/elasticsearch-ruby)),
you must pass three additional HTTP headers in your requests for your
request to be properly authenticated:

-   `chef-delivery-user`: the Chef Automate username for whom a token
    has been generated
-   `chef-delivery-token`: a valid token generated for the user
-   `chef-delivery-enterprise`: the Chef Automate enterprise name. This is the
    string after the `/e/` in your Chef Automate URLs.

Example: if your Workflow dashboard URL is `https://my-automate-server.mycompany.biz/e/coolcompany/#/dashboard`
and your enterprise is `coolcompany`.

To generate a token, use the `delivery token` command of the [Delivery
CLI](/delivery_cli/).

For example, to pass the required headers using curl:

``` bash
curl https://my-automate-server.mycompany.biz/elasticsearch/_cat/indices -H "chef-delivery-user: myuser" -H "chef-delivery-enterprise: coolcompany" -H "chef-delivery-token: s00pers33krett0ken"
```

## Accessing Kibana with Authentication

Your browser must have a valid cookie containing a valid token before
access to Kibana will be permitted. If you encounter a "401
Unauthorized" error message, follow these steps:

-   Log into the Chef Automate UI normally.
-   Change your browser URI to `/kibana`.

Example: `https://my-automate-server.mycompany.biz/kibana`

## Configuration

{{< warning >}}

It is strongly recommended that authentication to Elasticsearch and
Kibana remain enabled at all times. Without authentication, any user
with network access to your Automate server will be able to view any
available Visibility data.

{{< /warning >}}

If you wish to disable authentication for either Kibana or
Elasticsearch, you may use the following configuration parameters in
your `/etc/delivery/delivery.rb` configuration file:

-   `elasticsearch['enable_auth']`: If `true`, a valid
    user/enterprise/token must be supplied in a cookie or in HTTP
    headers for the request to be accepted and passed to Elasticsearch.
    If `false`, all Elasticsearch queries are permitted without
    authentication. Default: `true`
-   `kibana['enable_auth']`: If `true`, a valid user/enterprise/token
    must be supplied in a cookie or in HTTP headers for access to be
    granted to the Kibana UI. If `false`, all Kibana access is permitted
    without authentication. Default: `true`
