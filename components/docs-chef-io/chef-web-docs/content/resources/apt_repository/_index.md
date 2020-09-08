---
resource_reference: true
properties_shortcode: 
resources_common_guards: true
resources_common_notification: true
resources_common_properties: true
title: apt_repository resource
resource: apt_repository
aliases:
- "/resource_apt_repository.html"
menu:
  infra:
    title: apt_repository
    identifier: chef_infra/cookbook_reference/resources/apt_repository apt_repository
    parent: chef_infra/cookbook_reference/resources
resource_description_list:
- markdown: Use the **apt_repository** resource to specify additional APT repositories.
    Adding a new repository will update the APT package cache immediately.
resource_new_in: '12.9'
syntax_description: "An **apt_repository** resource specifies APT repository information\
  \ and\nadds an additional APT repository to the existing list of repositories:\n\
  \n``` ruby\napt_repository 'nginx' do\n  uri        'http://nginx.org/packages/ubuntu/'\n\
  \  components ['nginx']\nend\n```"
syntax_full_code_block: |-
  apt_repository 'name' do
    arch               String, false
    cache_rebuild      true, false # default value: true
    components         Array # default value: `main` if using a PPA repository.
    cookbook           String, false
    deb_src            true, false # default value: false
    distribution       String, false # default value: The LSB codename of the node such as 'focal'.
    key                String, Array, false
    key_proxy          String, false
    keyserver          String, false # default value: "keyserver.ubuntu.com"
    repo_name          String # default value: 'name' unless specified
    trusted            true, false # default value: false
    uri                String
    action             Symbol # defaults to :add if not specified
  end
syntax_properties_list: 
syntax_full_properties_list:
- "`apt_repository` is the resource."
- "`name` is the name given to the resource block."
- "`action` identifies which steps Chef Infra Client will take to bring the node into
  the desired state."
- "`arch`, `cache_rebuild`, `components`, `cookbook`, `deb_src`, `distribution`, `key`,
  `key_proxy`, `keyserver`, `repo_name`, `trusted`, and `uri` are the properties available
  to this resource."
actions_list:
  :add:
    markdown: Default. Creates a repository file at `/etc/apt/sources.list.d/` and
      builds the repository listing.
  :remove:
    markdown: Removes the repository listing.
  :nothing:
    shortcode: resources_common_actions_nothing.md
properties_list:
- property: arch
  ruby_type: String, false
  required: false
  description_list:
  - markdown: Constrain packages to a particular CPU architecture such as `i386` or
      `amd64`.
- property: cache_rebuild
  ruby_type: true, false
  required: false
  default_value: 'true'
  description_list:
  - markdown: Determines whether to rebuild the APT package cache.
- property: components
  ruby_type: Array
  required: false
  default_value: "`main` if using a PPA repository."
  description_list:
  - markdown: Package groupings, such as 'main' and 'stable'.
- property: cookbook
  ruby_type: String, false
  required: false
  description_list:
  - markdown: If key should be a cookbook_file, specify a cookbook where the key is
      located for files/default. Default value is nil, so it will use the cookbook
      where the resource is used.
- property: deb_src
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: Determines whether or not to add the repository as a source repo as
      well.
- property: distribution
  ruby_type: String, false
  required: false
  default_value: The LSB codename of the node such as 'focal'.
  description_list:
  - markdown: Usually a distribution's codename, such as `xenial`, `bionic`, or `focal`.
- property: key
  ruby_type: String, Array, false
  required: false
  description_list:
  - markdown: If a keyserver is provided, this is assumed to be the fingerprint; otherwise
      it can be either the URI of GPG key for the repo, or a cookbook_file.
- property: key_proxy
  ruby_type: String, false
  required: false
  description_list:
  - markdown: If set, a specified proxy is passed to GPG via `http-proxy=`.
- property: keyserver
  ruby_type: String, false
  required: false
  default_value: keyserver.ubuntu.com
  description_list:
  - markdown: The GPG keyserver where the key for the repo should be retrieved.
- property: repo_name
  ruby_type: String
  required: false
  default_value: The resource block's name
  new_in: '14.1'
  description_list:
  - markdown: An optional property to set the repository name if it differs from the
      resource block's name. The value of this setting must not contain spaces.
- property: trusted
  ruby_type: true, false
  required: false
  default_value: 'false'
  description_list:
  - markdown: Determines whether you should treat all packages from this repository
      as authenticated regardless of signature.
- property: uri
  ruby_type: String
  required: false
  description_list:
  - markdown: The base of the Debian distribution.
examples: |
  **Add repository with basic settings**:

   ```ruby
  apt_repository 'nginx' do
    uri        'http://nginx.org/packages/ubuntu/'
    components ['nginx']
  end
  ```

  **Enable Ubuntu multiverse repositories**:

  ```ruby
  apt_repository 'security-ubuntu-multiverse' do
    uri          'http://security.ubuntu.com/ubuntu'
    distribution 'xenial-security'
    components   ['multiverse']
    deb_src      true
  end
  ```

  **Add the Nginx PPA, autodetect the key and repository url**:

  ```ruby
  apt_repository 'nginx-php' do
    uri          'ppa:nginx/stable'
  end
  ```

  **Add the JuJu PPA, grab the key from the keyserver, and add source repo**:

  ```ruby
  apt_repository 'juju' do
    uri 'http://ppa.launchpad.net/juju/stable/ubuntu'
    components ['main']
    distribution 'xenial'
    key 'C8068B11'
    keyserver 'keyserver.ubuntu.com'
    action :add
    deb_src true
  end
  ```

  **Add repository that requires multiple keys to authenticate packages**:

  ```ruby
  apt_repository 'rundeck' do
    uri 'https://dl.bintray.com/rundeck/rundeck-deb'
    distribution '/'
    key ['379CE192D401AB61', 'http://rundeck.org/keys/BUILD-GPG-KEY-Rundeck.org.key']
    keyserver 'keyserver.ubuntu.com'
    action :add
  end
  ```

  **Add the Cloudera Repo of CDH4 packages for Ubuntu 16.04 on AMD64**:

  ```ruby
  apt_repository 'cloudera' do
    uri          'http://archive.cloudera.com/cdh4/ubuntu/xenial/amd64/cdh'
    arch         'amd64'
    distribution 'xenial-cdh4'
    components   ['contrib']
    key          'http://archive.cloudera.com/debian/archive.key'
  end
  ```

  **Remove a repository from the list**:

  ```ruby
  apt_repository 'zenoss' do
    action :remove
  end
  ```
---