Search indexes allow queries to be made for any type of data that is
indexed by the Chef Infra Server, including data bags (and data bag
items), environments, nodes, and roles. A defined query syntax is used
to support search patterns like exact, wildcard, range, and fuzzy. A
search is a full-text query that can be done from several locations,
including from within a recipe, by using the `search` subcommand in
knife, the `search` method in the Recipe DSL, the search box in the Chef
management console, and by using the `/search` or `/search/INDEX`
endpoints in the Chef Infra Server API. The search engine is based on
Apache Solr and is run from the Chef Infra Server.