A `Policyfile.rb` file may contain the following settings:

`name "name"`

:   Required. The name of the policy. Use a name that reflects the
    purpose of the machines against which the policy will run.

`run_list "ITEM", "ITEM", ...`

:   Required. The run-list Chef Infra Client will use to apply the
    policy to one (or more) nodes.

`default_source :SOURCE_TYPE, *args`

:   The location in which any cookbooks not specified by `cookbook` are
    located. Possible values: `chef_repo`, `chef_server`, `:community`,
    `:supermarket`, and `:artifactory`. Use more than one
    `default_source` to specify more than one location for cookbooks.

    `default_source :supermarket` pulls cookbooks from the public Chef
    Supermarket.

    `default_source :supermarket, "https://mysupermarket.example"` pulls
    cookbooks from a named private Chef Supermarket.

    `default_source :chef_server, "https://chef-server.example/organizations/example"`
    pulls cookbooks from the Chef Infra Server.

    `default_source :community` is an alias for `:supermarket`.

    `default_source :chef_repo, "path/to/repo"` pulls cookbooks from a
    monolithic cookbook repository. This may be a path to the top-level
    of a cookbook repository or to the `/cookbooks` directory within
    that repository.

    `default_source :artifactory, "https://artifactory.example/api/chef/my-supermarket"`
    pulls cookbooks from an Artifactory server. Requires either
    `artifactory_api_key` to be set in `config.rb` or
    `ARTIFACTORY_API_KEY` to be set in your environment.

    Multiple cookbook sources may be specified. For example from the
    public Chef Supermarket and a monolithic repository:

    ``` ruby
    default_source :supermarket
    default_source :chef_repo, "path/to/repo"
    ```

    or from both a public and private Chef Supermarket:

    ``` ruby
    default_source :supermarket
    default_source :supermarket, "https://supermarket.example"
    ```

    <div class="admonition-note">

    <p class="admonition-note-title">Note</p>

    <div class="admonition-note-text">

    If a run-list or any dependencies require a cookbook that is present
    in more than one source, be explicit about which source is
    preferred. This will ensure that a cookbook is always pulled from an
    expected source. For example, an internally-developed cookbook named
    `chef-client` will conflict with the public `chef-client` cookbook
    that is maintained by Chef. To specify a named source for a
    cookbook:

    ``` ruby
    default_source :supermarket
    default_source :supermarket, "https://supermarket.example" do |s|
      s.preferred_for "chef-client"
    end
    ```

    List multiple cookbooks on the same line:

    ``` ruby
    default_source :supermarket
    default_source :supermarket, "https://supermarket.example" do |s|
      s.preferred_for "chef-client", "nginx", "mysql"
    end
    ```

    

    </div>

    </div>

`cookbook "NAME" [, "VERSION_CONSTRAINT"] [, SOURCE_OPTIONS]`

:   Add cookbooks to the policy, specify a version constraint, or
    specify an alternate source location, such as Chef Supermarket. For
    example, add a cookbook:

    ``` ruby
    cookbook "apache2"
    ```

    Specify a version constraint:

    ``` ruby
    run_list "jenkins::master"

    # Restrict the jenkins cookbook to version 2.x, greater than 2.1
    cookbook "jenkins", "~> 2.1"
    ```

    Specify an alternate source:

    ``` ruby
    cookbook 'my_app', path: 'cookbooks/my_app'
    ```

    or:

    ``` ruby
    cookbook 'mysql', github: 'opscode-cookbooks/mysql', branch: 'master'
    ```

    or:

    ``` ruby
    cookbook 'chef-ingredient', git: 'https://github.com/chef-cookbooks/chef-ingredient.git', tag: 'v0.12.0'
    ```

`named_run_list "NAME", "ITEM1", "ITEM2", ...`

:   Specify a named run-list to be used as an alternative to the
    override run-list. This setting should be used carefully and for
    specific use cases, like running a small set of recipes to quickly
    converge configuration for a single application on a host or for
    one-time setup tasks. For example:

    ``` ruby
    named_run_list :update_app, "my_app_cookbook::default"
    ```

`include_policy "NAME", *args`

:   Specify a policyfile lock to be merged with this policy. Chef
    Workstation supports pulling this lock from a local or remote file,
    from a Chef Infra Server, or from a git repository. When the
    policyfile lock is included, its run-list will appear before the
    current policyfile's run-list. This setting requires that the solved
    cookbooks appear as-is from the included policyfile lock. If
    conflicting attributes or cookbooks are provided, an error will be
    presented. See
    [RFC097](https://github.com/chef/chef-rfc/blob/master/rfc097-policyfile-includes.md)
    for the full specifications of this feature.

    Pull the policyfile lock from `./NAME.lock.json`:

    ``` ruby
    include_policy "NAME", path: "."
    ```

    Pull the policyfile lock from `./foo.lock.json`.

    ``` ruby
    include_policy "NAME", path: "./foo.lock.json"
    ```

    Pull the policyfile lock from `./bar.lock.json` with revision ID
    'revision1'.

    ``` ruby
    include_policy "NAME", policy_revision_id: "revision1", path: "./bar.lock.json"
    ```

    Pull the policyfile lock from a remote server
    `https://internal.example.com/foo.lock.json`.

    ``` ruby
    include_policy "NAME", remote: "https://internal.example.com/foo.lock.json"
    ```

    Pull the policyfile lock from a remote server
    `https://internal.example.com/bar.lock.json` and with revision ID
    'revision1'.

    ``` ruby
    include_policy "NAME", policy_revision_id: "revision1", remote: "https://internal.example.com/foo.lock.json"
    ```

    Pull the policy `NAME` with revision ID `revision1` from the
    `http://chef-server.example` Chef Infra Server:

    ``` ruby
    include_policy "NAME", policy_revision_id: "revision1", server: "http://chef-server.example"
    ```

    Pull the policy `foo` with revision ID `revision1`:

    ``` ruby
    include_policy "NAME", policy_name: "foo", policy_revision_id: "revision1", server: "http://chef-server.example"
    ```

    Pull and lock the current revision for policy `foo` in policy group
    `prod`:

    ``` ruby
    include_policy "NAME", policy_name: "foo", policy_group: "prod", server: "http://chef-server.example"
    ```