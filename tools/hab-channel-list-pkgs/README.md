## List All Packages in a Channel

This is a a tool which will fetch all packages that are in an origin's channel across multiple paged API responses.

### Usage

The following command line tools are required and will be checked before running:

* `curl`
* `jq`

To run:

```sh
hab-channel-list-pkgs.sh core unstable
```

A list of all fully qualified package identifiers that exist in the channel will be returned, printed to standard out, one entry per line.

This program also honors the `HAB_BLDR_URL` environment variable and will default to the public Builder.
