All communication with the Chef Infra Server must be authenticated using
the Chef Infra Server API, which is a REST API that allows requests to
be made to the Chef Infra Server. Only authenticated requests will be
authorized. Most of the time, and especially when using knife, Chef
Infra Client, or the Chef Infra Server web interface, the use of the
Chef Infra Server API is transparent. In some cases, the use of the Chef
Infra Server API requires more detail, such as when making the request
in Ruby code, with a knife plugin, or when using cURL.