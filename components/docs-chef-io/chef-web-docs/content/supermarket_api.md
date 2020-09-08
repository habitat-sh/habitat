+++
title = "Supermarket API"
draft = false

aliases = ["/supermarket_api.html"]

[menu]
  [menu.infra]
    title = "Supermarket API"
    identifier = "chef_infra/setup/supermarket/supermarket_api.md Supermarket API"
    parent = "chef_infra/setup/supermarket"
    weight = 100
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/supermarket_api.md)

{{% supermarket_api_summary %}} .. note:: In general, using
knife (and the `knife supermarket` subcommand) to manage cookbooks that
are located on the Cookbooks site is more efficient than using the
Supermarket API and is the recommended approach for managing cookbooks
on that site. This document provides information about the Supermarket
API in the event that using the API is necessary.

## Endpoints

The Supermarket API has the following endpoints.

### /cookbooks

{{% cookbooks_summary %}}

The `/cookbooks` endpoint has the following methods: `GET` and `POST`.

#### POST

The `POST` method is used to create a new cookbook.

This method has no parameters.

**Request**

``` none
POST /api/v1/cookbooks/COOKBOOK_NAME
```

**Response**

The response is similar to:

``` javascript
{
  "name": "apt",
  "maintainer": "opscode",
  "description": "Configures apt and apt services and LWRPs for managing apt repositories and preferences",
  "category": "Package Management",
  "latest_version": "http://supermarket.chef.io/api/v1/cookbooks/apt/versions/2_4_0",
  "external_url": "https://github.com/chef-cookbooks/apt",
  "average_rating": null,
  "created_at": "2009-10-25T23:48:48.000Z",
  "updated_at": "2014-05-15T17:45:14.000Z"
}
```

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful. The cookbook was posted to the Supermarket API.</td>
</tr>
<tr class="even">
<td><p><code>400</code></p></td>
<td><p>The request was unsuccessful. The cookbook was not posted to the Supermarket API. For example:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode javascript"><code class="sourceCode javascript"><span id="cb1-1"><a href="#cb1-1"></a><span class="op">{</span></span>
<span id="cb1-2"><a href="#cb1-2"></a>   <span class="st">&quot;error_messages&quot;</span><span class="op">:</span></span>
<span id="cb1-3"><a href="#cb1-3"></a>   [<span class="st">&quot;You&#39;re not authorized to upload this cookbook.&quot;</span>]<span class="op">,</span></span>
<span id="cb1-4"><a href="#cb1-4"></a>   <span class="st">&quot;error_code&quot;</span><span class="op">:</span> <span class="st">&quot;UNAUTHORIZED&quot;</span></span>
<span id="cb1-5"><a href="#cb1-5"></a><span class="op">}</span></span></code></pre></div></td>
</tr>
</tbody>
</table>

#### GET

The `GET` method is used to get a listing of the available cookbooks.
Use the `start` and `items` parameters to set limits on the number of
cookbooks returned. Use the `order` parameter to change the way results
are sorted. Use the `user` parameter to filter cookbooks by maintainer:

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Parameter</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>start</code></td>
<td>The offset into a list of cookbooks, at which point the list of cookbooks will begin.</td>
</tr>
<tr class="even">
<td><code>items</code></td>
<td>The number of items to be returned as a result of the request.</td>
</tr>
<tr class="odd">
<td><code>order</code></td>
<td>A token specifying how to order results. Possible values: <code>recently_updated</code>, <code>recently_added</code>, <code>most_downloaded</code>, or <code>most_followed</code>.</td>
</tr>
<tr class="even">
<td><code>user</code></td>
<td>The username to filter by. Only cookbooks maintained by this user will be returned.</td>
</tr>
</tbody>
</table>

**Request**

``` none
GET /api/v1/cookbooks?start=START&items=ITEMS
```

or:

``` none
GET /api/v1/cookbooks?user=smith
```

**Response**

The response will return the name of the cookbook, a description, URI,
the name of the individual who maintains the cookbook. In addition, the
total number of cookbooks on Supermarket API is shown, as well (if
`start` is specified) the point at which the list of returned cookbooks
began:

``` javascript
{
   "total": 5234,
   "start": 20,
   "items":
      [
        {"cookbook_name": "apache",
         "cookbook_description": "installs apache.",
         "cookbook": "http://supermarket.chef.io/api/v1/cookbooks/apache",
         "cookbook_maintainer": "john"
        },
        {"cookbook_name": "fail2ban",
         "cookbook_description": "installs fail2ban.",
         "cookbook": "http://supermarket.chef.io/api/v1/cookbooks/fail2ban",
         "cookbook_maintainer": "jill"
        },
        {"cookbook_name": "mysql",
         "cookbook_description": null,
         "cookbook": "http://supermarket.chef.io/api/v1/cookbooks/mysql",
         "cookbook_maintainer": "barry"
        },
        {"cookbook_name": "capistrano",
         "cookbook_description": null,
         "cookbook": "http://supermarket.chef.io/api/v1/cookbooks/capistrano",
         "cookbook_maintainer": "pt"
        },
        {"cookbook_name": "ptapache",
         "cookbook_description": "an alternate apache recipe.",
         "cookbook": "http://supermarket.chef.io/api/v1/cookbooks/ptapache",
         "cookbook_maintainer": "pt"
        }
      ]
}
```

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful. One or more cookbooks were returned as a result of the search query.</td>
</tr>
</tbody>
</table>

### /cookbooks/NAME

The `cookbooks/[NAME]` endpoint allows a specific cookbook to be
accessed. This endpoint has the following methods: `DELETE` and `GET`.

#### DELETE

The `DELETE` method is used to delete a cookbook.

This method has no parameters.

**Request**

``` none
DELETE /api/v1/cookbooks/cookbook_name
```

**Response**

The response is similar to:

``` javascript
{
  "name": "apt",
  "maintainer": "opscode",
  "description": "Configures apt and apt services and LWRPs for managing apt repositories and preferences",
  "category": "Package Management",
  "latest_version": "http://supermarket.chef.io/api/v1/cookbooks/apt/versions/2_4_0",
  "external_url": "https://github.com/chef-cookbooks/apt",
  "average_rating": null,
  "created_at": "2009-10-25T23:48:48.000Z",
  "updated_at": "2014-05-15T17:45:14.000Z"
}
```

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful. The cookbook was deleted.</td>
</tr>
<tr class="even">
<td><p><code>400</code></p></td>
<td><p>The request was unsuccessful. The requested cookbook does not exist. For example:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode javascript"><code class="sourceCode javascript"><span id="cb1-1"><a href="#cb1-1"></a><span class="op">{</span></span>
<span id="cb1-2"><a href="#cb1-2"></a>   <span class="st">&quot;error_messages&quot;</span><span class="op">:</span></span>
<span id="cb1-3"><a href="#cb1-3"></a>   [<span class="st">&quot;Resource does not exist&quot;</span>]<span class="op">,</span></span>
<span id="cb1-4"><a href="#cb1-4"></a>   <span class="st">&quot;error_code&quot;</span><span class="op">:</span> <span class="st">&quot;NOT_FOUND&quot;</span></span>
<span id="cb1-5"><a href="#cb1-5"></a><span class="op">}</span></span></code></pre></div></td>
</tr>
<tr class="odd">
<td><p><code>403</code></p></td>
<td><p>Unauthorized. The user who made the request is not authorized to perform the action. The user is not authorized to delete the cookbook. For example:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode javascript"><code class="sourceCode javascript"><span id="cb2-1"><a href="#cb2-1"></a><span class="op">{}</span></span></code></pre></div></td>
</tr>
</tbody>
</table>

#### GET

The `GET` method is used to get the details for a cookbook.

This method has no parameters.

**Request**

``` none
GET /api/v1/cookbooks/COOKBOOK_NAME
```

**Response**

The response will return details for a cookbook, including name of the
cookbook, the category to which it belongs, the name of the individual
who maintains the cookbook, the URI for the latest version and previous
versions, its description, and so on it also includes metrics about the
cookbooks namely number of downloads and followers:

``` javascript
{
  "name": "yum",
  "maintainer": "opscode",
  "description": "Configures various yum components on Red Hat-like systems",
  "category": "Package Management",
  "latest_version": "http://supermarket.chef.io/api/v1/cookbooks/yum/versions/3_2_2",
  "external_url": "https://github.com/chef-cookbooks/yum",
  "average_rating": null,
  "created_at": "2011-04-20T22:16:12.000Z",
  "updated_at": "2014-06-11T19:06:37.000Z",
  "deprecated": false,
  "versions": [
    "http://supermarket.chef.io/api/v1/cookbooks/yum/versions/3_2_2",
    "http://supermarket.chef.io/api/v1/cookbooks/yum/versions/3_2_0"
  ],
  "metrics": {
    "downloads": {
      "total": 8500
      "versions": {
        "3.2.0": 399,
        "3.2.2": 1
      }
   },
   "followers": 55
  }
}
```

If a cookbook is deprecated, that status is noted by the `deprecated`
field (being `true`):

``` javascript
{
  "name": "apache",
  "category": "web servers",
  ...
  "deprecated": true,
  ...
}
```

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful. The requested cookbook exists.</td>
</tr>
<tr class="even">
<td><p><code>400</code></p></td>
<td><p>The request was unsuccessful. The requested cookbook does not exist. For example:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode javascript"><code class="sourceCode javascript"><span id="cb1-1"><a href="#cb1-1"></a><span class="op">{</span></span>
<span id="cb1-2"><a href="#cb1-2"></a>   <span class="st">&quot;error_messages&quot;</span><span class="op">:</span></span>
<span id="cb1-3"><a href="#cb1-3"></a>   [<span class="st">&quot;Resource does not exist&quot;</span>]<span class="op">,</span></span>
<span id="cb1-4"><a href="#cb1-4"></a>   <span class="st">&quot;error_code&quot;</span><span class="op">:</span> <span class="st">&quot;NOT_FOUND&quot;</span></span>
<span id="cb1-5"><a href="#cb1-5"></a><span class="op">}</span></span></code></pre></div></td>
</tr>
</tbody>
</table>

### /cookbooks/VERSION

A cookbook version always takes the form x.y.z, where x, y, and z are
decimal numbers that are used to represent major (x), minor (y), and
patch (z) versions. A two-part version (x.y) is also allowed. When
passing a cookbook version using this method, underscores ("_") should
be used as the separator between versions. For example, a cookbook with
a version 1.0.1 would be 1_0_1.

The `/cookbooks/[VERSION]` endpoint has the following methods: `DELETE`
and `GET`.

#### DELETE

The `DELETE` method is used to delete a cookbook version.

This method has no parameters.

**Request**

``` none
DELETE /api/v1/cookbooks/cookbook_name/versions/version
```

**Response**

The response is similar to:

``` javascript
{
  "license": "Apache 2.0",
  "tarball_file_size": 18553,
  "version": "2.4.0",
  "average_rating": null,
  "cookbook": "http://supermarket.chef.io/api/v1/cookbooks/apt",
  "file": "http://supermarket.chef.io/api/v1/cookbooks/apt/versions/2_4_0/download",
  "dependencies": {},
  "platforms": {
    "debian": ">= 0.0.0",
    "ubuntu": ">= 0.0.0"
  }
}
```

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful. The cookbook version was deleted.</td>
</tr>
<tr class="even">
<td><p><code>400</code></p></td>
<td><p>The request was unsuccessful. The requested cookbook or cookbook version does not exist. For example:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode javascript"><code class="sourceCode javascript"><span id="cb1-1"><a href="#cb1-1"></a><span class="op">{</span></span>
<span id="cb1-2"><a href="#cb1-2"></a>   <span class="st">&quot;error_messages&quot;</span><span class="op">:</span></span>
<span id="cb1-3"><a href="#cb1-3"></a>   [<span class="st">&quot;Resource does not exist&quot;</span>]<span class="op">,</span></span>
<span id="cb1-4"><a href="#cb1-4"></a>   <span class="st">&quot;error_code&quot;</span><span class="op">:</span> <span class="st">&quot;NOT_FOUND&quot;</span></span>
<span id="cb1-5"><a href="#cb1-5"></a><span class="op">}</span></span></code></pre></div></td>
</tr>
<tr class="odd">
<td><p><code>403</code></p></td>
<td><p>Unauthorized. The user who made the request is not authorized to perform the action. The user is not authorized to delete the cookbook version. For example:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode javascript"><code class="sourceCode javascript"><span id="cb2-1"><a href="#cb2-1"></a><span class="op">{}</span></span></code></pre></div></td>
</tr>
</tbody>
</table>

#### GET

The `GET` method is used to get a specific version of a cookbook. Use
`latest` to get the most recent version of a cookbook.

This method has no parameters.

**Request**

``` none
GET /api/v1/cookbooks/COOKBOOK_NAME/versions/latest
```

or:

``` none
GET /api/v1/cookbooks/COOKBOOK_NAME/versions/VERSION
```

**Response**

The response will return details for a cookbook version, including the
license under which the cookbook is distributed, the most recent update,
version, URI, date of cookbook creation, path to the cookbook's tar.gz
file, its dependencies and platforms it supports and so on:

``` javascript
{
  "license": "Apache 2.0",
  "tarball_file_size": 18553,
  "version": "2.4.0",
  "average_rating": null,
  "cookbook": "http://supermarket.chef.io/api/v1/cookbooks/apt",
  "file": "http://supermarket.chef.io/api/v1/cookbooks/apt/versions/2_4_0/download",
  "dependencies": {},
  "platforms": {
    "debian": ">= 0.0.0",
    "ubuntu": ">= 0.0.0"
  }
}
```

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful. The requested cookbook exists.</td>
</tr>
<tr class="even">
<td><p><code>400</code></p></td>
<td><p>The request was unsuccessful. The requested cookbook does not exist. For example:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode javascript"><code class="sourceCode javascript"><span id="cb1-1"><a href="#cb1-1"></a><span class="op">{</span></span>
<span id="cb1-2"><a href="#cb1-2"></a>   <span class="st">&quot;error_messages&quot;</span><span class="op">:</span></span>
<span id="cb1-3"><a href="#cb1-3"></a>   [<span class="st">&quot;Resource does not exist&quot;</span>]<span class="op">,</span></span>
<span id="cb1-4"><a href="#cb1-4"></a>   <span class="st">&quot;error_code&quot;</span><span class="op">:</span> <span class="st">&quot;NOT_FOUND&quot;</span></span>
<span id="cb1-5"><a href="#cb1-5"></a><span class="op">}</span></span></code></pre></div></td>
</tr>
</tbody>
</table>

### /search

Search performs a fuzzy, keyword search on cookbook names, cookbook
descriptions, and the cookbook owners' usernames.

The `/search` endpoint has the following methods: `GET`.

#### GET

The `GET` method is used to get a list of cookbooks that match a search
query. Use the `start` and `items` parameters to set limits on the
number of cookbooks returned:

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Parameter</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>q</code></td>
<td>The search query used to identify a list of items on a Chef Infra Server. This option uses the same syntax as the <code>search</code> subcommand.</td>
</tr>
<tr class="even">
<td><code>start</code></td>
<td>The row at which return results begin.</td>
</tr>
<tr class="odd">
<td><code>items</code></td>
<td>The number of rows to be returned.</td>
</tr>
</tbody>
</table>

**Request**

``` none
GET /api/v1/search?q=SEARCH_QUERY
```

or:

``` none
GET /api/v1/search?q=SEARCH_QUERY&start=START&items=ITEMS
```

**Response**

The response will return a list of cookbooks by name and description and
will return a list of cookbooks that match the search query. Each
returned data set will include the name of the cookbook, a description,
the URI, and the name of the individual who maintains the cookbook. In
addition, the total number of cookbooks on Supermarket API is shown, as
well (if `start` is specified) the point at which the list of returned
cookbooks began:

``` javascript
{
  "total": 2,
  "start": 0,
  "items": [
    {
      "cookbook_name": "apache",
      "cookbook_description": "installs a web server.",
      "cookbook": "http://supermarket.chef.io/api/v1/cookbooks/apache",
      "cookbook_maintainer": "jtimberman"
    },
    {
      "cookbook_name": "webserver",
      "cookbook_description": "installs apache.",
      "cookbook": "http://supermarket.chef.io/api/v1/cookbooks/webserver",
      "cookbook_maintainer": "raxmus"
    }
  ]
}
```

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful. One or more cookbooks were returned as a result of the search query.</td>
</tr>
</tbody>
</table>

### /tools

The `tools` endpoint allows Chef Supermarket tools to be accessed. This
endpoint has the following methods: `GET`.

#### GET

The `GET` method is used to get a listing of the available tools. Use
the `start` and `items` parameters to set limits on the number of tools
returned. Use the `order` parameter to change the way results are
sorted.

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Parameter</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>start</code></td>
<td>The offset into a list of tools, at which point the list of tools will begin.</td>
</tr>
<tr class="even">
<td><code>items</code></td>
<td>The number of items to be returned as a result of the request.</td>
</tr>
<tr class="odd">
<td><code>order</code></td>
<td>A token specifying how to order results. Possible values: <code>recently_added</code>.</td>
</tr>
</tbody>
</table>

**Request**

``` none
GET /api/v1/tools?start=START&items=ITEMS
```

or:

``` none
GET /api/v1/tools?order=recently_added
```

**Response**

The response will return the name of the tool, a type, description,
owner, source URL and URI. In addition, the total number of tools on
Supermarket API is shown, as well (if `start` is specified) the point at
which the list of returned tools began:

``` javascript
{
  "start": 0,
  "total": 56,
  "items": [
    {
      "tool_name": "Berkflow",
      "tool_type": "chef_tool",
      "tool_source_url": "https://github.com/reset/berkflow",
      "tool_description": "A Cookbook-Centric Deployment workflow tool",
      "tool_owner": "reset",
      "tool": "https://supermarket.chef.io/api/v1/tools/berkflow"
    },
    {
      "tool_name": "Berkshelf",
      "tool_type": "chef_tool",
      "tool_source_url": "https://github.com/berkshelf/berkshelf",
      "tool_description": "A Chef Cookbook manager",
      "tool_owner": "reset",
      "tool": "https://supermarket.chef.io/api/v1/tools/berkshelf"
    },
    {
      "tool_name": "Berkshelf-API",
      "tool_type": "chef_tool",
      "tool_source_url": "https://github.com/berkshelf/berkshelf-api",
      "tool_description": "Berkshelf dependency API server",
      "tool_owner": "reset",
      "tool": "https://supermarket.chef.io/api/v1/tools/berkshelf-api"
    }
  ]
}
```

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful. One or more tools were returned.</td>
</tr>
</tbody>
</table>

### /tools-search

The `tools` endpoint allows Chef Supermarket tools to be searched. This
endpoint has the following methods: `GET`.

#### GET

The `GET` method is used to get a list of tools that match a search
query. Use the `start` and `items` parameters to set limits on the
number of tools returned:

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Parameter</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>q</code></td>
<td>The search query used to identify a list of items on a Chef Infra Server. This option uses the same syntax as the <code>search</code> subcommand.</td>
</tr>
<tr class="even">
<td><code>start</code></td>
<td>The row at which return results begin.</td>
</tr>
<tr class="odd">
<td><code>items</code></td>
<td>The number of rows to be returned.</td>
</tr>
</tbody>
</table>

**Request**

``` none
GET /api/v1/tools-search?q=SEARCH_QUERY
```

or:

``` none
GET /api/v1/tools-search?q=SEARCH_QUERY&start=START&items=ITEMS
```

**Response**

The response will return a list of tools that match the search query.
Each returned data set will include the name of the tool, a type,
description, owner, source URL and URI. In addition, the total number of
tools that match the query on Supermarket API is shown, as well (if
`start` is specified) the point at which the list of returned tools
began:

``` javascript
{
  "start": 0,
  "total": 1,
  "items": [
    {
      "tool_name": "knife-rhn",
      "tool_type": "knife_plugin",
      "tool_source_url": "https://github.com/bflad/knife-rhn",
      "tool_description": "Knife Plugin for Red Hat Network (RHN)",
      "tool_owner": "bflad",
      "tool": "https://supermarket.chef.io/api/v1/tools/knife-rhn"
    }
  ]
}
```

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful. One or more tools were returned as a result of the search query.</td>
</tr>
</tbody>
</table>

### /tools/SLUG

The `tools/[SLUG]` endpoint allows a specific tool to be accessed. This
endpoint has the following methods: `GET`.

#### GET

The `GET` method is used to get the details for a tool.

This method has no parameters.

**Request**

``` none
GET /api/v1/tools/TOOL_SLUG
```

**Response**

The response will return details for a tool, including the name of the
tool, a type, description, owner, source URL and install instructions as
markdown:

``` javascript
{
  "name": "Berkshelf",
  "slug": "berkshelf",
  "type": "chef_tool",
  "source_url": "https://github.com/berkshelf/berkshelf",
  "description": "A Chef Cookbook manager",
  "instructions": "# Berkshelf\r\n[![Gem Version](https://img.shields.io/gem/v/berkshelf.svg)][gem]\r\n[![Build Status](https://img.shields.io/travis/berkshelf/berkshelf.svg)][travis]\r\n\r\n[gem]: https://rubygems.org/gems/berkshelf\r\n[travis]: https://travis-ci.org/berkshelf/berkshelf\r\n\r\nManage a Cookbook or an Application's Cookbook dependencies\r\n\r\n## Installation\r\n\r\nBerkshelf is now included as part of the [Chef-DK](http://chef.io/downloads/chef-dk). This is fastest, easiest, and the recommended installation method for getting up and running with Berkshelf.\r\n\r\n> note: You may need to uninstall the Berkshelf gem especially if you are using a Ruby version manager you may need to uninstall all Berkshelf gems from each Ruby installation.\r\n\r\n### From Rubygems\r\n\r\nIf you are a developer or you prefer to install from Rubygems, we've got you covered.\r\n\r\nAdd Berkshelf to your repository's `Gemfile`:\r\n\r\n```ruby\r\ngem 'berkshelf'\r\n```\r\n\r\nOr run it as a standalone:\r\n\r\n    gem install berkshelf\r\n\r\n## Usage\r\n\r\nSee [berkshelf.com](http://berkshelf.com) for up-to-date usage instructions.\r\n\r\n## Supported Platforms\r\n\r\nBerkshelf is tested on Ruby 1.9.3, 2.0, and 2.1.\r\n\r\nRuby 1.9 mode is required on all interpreters.\r\n\r\nRuby 1.9.1 and 1.9.2 are not officially supported. If you encounter problems, please upgrade to Ruby 2.0 or 1.9.3.\r\n\r\n## Configuration\r\n\r\nBerkshelf will search in specific locations for a configuration file. In order:\r\n\r\n    $PWD/.berkshelf/config.json\r\n    $PWD/berkshelf/config.json\r\n    $PWD/berkshelf-config.json\r\n    $PWD/config.json\r\n    ~/.berkshelf/config.json\r\n\r\nYou are encouraged to keep project-specific configuration in the `$PWD/.berkshelf` directory. A default configuration file is generated for you, but you can update the values to suit your needs.\r\n\r\n## Shell Completion\r\n\r\n- [Bash](https://github.com/berkshelf/berkshelf-bash-plugin)\r\n- [ZSH](https://github.com/berkshelf/berkshelf-zsh-plugin)\r\n\r\n## Plugins\r\n\r\nPlease see [Plugins page](https://github.com/berkshelf/berkshelf/blob/master/PLUGINS.md) for more information.\r\n\r\n## Getting Help\r\n\r\n* If you have an issue: report it on the [issue tracker](https://github.com/berkshelf/berkshelf/issues)\r\n* If you have a question: visit the #general or #berkshelf channel in the Chef Community Slack (http://community-slack.chef.io/)\r\n\r\n## Authors\r\n\r\n[The Berkshelf Core Team](https://github.com/berkshelf/berkshelf/wiki/Core-Team)\r\n\r\nThank you to all of our [Contributors](https://github.com/berkshelf/berkshelf/graphs/contributors), testers, and users.\r\n\r\nIf you'd like to contribute, please see our [contribution guidelines](https://github.com/berkshelf/berkshelf/blob/master/CONTRIBUTING.md) first.\r\n",
  "owner": "reset"
}
```

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful. The requested tool exists.</td>
</tr>
<tr class="even">
<td><p><code>400</code></p></td>
<td><p>The request was unsuccessful. The requested tool does not exist. For example:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode javascript"><code class="sourceCode javascript"><span id="cb1-1"><a href="#cb1-1"></a><span class="op">{</span></span>
<span id="cb1-2"><a href="#cb1-2"></a>   <span class="st">&quot;error_messages&quot;</span><span class="op">:</span></span>
<span id="cb1-3"><a href="#cb1-3"></a>   [<span class="st">&quot;Resource does not exist&quot;</span>]<span class="op">,</span></span>
<span id="cb1-4"><a href="#cb1-4"></a>   <span class="st">&quot;error_code&quot;</span><span class="op">:</span> <span class="st">&quot;NOT_FOUND&quot;</span></span>
<span id="cb1-5"><a href="#cb1-5"></a><span class="op">}</span></span></code></pre></div></td>
</tr>
</tbody>
</table>

### /universe

The universe is the known collection of cookbooks that have been
uploaded to Chef Supermarket. The universe is JSON data organized by
cookbook, then by cookbook version, and then by a dependency graph that
lists each dependency a cookbook version may have on other cookbooks or
cookbook versions.

Use the `/universe` endpoint to retrieve the known collection of
cookbooks, and then use it with Berkshelf.

The `/universe` endpoint has the following methods: `GET`.

#### GET

The `GET` method is used to retrieve the universe data.

This method has no parameters.

**Request**

``` none
GET /universe
```

**Response**

The response will return an embedded hash, with the name of each
cookbook as a top-level key. Each cookbook will list each version, along
with its location information and dependencies:

``` javascript
{
  "ffmpeg": {
    "0.1.0": {
      "location_path": "http://supermarket.chef.io/api/v1/cookbooks/ffmpeg/0.1.0/download"
      "location_type": "supermarket",
      "dependencies": {
        "git": ">= 0.0.0",
        "build-essential": ">= 0.0.0",
        "libvpx": "~> 0.1.1",
        "x264": "~> 0.1.1"
      },
    },
    "0.1.1": {
      "location_path": "http://supermarket.chef.io/api/v1/cookbooks/ffmpeg/0.1.1/download"
      "location_type": "supermarket",
      "dependencies": {
        "git": ">= 0.0.0",
        "build-essential": ">= 0.0.0",
        "libvpx": "~> 0.1.1",
        "x264": "~> 0.1.1"
      },
    },
   "pssh": {
    "0.1.0": {
      "location_path": "http://supermarket.chef.io/api/v1/cookbooks/pssh.1.0/download"
      "location_type": "supermarket",
      "dependencies": {},
    }
  }
}
```

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>200</code></td>
<td>OK. The request was successful. One (or more) cookbooks and associated cookbook version information was returned.</td>
</tr>
</tbody>
</table>

### /users/USERNAME

The `users/[USERNAME]` endpoint allows a specific Chef Supermarket user
to be accessed. This endpoint has the following methods: `GET`.

#### GET

The `GET` method is used to get the details for a user.

This method has no parameters.

**Request**

``` none
GET /api/v1/users/USERNAME
```

**Response**

The response will return details for a user, including their name, Chef
username, associated account details, and a list of cookbooks and tools
that are associated with the user. Cookbooks are grouped into three
categories: those that are owned by this user, those in which this user
has collaborated, and those that are followed.

``` javascript
{
  "username": "stevedanno",
  "name": "Steve Danno",
  "company": "Chef Software, Inc",
  "github": [
    "stevedanno"
  ],
  "twitter": "stevedanno",
  "irc": "stevedanno",
  "cookbooks": {
    "owns": {
      "bacon": "https://supermarket.chef.io/api/v1/cookbooks/bacon"
      "chef-sugar": "https://supermarket.chef.io/api/v1/cookbooks/chef-sugar"
    },
    "collaborates": {
      "build-essential": "https://supermarket.chef.io/api/v1/cookbooks/build-essential"
      "jenkins": "https://supermarket.chef.io/api/v1/cookbooks/jenkin"
    },
    "follows": {
      "bacon": "https://supermarket.chef.io/api/v1/cookbooks/bacon"
      "chef-sugar": "https://supermarket.chef.io/api/v1/cookbooks/chef-sugar"
    }
  },
  "tools": {
    "owns": {
      "bacon_tool": "https://supermarket.chef.io/api/v1/tools/bacon_tool"
    }
  }
}
```
