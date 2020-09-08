+++
title = "Sending Compliance Data to Chef Automate with Audit Cookbook"
draft = false
robots = "noindex"


aliases = ["/audit_cookbook.html", "/audit_supported_configurations.html"]

[menu]
  [menu.legacy]
    title = "Audit Cookbook"
    identifier = "legacy/workflow/workflow_basics/audit_cookbook.md Audit Cookbook"
    parent = "legacy/workflow/workflow_basics"
    weight = 70
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/audit_cookbook.md)



{{< note >}}

The `audit` cookbook uses the Chef InSpec gem. It will check for an
installed version of Chef InSpec and install it upon finding none. The
latest version of Chef InSpec will be installed by default unless
otherwise specified with `node['audit']['inspec_version']`.

{{< /note >}}

[Cookbooks](/cookbooks/) are Chef's primary unit of configuration
management. For tutorials on working with cookbooks in Chef, see [Learn
Chef Rally](https://learn.chef.io).

## Audit Cookbook

{{< note >}}

Audit Cookbook version 4.2.0 or later requires Chef InSpec 1.25.1 or
later. You can upgrade your Chef InSpec package in several different
ways: by upgrading Automate, by upgrading the Chef Workstation, by
upgrading Chef Infra Client, or by setting the
`node['audit']['inspec_version']` attribute in your cookbook.

{{< /note >}}

To send compliance data gathered by Chef InSpec as part of a Chef Infra
Client run, you will need to use the [audit
cookbook](https://github.com/chef-cookbooks/audit). All profiles
configured to run during the audit cookbook execution will send their
results back to the Chef Automate server.

### Configure the Node for Audit Cookbook

Once the cookbook is available in Chef Infra Server, you will need to
add the `audit::default` recipe to the run-list of each node. Compliance
profiles are added using the `node['audit']['profiles']` attribute. A
complete list of the configurations is documented on GitHub in the
[Audit Cookbook
Repository](https://github.com/chef-cookbooks/audit/blob/master/README.md).

To configure the audit cookbook to report compliance data directly to
Chef Automate, you will first need to configure Chef Infra Client to
send node converge data, as described in [Data
Collection](/data_collection/). Next, configure the audit cookbook
collector by setting the `reporter`, `server`, `owner`, `refresh_token`
and `profiles` attributes.

-   `reporter` - `'chef-automate'` to report to Chef Automate.
-   `server` - url of Chef Automate server with `/api`.
-   `owner` - Chef Automate user or organization that will receive this
    scan report.
-   `refresh_token` - refresh token for Chef Automate API. Please note
    that logging out of the user interface revokes the `refresh_token`.
    To workaround, log in once in a private browser session, grab the
    token and then close the browser without logging out.
-   `insecure` - a `true` value will skip the SSL certificate
    verification when retrieving an access token. The default value is
    `false`.

A complete audit cookbook attribute configuration will look something
like this:

``` ruby
['audit']['reporter'] = 'chef-automate'
['audit']['server'] = 'https://chef-automate-server/api'
['audit']['owner'] = 'my-comp-org'
['audit']['refresh_token'] = '5/4T...g=='
['audit']['insecure'] = false
['audit']['profiles'] = [
  {
   'name': 'windows',
   'compliance': 'base/windows'
   }
]
```

Instead of a refresh token, it is also possible to use a `token` that
expires in 12h after creation.

``` ruby
['audit']['reporter'] = 'chef-automate'
['audit']['server'] =  'https://chef-automate-fqdn/api'
['audit']['owner'] =  'my-comp-org'
['audit']['token'] =  'eyJ........................YQ'
['audit']['profiles'] = [
   {
     'name': 'windows',
     'compliance': 'base/windows'
     }
 ]
```

## Supported Audit Cookbook Configurations

The `audit` cookbook supports several different methods of fetching and
reporting compliance information.

{{% EOL_compliance_server %}}

### Fetch From Automate via Chef Infra Server

{{< note >}}

The Compliance server must be integrated with Chef Infra Server for use
in reporting.

{{< /note >}}

<table>
<thead>
<tr class="header">
<th>Action</th>
<th>Configuration</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>Fetch From Automate via Chef Infra Server and Report Directly to Automate</td>
<td><div class="sourceCode" id="cb1"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb1-1"><a href="#cb1-1"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;reporter&#39;</span>] = <span class="st">&#39;chef-automate&#39;</span></span>
<span id="cb1-2"><a href="#cb1-2"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;fetcher&#39;</span>] = <span class="st">&#39;chef-server&#39;</span></span>
<span id="cb1-3"><a href="#cb1-3"></a><span class="co">#Set in chef-server.rb:</span></span>
<span id="cb1-4"><a href="#cb1-4"></a>profiles[<span class="st">&#39;root_url&#39;</span>] = <span class="st">&#39;https://chef-automate.test&#39;</span></span>
<span id="cb1-5"><a href="#cb1-5"></a><span class="co">#Set in client.rb:</span></span>
<span id="cb1-6"><a href="#cb1-6"></a>data_collector[<span class="st">&#39;server_url&#39;</span>] = <span class="st">&#39;https://chef-automate.test/data-collector/v0/&#39;</span></span>
<span id="cb1-7"><a href="#cb1-7"></a>data_collector[<span class="st">&#39;token&#39;</span>] = <span class="st">&#39;..&#39;</span></span>
<span id="cb1-8"><a href="#cb1-8"></a><span class="co">#Set in delivery.rb:</span></span>
<span id="cb1-9"><a href="#cb1-9"></a>compliance_profiles[<span class="st">&quot;enable&quot;</span>] = <span class="dv">true</span></span></code></pre></div></td>
</tr>
<tr class="even">
<td>Fetch From Automate via Chef Infra Server and Report Directly to Compliance</td>
<td><div class="sourceCode" id="cb2"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb2-1"><a href="#cb2-1"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;reporter&#39;</span>] = <span class="st">&#39;chef-compliance&#39;</span></span>
<span id="cb2-2"><a href="#cb2-2"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;fetcher&#39;</span>] = <span class="st">&#39;chef-server&#39;</span></span>
<span id="cb2-3"><a href="#cb2-3"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;server&#39;</span>] = <span class="st">&#39;https://compliance-server.test/api&#39;</span></span>
<span id="cb2-4"><a href="#cb2-4"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;refresh_token&#39;</span> <span class="dt">OR</span> <span class="st">&#39;token&#39;</span>] = <span class="st">&#39;..&#39;</span></span>
<span id="cb2-5"><a href="#cb2-5"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;owner&#39;</span>] = <span class="st">&#39;User/Org&#39;</span></span>
<span id="cb2-6"><a href="#cb2-6"></a><span class="co"># Set in chef-server.rb:</span></span>
<span id="cb2-7"><a href="#cb2-7"></a>profiles[<span class="st">&#39;root_url&#39;</span>] = <span class="st">&#39;https://automate-server.test&#39;</span></span>
<span id="cb2-8"><a href="#cb2-8"></a><span class="co"># Set in delivery.rb:</span></span>
<span id="cb2-9"><a href="#cb2-9"></a>compliance_profiles[<span class="st">&quot;enable&quot;</span>] = <span class="dv">true</span></span></code></pre></div></td>
</tr>
<tr class="odd">
<td>Fetch From Automate via Chef Infra Server and Report to Automate via Chef Infra Server</td>
<td><div class="sourceCode" id="cb3"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb3-1"><a href="#cb3-1"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;reporter&#39;</span>] = <span class="st">&#39;chef-server-automate&#39;</span></span>
<span id="cb3-2"><a href="#cb3-2"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;fetcher&#39;</span>] = <span class="st">&#39;chef-server&#39;</span></span>
<span id="cb3-3"><a href="#cb3-3"></a><span class="co">#Set in chef-server.rb:</span></span>
<span id="cb3-4"><a href="#cb3-4"></a>data_collector[<span class="st">&#39;root_url&#39;</span>] = <span class="st">&#39;https://chef-automate.test/data-collector/v0/&#39;</span></span>
<span id="cb3-5"><a href="#cb3-5"></a>profiles[<span class="st">&#39;root_url&#39;</span>] = <span class="st">&#39;https://chef-automate.test&#39;</span></span>
<span id="cb3-6"><a href="#cb3-6"></a><span class="co">#Set in delivery.rb:</span></span>
<span id="cb3-7"><a href="#cb3-7"></a>compliance_profiles[<span class="st">&quot;enable&quot;</span>] = <span class="dv">true</span></span></code></pre></div></td>
</tr>
<tr class="even">
<td>Fetch From Automate via Chef Infra Server and Report to Compliance via Chef Infra Server</td>
<td><div class="sourceCode" id="cb4"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb4-1"><a href="#cb4-1"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;reporter&#39;</span>] = <span class="st">&#39;chef-server-compliance&#39;</span></span>
<span id="cb4-2"><a href="#cb4-2"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;fetcher&#39;</span>] = <span class="st">&#39;chef-server&#39;</span></span>
<span id="cb4-3"><a href="#cb4-3"></a><span class="co">#Set in chef-server.rb:</span></span>
<span id="cb4-4"><a href="#cb4-4"></a>profiles[<span class="st">&#39;root_url&#39;</span>] = <span class="st">&#39;https://chef-automate.test&#39;</span></span>
<span id="cb4-5"><a href="#cb4-5"></a><span class="co">#Set in delivery.rb:</span></span>
<span id="cb4-6"><a href="#cb4-6"></a>compliance_profiles[<span class="st">&quot;enable&quot;</span>] = <span class="dv">true</span></span></code></pre></div></td>
</tr>
</tbody>
</table>

### Fetch From Compliance via Chef Infra Server

{{< note >}}

The Compliance server must be integrated with Chef Infra Server for use
in reporting.

{{< /note >}}

<table>
<thead>
<tr class="header">
<th>Action</th>
<th>Configuration</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>Fetch From Compliance via Chef Infra Server and Report Directly to Automate</td>
<td><div class="sourceCode" id="cb1"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb1-1"><a href="#cb1-1"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;reporter&#39;</span>] = <span class="st">&#39;chef-automate&#39;</span></span>
<span id="cb1-2"><a href="#cb1-2"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;fetcher&#39;</span>] = <span class="st">&#39;chef-server&#39;</span></span>
<span id="cb1-3"><a href="#cb1-3"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;server&#39;</span>] = <span class="st">&#39;https://compliance-server.test/api&#39;</span></span>
<span id="cb1-4"><a href="#cb1-4"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;refresh_token&#39;</span> <span class="dt">OR</span> <span class="st">&#39;token&#39;</span>] = <span class="st">&#39;..&#39;</span></span>
<span id="cb1-5"><a href="#cb1-5"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;owner&#39;</span>] = <span class="st">&#39;User/Org&#39;</span></span>
<span id="cb1-6"><a href="#cb1-6"></a><span class="co">#Set in client.rb:</span></span>
<span id="cb1-7"><a href="#cb1-7"></a>data_collector[<span class="st">&#39;server_url&#39;</span>] = <span class="st">&#39;https://chef-automate.test/data-collector/v0/&#39;</span></span>
<span id="cb1-8"><a href="#cb1-8"></a>data_collector[<span class="st">&#39;token&#39;</span>] = <span class="st">&#39;..&#39;</span></span></code></pre></div></td>
</tr>
<tr class="even">
<td>Fetch From Compliance via Chef Infra Server and Report Directly to Compliance</td>
<td><div class="sourceCode" id="cb2"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb2-1"><a href="#cb2-1"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;reporter&#39;</span>] = <span class="st">&#39;chef-compliance&#39;</span></span>
<span id="cb2-2"><a href="#cb2-2"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;fetcher&#39;</span>] = <span class="st">&#39;chef-server&#39;</span></span>
<span id="cb2-3"><a href="#cb2-3"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;server&#39;</span>] = <span class="st">&#39;https://compliance-server.test/api&#39;</span></span>
<span id="cb2-4"><a href="#cb2-4"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;refresh_token&#39;</span> <span class="dt">OR</span> <span class="st">&#39;token&#39;</span>] = <span class="st">&#39;..&#39;</span></span>
<span id="cb2-5"><a href="#cb2-5"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;owner&#39;</span>] = <span class="st">&#39;User/Org&#39;</span></span></code></pre></div></td>
</tr>
<tr class="odd">
<td>Fetch From Compliance via Chef Infra Server and Report to Compliance via Chef Infra Server</td>
<td><div class="sourceCode" id="cb3"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb3-1"><a href="#cb3-1"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;reporter&#39;</span>] = <span class="st">&#39;chef-server-compliance&#39;</span></span>
<span id="cb3-2"><a href="#cb3-2"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;fetcher&#39;</span>] = <span class="st">&#39;chef-server&#39;</span></span></code></pre></div></td>
</tr>
<tr class="even">
<td>Fetch From Compliance via Chef Infra Server and Report to Automate via Chef Infra Server</td>
<td><div class="sourceCode" id="cb4"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb4-1"><a href="#cb4-1"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;reporter&#39;</span>] = <span class="st">&#39;chef-server-automate&#39;</span></span>
<span id="cb4-2"><a href="#cb4-2"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;fetcher&#39;</span>] = <span class="st">&#39;chef-server&#39;</span></span>
<span id="cb4-3"><a href="#cb4-3"></a><span class="co">#Set in chef-server.rb:</span></span>
<span id="cb4-4"><a href="#cb4-4"></a>data_collector[<span class="st">&#39;root_url&#39;</span>] = <span class="st">&#39;https://chef-automate.test/data-collector/v0/&#39;</span></span></code></pre></div></td>
</tr>
</tbody>
</table>

### Fetch Directly From Compliance

<table>
<thead>
<tr class="header">
<th>Action</th>
<th>Configuration</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>Fetch from Compliance and Report Directly to Automate</td>
<td><div class="sourceCode" id="cb1"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb1-1"><a href="#cb1-1"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;reporter&#39;</span>] = <span class="st">&#39;chef-automate&#39;</span></span>
<span id="cb1-2"><a href="#cb1-2"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;server&#39;</span>] = <span class="st">&#39;https://compliance-server.test/api&#39;</span></span>
<span id="cb1-3"><a href="#cb1-3"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;refresh_token&#39;</span> <span class="dt">OR</span> <span class="st">&#39;token&#39;</span>] = <span class="st">&#39;..&#39;</span></span>
<span id="cb1-4"><a href="#cb1-4"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;owner&#39;</span>] = <span class="st">&#39;User/Org&#39;</span></span>
<span id="cb1-5"><a href="#cb1-5"></a><span class="co">#</span></span>
<span id="cb1-6"><a href="#cb1-6"></a><span class="co">#Set in the client.rb:</span></span>
<span id="cb1-7"><a href="#cb1-7"></a>data_collector[<span class="st">&#39;server_url&#39;</span>] = <span class="st">&#39;https://chef-automate.test/data-collector/v0/&#39;</span></span>
<span id="cb1-8"><a href="#cb1-8"></a>data_collector[<span class="st">&#39;token&#39;</span>] = <span class="st">&#39;..&#39;</span></span></code></pre></div></td>
</tr>
<tr class="even">
<td>Fetch from Compliance and Report Directly to Compliance</td>
<td><div class="sourceCode" id="cb2"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb2-1"><a href="#cb2-1"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;reporter&#39;</span>] = <span class="st">&#39;chef-compliance&#39;</span></span>
<span id="cb2-2"><a href="#cb2-2"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;server&#39;</span>] = <span class="st">&#39;https://compliance-server.test/api&#39;</span></span>
<span id="cb2-3"><a href="#cb2-3"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;refresh_token&#39;</span> <span class="dt">OR</span> <span class="st">&#39;token&#39;</span>] = <span class="st">&#39;..&#39;</span></span>
<span id="cb2-4"><a href="#cb2-4"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;owner&#39;</span>] = <span class="st">&#39;User/Org&#39;</span></span></code></pre></div></td>
</tr>
<tr class="odd">
<td>Fetch from Compliance and Report to Automate via Chef Infra Server</td>
<td><div class="sourceCode" id="cb3"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb3-1"><a href="#cb3-1"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;reporter&#39;</span>] = <span class="st">&#39;chef-server-automate&#39;</span></span>
<span id="cb3-2"><a href="#cb3-2"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;server&#39;</span>] = <span class="st">&#39;https://compliance-server.test/api&#39;</span></span>
<span id="cb3-3"><a href="#cb3-3"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;refresh_token&#39;</span> <span class="dt">OR</span> <span class="st">&#39;token&#39;</span>] = <span class="st">&#39;..&#39;</span></span>
<span id="cb3-4"><a href="#cb3-4"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;owner&#39;</span>] = <span class="st">&#39;User/Org&#39;</span></span>
<span id="cb3-5"><a href="#cb3-5"></a><span class="co">#Set in chef-server.rb:</span></span>
<span id="cb3-6"><a href="#cb3-6"></a>data_collector[<span class="st">&#39;root_url&#39;</span>] = <span class="st">&#39;https://chef-automate.test/data-collector/v0/&#39;</span></span></code></pre></div></td>
</tr>
<tr class="even">
<td>Fetch from Compliance and Report to Compliance via Chef Infra Server</td>
<td><div class="sourceCode" id="cb4"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb4-1"><a href="#cb4-1"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;reporter&#39;</span>] = <span class="st">&#39;chef-server-compliance&#39;</span></span>
<span id="cb4-2"><a href="#cb4-2"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;server&#39;</span>] = <span class="st">&#39;https://compliance-server.test/api&#39;</span></span>
<span id="cb4-3"><a href="#cb4-3"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;refresh_token&#39;</span> <span class="dt">OR</span> <span class="st">&#39;token&#39;</span>] = <span class="st">&#39;..&#39;</span></span>
<span id="cb4-4"><a href="#cb4-4"></a>[<span class="st">&#39;audit&#39;</span>][<span class="st">&#39;owner&#39;</span>] = <span class="st">&#39;User/Org&#39;</span></span></code></pre></div></td>
</tr>
</tbody>
</table>
