The **oc-id** service enables OAuth 2.0 authentication to the Chef Infra
Server by external applications, including Chef Supermarket. OAuth 2.0
uses token-based authentication, where external applications use tokens
that are issued by the **oc-id** provider. No special
credentials---`webui_priv.pem` or privileged keys---are stored on the
external application.