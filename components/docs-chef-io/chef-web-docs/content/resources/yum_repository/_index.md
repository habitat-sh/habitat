---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: yum_repository resource
resource: yum_repository
aliases:
- "/resource_yum_repository.html"
menu:
  infra:
    title: yum_repository
    identifier: chef_infra/cookbook_reference/resources/yum_repository yum_repository
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **yum_repository** resource to manage a Yum repository configuration
    file located at `/etc/yum.repos.d/repositoryid.repo` on the local machine. This
    configuration file specifies which repositories to reference, how to handle cached
    data, etc.
resource_new_in: '12.14'
syntax_full_code_block: |-
  yum_repository 'name' do
    baseurl                    String, Array
    clean_metadata             true, false # default value: true
    cost                       String
    description                String # default value: "Yum Repository"
    enabled                    true, false # default value: true
    enablegroups               true, false
    exclude                    String
    failovermethod             String
    fastestmirror_enabled      true, false
    gpgcheck                   true, false # default value: true
    gpgkey                     String, Array
    http_caching               String
    include_config             String
    includepkgs                String
    keepalive                  true, false
    make_cache                 true, false # default value: true
    max_retries                String, Integer
    metadata_expire            String
    metalink                   String
    mirror_expire              String
    mirrorlist                 String
    mirrorlist_expire          String
    mode                       String, Integer # default value: "0644"
    options                    Hash
    password                   String
    priority                   String
    proxy                      String
    proxy_password             String
    proxy_username             String
    repo_gpgcheck              true, false
    report_instanceid          true, false
    repositoryid               String # default value: 'name' unless specified
    skip_if_unavailable        true, false
    source                     String
    sslcacert                  String
    sslclientcert              String
    sslclientkey               String
    sslverify                  true, false
    throttle                   String, Integer
    timeout                    String
    username                   String
    action                     Symbol # defaults to :create if not specified
  end
syntax_properties_list:
syntax_full_properties_list:
- "`yum_repository` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`baseurl`, `clean_metadata`, `cost`, `description`, `enabled`, `enablegroups`,
  `exclude`, `failovermethod`, `fastestmirror_enabled`, `gpgcheck`, `gpgkey`, `http_caching`,
  `include_config`, `includepkgs`, `keepalive`, `make_cache`, `max_retries`, `metadata_expire`,
  `metalink`, `mirror_expire`, `mirrorlist`, `mirrorlist_expire`, `mode`, `options`,
  `password`, `priority`, `proxy`, `proxy_password`, `proxy_username`, `repo_gpgcheck`,
  `report_instanceid`, `repositoryid`, `skip_if_unavailable`, `source`, `sslcacert`,
  `sslclientcert`, `sslclientkey`, `sslverify`, `throttle`, `timeout`, and `username`
  are the properties available to this resource."
actions_list:
  :create:
    markdown: Create a repository based on the properties.
  :makecache:
    markdown: Force the creation of the repository cache. This is also done automatically when a repository is updated.
  :nothing:
    shortcode: resources_common_actions_nothing.md
  :remove:
    markdown: Remove a repository. 
properties_list:
- property: baseurl
  ruby_type: String, Array
  required: false
  description_list:
  - markdown: URL to the directory where the Yum repository's `repodata` directory
      lives. Can be an `http://`, `https://` or a `ftp://` URLs. You can specify multiple
      URLs in one `baseurl` statement.
- property: clean_metadata
  ruby_type: true, false
  required: false
  default_value: 'true'
  description_list:
  - markdown: Specifies whether you want to purge all of the packages downloaded from
      a Yum repository and held in a cache directory.
- property: cost
  ruby_type: String
  required: false
  description_list:
  - markdown: Relative cost of accessing this repository. Useful for weighing one
      repo's packages as greater/less than any other.
- property: description
  ruby_type: String
  required: false
  default_value: Yum Repository
  description_list:
  - markdown: Descriptive name for the repository channel and maps to the 'name' parameter
      in a repository .conf.
- property: enabled
  ruby_type: true, false
  required: false
  default_value: 'true'
  description_list:
  - markdown: Specifies whether or not Yum should use this repository.
- property: enablegroups
  ruby_type: true, false
  required: false
  description_list:
  - markdown: Specifies whether Yum will allow the use of package groups for this
      repository.
- property: exclude
  ruby_type: String
  required: false
  description_list:
  - markdown: List of packages to exclude from updates or installs. This should be
      a space separated list. Shell globs using wildcards (eg. * and ?) are allowed.
- property: failovermethod
  ruby_type: String
  required: false
  allowed_values: '"priority", "roundrobin"'
  description_list:
  - markdown: Method to determine how to switch to a new server if the current one
      fails, which can either be `roundrobin` or `priority`. `roundrobin` randomly
      selects a URL out of the list of URLs to start with and proceeds through each
      of them as it encounters a failure contacting the host. `priority` starts from
      the first `baseurl` listed and reads through them sequentially.
- property: fastestmirror_enabled
  ruby_type: true, false
  required: false
  description_list:
  - markdown: Specifies whether to use the fastest mirror from a repository configuration
      when more than one mirror is listed in that configuration.
- property: gpgcheck
  ruby_type: true, false
  required: false
  default_value: 'true'
  description_list:
  - markdown: Specifies whether or not Yum should perform a GPG signature check on
      the packages received from a repository.
- property: gpgkey
  ruby_type: String, Array
  required: false
  description_list:
  - markdown: |-
      URL pointing to the ASCII-armored GPG key file for the repository. This is used if Yum needs a public key to verify a package and the required key hasn't been imported into the RPM database. If this option is set, Yum will automatically import the key from the specified URL. Multiple URLs may be specified in the same manner as the baseurl option. If a GPG key is required to install a package from a repository, all keys specified for that repository will be installed.
      Multiple URLs may be specified in the same manner as the baseurl option. If a GPG key is required to install a package from a repository, all keys specified for that repository will be installed.
- property: http_caching
  ruby_type: String
  required: false
  allowed_values: '"all", "none", "packages"'
  description_list:
  - markdown: |-
      Determines how upstream HTTP caches are instructed to handle any HTTP downloads that Yum does. This option can take the following values:
       - `all` means all HTTP downloads should be cached
       - `packages` means only RPM package downloads should be cached, but not repository metadata downloads
       - `none` means no HTTP downloads should be cached.

      The default value of `all` is recommended unless you are experiencing caching related issues.
- property: include_config
  ruby_type: String
  required: false
  description_list:
  - markdown: An external configuration file using the format `url://to/some/location`.
- property: includepkgs
  ruby_type: String
  required: false
  description_list:
  - markdown: Inverse of exclude property. This is a list of packages you want to
      use from a repository. If this option lists only one package then that is all
      Yum will ever see from the repository.
- property: keepalive
  ruby_type: true, false
  required: false
  description_list:
  - markdown: Determines whether or not HTTP/1.1 `keep-alive` should be used with
      this repository.
- property: make_cache
  ruby_type: true, false
  required: false
  default_value: 'true'
  description_list:
  - markdown: Determines whether package files downloaded by Yum stay in cache directories.
      By using cached data, you can carry out certain operations without a network
      connection.
- property: max_retries
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: Number of times any attempt to retrieve a file should retry before returning
      an error. Setting this to `0` makes Yum try forever.
- property: metadata_expire
  ruby_type: String
  required: false
  description_list:
  - markdown: 'Time (in seconds) after which the metadata will expire. If the current
      metadata downloaded is less than the value specified, then Yum will not update
      the metadata against the repository. If you find that Yum is not downloading
      information on updates as often as you would like lower the value of this option.
      You can also change from the default of using seconds to using days, hours or
      minutes by appending a `d`, `h` or `m` respectively. The default is six hours
      to compliment yum-updates running once per hour. It is also possible to use
      the word `never`, meaning that the metadata will never expire. Note: When using
      a metalink file, the metalink must always be newer than the metadata for the
      repository due to the validation, so this timeout also applies to the metalink
      file.'
  - note:
    - markdown: 'When using a metalink file, the metalink must always be newer than

        the metadata for the repository due to the validation, so this

        timeout also applies to the metalink file.'
- property: metalink
  ruby_type: String
  required: false
  description_list:
  - markdown: Specifies a URL to a metalink file for the repomd.xml, a list of mirrors
      for the entire repository are generated by converting the mirrors for the repomd.xml
      file to a baseurl.
- property: mirror_expire
  ruby_type: String
  required: false
  description_list:
  - markdown: Time (in seconds) after which the mirrorlist locally cached will expire.
      If the current mirrorlist is less than this many seconds old then Yum will not
      download another copy of the mirrorlist, it has the same extra format as metadata_expire.
      If you find that Yum is not downloading the mirrorlists as often as you would
      like lower the value of this option. You can also change from the default of
      using seconds to using days, hours or minutes by appending a `d`, `h` or `m`
      respectively.
- property: mirrorlist
  ruby_type: String
  required: false
  description_list:
  - markdown: URL to a file containing a list of baseurls. This can be used instead
      of or with the baseurl option. Substitution variables, described below, can
      be used with this option.
- property: mirrorlist_expire
  ruby_type: String
  required: false
  description_list:
  - markdown: Specifies the time (in seconds) after which the mirrorlist locally cached
      will expire. If the current mirrorlist is less than the value specified, then
      Yum will not download another copy of the mirrorlist. You can also change from
      the default of using seconds to using days, hours or minutes by appending a
      `d`, `h` or `m` respectively.
- property: mode
  ruby_type: String, Integer
  required: false
  default_value: '"0644"'
  description_list:
  - markdown: Permissions mode of .repo file on disk. This is useful for scenarios
      where secrets are in the repo file. If this value is set to `600`, normal users
      will not be able to use Yum search, Yum info, etc.
- property: options
  ruby_type: Hash
  required: false
  description_list:
  - markdown: Specifies the repository options.
- property: password
  ruby_type: String
  required: false
  description_list:
  - markdown: Password to use with the username for basic authentication.
- property: priority
  ruby_type: String
  required: false
  description_list:
  - markdown: Assigns a priority to a repository where the priority value is between
      `1` and `99` inclusive. Priorities are used to enforce ordered protection of
      repositories. Packages from repositories with a lower priority (higher numerical
      value) will never be used to upgrade packages that were installed from a repository
      with a higher priority (lower numerical value). The repositories with the lowest
      numerical priority number have the highest priority.
- property: proxy
  ruby_type: String
  required: false
  description_list:
  - markdown: URL to the proxy server that Yum should use.
- property: proxy_password
  ruby_type: String
  required: false
  description_list:
  - markdown: Password for this proxy.
- property: proxy_username
  ruby_type: String
  required: false
  description_list:
  - markdown: Username to use for proxy.
- property: repo_gpgcheck
  ruby_type: true, false
  required: false
  description_list:
  - markdown: Determines whether or not Yum should perform a GPG signature check on
      the repodata from this repository.
- property: report_instanceid
  ruby_type: true, false
  required: false
  description_list:
  - markdown: Determines whether to report the instance ID when using Amazon Linux
      AMIs and repositories.
- property: repositoryid
  ruby_type: String
  required: false
  default_value: The resource block's name
  description_list:
  - markdown: An optional property to set the repository name if it differs from the
      resource block's name.
- property: skip_if_unavailable
  ruby_type: true, false
  required: false
  description_list:
  - markdown: Allow yum to continue if this repository cannot be contacted for any
      reason.
- property: source
  ruby_type: String
  required: false
  description_list:
  - markdown: Use a custom template source instead of the default one.
- property: sslcacert
  ruby_type: String
  required: false
  description_list:
  - markdown: Path to the directory containing the databases of the certificate authorities
      Yum should use to verify SSL certificates.
- property: sslclientcert
  ruby_type: String
  required: false
  description_list:
  - markdown: Path to the SSL client certificate Yum should use to connect to repos/remote
      sites.
- property: sslclientkey
  ruby_type: String
  required: false
  description_list:
  - markdown: Path to the SSL client key Yum should use to connect to repos/remote
      sites.
- property: sslverify
  ruby_type: true, false
  required: false
  description_list:
  - markdown: Determines whether Yum will verify SSL certificates/hosts.
- property: throttle
  ruby_type: String, Integer
  required: false
  description_list:
  - markdown: Enable bandwidth throttling for downloads.
- property: timeout
  ruby_type: String
  required: false
  description_list:
  - markdown: Number of seconds to wait for a connection before timing out. Defaults
      to 30 seconds. This may be too short of a time for extremely overloaded sites.
- property: username
  ruby_type: String
  required: false
  description_list:
  - markdown: Username to use for basic authentication to a repository.
examples: |
  **Add an internal company repository**:

  ```ruby
  yum_repository 'OurCo' do
    description 'OurCo yum repository'
    mirrorlist 'http://artifacts.ourco.org/mirrorlist?repo=ourco-8&arch=$basearch'
    gpgkey 'http://artifacts.ourco.org/pub/yum/RPM-GPG-KEY-OURCO-8'
    action :create
  end
  ```ruby

  **Delete a repository**:

  ```ruby
  yum_repository 'CentOS-Media' do
    action :delete
  end
  ```
---