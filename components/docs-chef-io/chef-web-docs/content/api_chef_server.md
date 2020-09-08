+++
title = "Chef Infra Server API"
draft = false

aliases = ["/api_chef_server.html"]

[menu]
  [menu.infra]
    title = "Chef Infra Server API"
    identifier = "chef_infra/managing_chef_infra_server/api_chef_server.md Chef Infra Server API"
    parent = "chef_infra/managing_chef_infra_server"
    weight = 200
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/api_chef_server.md)

The Chef Infra Server API is a REST API that provides access to objects
on the Chef Infra Server, including nodes, environments, roles, users, organizations,
cookbooks (and cookbook versions), and is used to manage an API client list and
the associated RSA public key-pairs.

## Requirements

The Chef Infra Server API has the following requirements:

-   The `Accept` header must be set to `application/json`.
-   For `PUT` and `POST` requests, the `Content-Type` header must be set
    to `application/json`.
-   The `X-Chef-Version` header must be set to the version of the Chef
    Infra Server API that is being used.
-   A request must be signed by adding authentication headers.
    'Mixlib::Authentication` may be used to sign requests.
-   A request must be well-formatted. The easiest way to ensure a
    well-formatted request is to use the `Chef::ServerAPI` library.

## Authentication Headers

Authentication to the Chef Infra Server requires a specific set of
HTTP headers signed using a private key that is associated with the
client making the request. The request is authorized if the
Chef Infra Server can verify the signature using the public key. Only
authorized actions are allowed.

{{< note >}}

Most authentication requests made to the Chef Infra Server are
abstracted from the user. Such as when using knife or the Chef Infra
Server user interface. In some cases, such as when using the
`knife exec` subcommand, the authentication requests need to be made
more explicitly, but still in a way that does not require authentication
headers. In a few cases, such as when using arbitrary Ruby code,
a Chef Infra Server API client, or cURL, it may be necessary to include the
full authentication header as part of the request to the Chef Infra Server.

{{< /note >}}

### Required Headers

The following authentication headers are required:

<table>
<colgroup>
<col style="width: 24%" />
<col style="width: 75%" />
</colgroup>
<thead>
<tr class="header">
<th>Feature</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>Accept</code></td>
<td>The format in which response data from the Chef Infra Server is provided. This header must be set to <code>application/json</code>.</td>
</tr>
<tr class="even">
<td><code>Content-Type</code></td>
<td>The format in which data is sent to the Chef Infra Server. This header is required for <code>PUT</code> and <code>POST</code> requests and must be set to <code>application/json</code>.</td>
</tr>
<tr class="odd">
<td><code>Host</code></td>
<td>The host name (and port number) to which a request is sent. (Port number <code>80</code> does not need to be specified.) For example: <code>api.opscode.com</code> (which is the same as <code>api.opscode.com:80</code>) or <code>api.opscode.com:443</code>.</td>
</tr>
<tr class="even">
<td><code>Method</code></td>
<td>The method from the request.</td>
</tr>
<tr class="even">
<td><code>Path</code></td>
<td>Omit for Authentication Version 1. Specify for Authentication Version 1.3</td>
</tr>
<tr class="even">
<td><code>X-Chef-Version</code></td>
<td>The version of the Chef Infra Client executable from which a request is made. This header ensures that responses are in the correct format. For example: <code>12.0.2</code> or <code>11.16.x</code>.</td>
</tr>
<tr class="odd">
<td><code>X-Ops-Authorization-N</code></td>
<td>One (or more) 60 character segments that comprise the canonical header. A canonical header is signed with the private key used by the client machine from which the request is sent, and is also encoded using Base64. If more than one segment is required, each should be named sequentially, e.g. <code>X-Ops-Authorization-1</code>, <code>X-Ops-Authorization-2</code>, <code>X-Ops-Authorization-N</code>, where <code>N</code> represents the integer used by the last header that is part of the request.</td>
</tr>
<tr class="even">
<td><code>X-Ops-Content-Hash</code></td>
<td>For API Version 1. The result of the SHA-1 hash of the request body encoded using Base64. Base64 encoding should have line breaks every 60 characters.</br>
For API Version 1.3. The result of the SHA-256 hash of the request body encoded using Base64. Base64 encoding should have line breaks every 60 characters.</td>
</tr>
<tr class="odd">
<td><code>X-Ops-Server-API-Version</code></td>
<td>Use <code>X-Ops-Server-API-Version</code> to specify the version of the Chef Infra Server API. For example: <code>X-Ops-Server-API-Version: 1</code>. <code>X-Ops-Server-API-Version: 0</code> is supported for use with Chef Infra Server version 12, but will be deprecated as part of the next major release.</td>
</tr>
<tr class="even">
<td><code>X-Ops-Sign</code></td>
<td>Set this header to the following value: <code>algorithm=sha1,version=1.0</code> or <code>version=1.3</code>.</td>
</tr>
<tr class="odd">
<td><code>X-Ops-Timestamp</code></td>
<td>The timestamp, in ISO-8601 format and with UTC indicated by a trailing <code>Z</code> and separated by the character <code>T</code>. For example: <code>2013-03-10T14:14:44Z</code>.</td>
</tr>
<tr class="even">
<td><code>X-Ops-UserId</code></td>
<td>The name of the API client whose private key will be used to create the authorization header.</td>
</tr>
</tbody>
</table>

{{< note >}}

Use `X-Ops-Server-API-Info` to identify the version of the Chef Infra
Server API.

{{< /note >}}

#### Canonical Header Format 1.0 using SHA-1

The signed headers are encrypted using the OpenSSL RSA_private_encrypt
method and encoded in Base64.  The signed headers are used to create
one or more X-Ops-Authorization-N headers of 60 character segments.
The canonical header should be created by concatenating the following
headers, encrypting and encoding:

``` none
Method:HTTP_METHOD
Hashed Path:HASHED_PATH
X-Ops-Content-Hash:HASHED_BODY
X-Ops-Timestamp:TIME
X-Ops-UserId:USERID
```

where:

-   `HTTP_METHOD` is the method used in the API request (`GET`, `POST`,
    and so on)
-   `HASHED_PATH` is the path of the request:
    `/organizations/NAME/name_of_endpoint`. The `HASHED_PATH` must be
    hashed using SHA-1 and encoded using Base64, must not have repeated
    forward slashes (`/`), must not end in a forward slash (unless the
    path is `/`), and must not include a query string.
-   `X-Ops-Content-Hash` is the Base64 encoded SHA256 hash of the json body of the request.
-   `X-Ops-Timestamp` UTC time in RFC3339 format. 
-   `X-Ops-UserId` is the plain text client or user name.

The Chef Infra Server decrypts this header and ensures its content
matches the content of the non-encrypted headers that were in the
request. The timestamp of the message is checked to ensure the request
was received within a reasonable amount of time. One approach generating
the signed headers is to use
[mixlib-authentication](https://github.com/chef/mixlib-authentication),
which is a class-based header signing authentication object similar to
the one used by Chef Infra Client.

##### Example

The following example shows an authentication request:

``` none
GET /organizations/NAME/nodes HTTP/1.1
  Accept: application/json
  Accept-Encoding: gzip;q=1.0,deflate;q=0.6,identity;q=0.3
  Host: api.opscode.com:443
  User-Agent: Chef Knife/12.0.2 (ruby-2.1.1-p320; ohai-8.0.0; x86_64-darwin12.0.2; +http://chef.io)
  X-Chef-Version: 12.0.2
  X-Ops-Authorization-1: BE3NnBritishaf3ifuwLSPCCYasdfXaRN5oZb4c6hbW0aefI
  X-Ops-Authorization-2: sL4j1qtEZzi/2WeF67UuytdsdfgbOc5CjgECQwqrym9gCUON
  X-Ops-Authorization-3: yf0p7PrLRCNasdfaHhQ2LWSea+kTcu0dkasdfvaTghfCDC57
  X-Ops-Authorization-4: 155i+ZlthfasfasdffukusbIUGBKUYFjhbvcds3k0i0gqs+V
  X-Ops-Authorization-5: /sLcR7JjQky7sdafIHNfsBQrISktNPower1236hbFIayFBx3
  X-Ops-Authorization-6: nodilAGMb166@haC/fttwlWQ2N1LasdqqGomRedtyhSqXA==
  X-Ops-Content-Hash: 2jmj7l5rfasfgSw0ygaVb/vlWAghYkK/YBwk=
  X-Ops-Server-API-Info: 1
  X-Ops-Sign: algorithm=sha1;version=1.0;
  X-Ops-Userid: user_id
  X-Ops-Timestamp: 2014-12-12T17:13:28Z
```

#### Canonical Header Format 1.3 using SHA-256

Chef Infra Server versions 12.4.0 and above support signing protocol version
1.3, which adds support for SHA-256 algorithms. It can be enabled on
Chef Infra Client via the `client.rb` file:

``` ruby
authentication_protocol_version = '1.3'
```

And for Chef's knife CLI via `config.rb`:

``` ruby
knife[:authentication_protocol_version] = '1.3'
```

To create the signed headers for direct use. Gather the following 
headers in the order listed, convert the signature headers to a concatenated string,
sign and Base64 encode the result. The concatenation of signature headers is
signed using the client RSA private key, with SHA-256 hashing and PKCS1v15 padding.
Chop the Base64 encoded value into 60 character chunks and create
X-Ops-Authorization-N headers with the chunks.

``` none
Method:HTTP_METHOD
Path:PATH
X-Ops-Content-Hash:HASHED_BODY
X-Ops-Sign
X-Ops-Timestamp:TIME
X-Ops-UserId:USERID
X-Ops-Server-API-Version
```

where:

-   `HTTP_METHOD` is the method used in the API request (`GET`, `POST`, ...)
-   `PATH` is the path of the request: `/organizations/NAME/name_of_endpoint`.
    The value must not have repeated forward slashes (`/`), must not end
    in a forward slash (unless the path is `/`), and must not include a query string.
-   `X-Ops-Content-Hash` is the Base64 encoded SHA256 hash of the json body of the request.
-   `X-Ops-Sign` has the value "version=1.3".
-   `X-Ops-Timestamp` UTC time in RFC3339 format. 
-   `X-Ops-UserId` is the plain text client or user name.
-   `X-Ops-Server-API-Version` is the numeric value of the Chef Infra Server API.

##### Example

The following example shows an authentication request:

``` none
GET /organizations/NAME/nodes HTTP/1.1
  Accept: application/json
  Accept-Encoding: gzip;q=1.0,deflate;q=0.6,identity;q=0.3
  Host: api.opscode.com:443
  Method: GET
  Path: /organizations/NAME/nodes
  User-Agent: Chef Knife/12.0.2 (ruby-2.1.1-p320; ohai-8.0.0; x86_64-darwin12.0.2; +http://chef.io)
  X-Chef-Version: 14.0.0
  X-Ops-Content-Hash: 2jmj7l5rfasfgSw0ygaVb/vlWAghYkK/YBwk=
  X-Ops-Authorization-1: BE3NnBritishaf3ifuwLSPCCYasdfXaRN5oZb4c6hbW0aefI
  X-Ops-Authorization-2: sL4j1qtEZzi/2WeF67UuytdsdfgbOc5CjgECQwqrym9gCUON
  X-Ops-Authorization-3: yf0p7PrLRCNasdfaHhQ2LWSea+kTcu0dkasdfvaTghfCDC57
  X-Ops-Authorization-4: 155i+ZlthfasfasdffukusbIUGBKUYFjhbvcds3k0i0gqs+V
  X-Ops-Authorization-5: /sLcR7JjQky7sdafIHNfsBQrISktNPower1236hbFIayFBx3
  X-Ops-Authorization-6: nodilAGMb166@haC/fttwlWQ2N1LasdqqGomRedtyhSqXA==
  X-Ops-Server-API-Info: 1
  X-Ops-Sign: version=1.3;
  X-Ops-Timestamp: 2014-12-12T17:13:28Z
  X-Ops-Userid: user_id
```

### Knife API Requests

{{% plugin_knife_summary %}}

{{% plugin_knife_using_authenticated_requests %}}

## Global Endpoints

A global endpoint may be used to access all of the organizations on the
Chef Infra Server.

### /authenticate_user

The `/authenticate_user` endpoint has the following methods: `POST`.

#### POST

The `POST` method is used to authenticate a user. This endpoint is used
by the Chef Identity Service to authenticate users of Chef Supermarket
to the Chef Infra Server.

This method has no parameters.

**Request**

``` none
POST /authenticate_user
```

with a request body similar to:

``` javascript
{
  "username" : "grantmc",
  "password" : "p@ssw0rd"
}
```

**Response**

This method has no response body.

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, password, and that the correct key was used to sign the request.</td>
</tr>
</tbody>
</table>

### /license

{{< note >}}

This endpoint is used for information purposes only and to trigger a
notification in the Chef management console about the number of licenses
owned vs. the number of licenses that should be owned. No other action
is taken and the functionality and behavior of the Chef Infra Server and
any added component does not change.

{{< /note >}}

The `/license` endpoint has the following methods: `GET`.

#### GET

The `GET` method is used to get license information for the Chef Infra
Server.

This method has no parameters.

**Request**

``` none
GET /license
```

This method has no request body.

**Response**

The response is similar to:

``` javascript
{
  "limit_exceeded": false,
  "node_license": 25,
  "node_count": 12,
  "upgrade_url": "http://www.chef.io/contact/on-premises-simple"
}
```

When `node_count` is greater than `node_license`, then `limit_exceeded`
is `true` and the Chef management console will display a notification
about this status. The way to resolve this is to visit the upgrade URL,
add the appropriate number of licenses, and then update the
configuration settings appropriately.

The chef-server.rb file contains settings that can be used to edit the
number of nodes that are under license:

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
</colgroup>
<thead>
<tr class="header">
<th>Setting</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>license['nodes']</code></td>
<td>The number of licensed nodes. Default value: <code>25</code>.</td>
</tr>
<tr class="even">
<td><code>license['upgrade_url']</code></td>
<td>The URL to visit for more information about how to update the number of nodes licensed for an organization. Default value: <code>"http://www.chef.io/contact/on-premises-simple"</code>.</td>
</tr>
</tbody>
</table>

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
</tbody>
</table>

### /organizations

The Chef Infra Server may contain multiple organizations.

The `/organizations` endpoint has the following methods: `GET` and
`POST`.

{{< warning >}}

This endpoint may only be accessed by the `pivotal` user, which is
created as part of the installation process for the Chef Infra Server.
(See the "Query for Users and Orgs" example below for an example of how
to access this endpoint with the `pivotal` user.)

{{< /warning >}}

#### GET

The `GET` method is used to get a list of organizations on the Chef
Infra Server.

**Request**

``` none
GET /organizations
```

**Response**

The response is similar to:

``` none
{
  "org_name1": https://url/for/org_name1",
  "org_name2": https://url/for/org_name2"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
</tbody>
</table>

#### POST

The `POST` method is used to create an organization on the Chef Infra
Server.

This method has no parameters.

**Request**

``` none
POST /organizations
```

with a request body similar to:

``` javascript
{
  "name": "org_name1",
  "full_name": "Org_name1 Full Name"
}
```

where:

-   `name` must begin with a lower-case letter or digit, may only
    contain lower-case letters, digits, hyphens, and underscores, and
    must be between 1 and 255 characters. For example: `chef`.
-   `full_name` must begin with a non-white space character and must be
    between 1 and 1023 characters. For example: `Chef Software, Inc.`.

{{< note >}}

An organization isn't usable until a user that belongs to the `admins`
group is associated with the organization.

{{< /note >}}

**Response**

The response is similar to:

``` javascript
{
  "clientname": "org_name1-validator",
  "private_key": "-----BEGIN RSA PRIVATE KEY----- MIIEpQIBAAKCAQEAx2uyX ...",
  "uri": "https://url/for/org_name1"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>201</code></td>
<td>Created. The request was successful. The organization was created.</td>
</tr>
<tr class="even">
<td><code>400</code></td>
<td>Bad request. The contents of the request are not formatted correctly.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>409</code></td>
<td>Conflict. The organization already exists.</td>
</tr>
</tbody>
</table>

### /organizations/NAME

An organization is a single instance of a Chef Infra Server, including
all of the nodes that are managed by that Chef Infra Server and each of
the workstations that will run knife and access the Chef Infra Server
using the Chef Infra Server API.

The `/organizations/NAME` endpoint has the following methods: `DELETE`,
`GET`, and `PUT`.

#### DELETE

The `DELETE` method is used to delete an organization.

This method has no parameters.

**Request**

``` none
DELETE /organizations/NAME
```

**Response**

The response is similar to:

``` javascript
{
  "name": "chef",
  "full_name": "Chef Software, Inc",
  "guid": "f980d1asdfda0331235s00ff36862"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
</tbody>
</table>

#### GET

The `GET` method is used to get the details for the named organization.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME
```

**Response**

The response is similar to:

``` none
{
  "name": "chef",
  "full_name": "Chef Software, Inc",
  "guid": "f980d1asdfda0331235s00ff36862"
     ...
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
</tbody>
</table>

#### PUT

The `PUT` method is used to update an organization definition.

This method has no parameters.

**Request**

``` none
PUT /organizations/NAME
```

with a request body similar to:

``` javascript
{
  "name": "chef",
  "full_name": "Chef Software, Inc"
}
```

**Response**

The response is similar to:

``` none
{
  "name": "chef",
  "full_name": "Chef Software, Inc",
  "guid": "f980d1asdfda0331235s00ff36862"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>400</code></td>
<td>Bad request. The contents of the request are not formatted correctly.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>410</code></td>
<td>Gone. Unable to update private key.</td>
</tr>
</tbody>
</table>

### /_stats

Use the `/_stats` endpoint to display statistics about connection pool usage inside Erchef, Postgresql, and the Erlang VM.
The `_stats` endpoint uses Basic Authorization instead of the X-Ops-Authorization scheme usually used to connect
to the Chef Infra Server. The default user used to query the `_stats` endpoint is `statsuser`.  The password for the
`statsuser` is available as the `opscode_erchef::stats_password` from the `chef-server-ctl show-service-credentials` command.

The `/_stats` endpoint has the following method: `GET`.

#### GET

The `GET` method is used to get the statistics.

This method has the following parameters:

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
<td><code>format=json</code></td>
<td>Return results as JSON.</td>
</tr>
<tr class="even">
<td><code>format=text</code></td>
<td>Return results as text.</td>
</tr>
</table>

**Request**

``` none
GET /_stats
```

This method has no parameters. This method has no request body.
The `/_stats` endpoint does not require authentication headers.

**Response**

The response body is similar to:

``` javascript
[
  {
    "name": "erlang_vm_time_correction",
    "type": "UNTYPED",
    "help": "1 if time correction is enabled, otherwise 0.",
    "metrics": [
      {
        "value": "1"
      }
    ]
  },
  {
    "name": "erlang_vm_thread_pool_size",
    "type": "GAUGE",
    "help": "The number of async threads in the async thread pool used for asynchronous driver calls.",
    "metrics": [
      {
        "value": "5"
      }
    ]
  },

  ...

  {
    "name": "pg_stat_seq_scan",
    "type": "COUNTER",
    "help": "Number of sequential scans initiated on all tables",
    "metrics": [
      {
        "value": "22147"
      }
    ]
  }
]

```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or password is not valid.</td>
</tr>
<tr class="odd">
<td><code>406</code></td>
<td>Not Acceptable. An invalid format was requested.</td>
</tr>
</tbody>
</table>

### /_status

Use the `/_status` endpoint to check the status of communications
between the front and back end servers. This endpoint is located at
`/_status` on the front end servers. The `/_status` endpoint does not
require authentication headers.

#### GET

The `GET` method is used to get the Chef Infra Server status details.

**Request**

``` none
GET /_status
```

This method has no parameters. This method has no request body.

**Response**

The response will return something like the following:

``` javascript
{
  "status": "pong",
  "upstreams":
    {
      "service_name": "pong",
      "service_name": "pong",
      ...
    }
  "keygen":
    {
      "keys": 10,
      ....
    }
 }
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>All communications are OK.</td>
</tr>
<tr class="even">
<td><p><code>500</code></p></td>
<td><p>One (or more) services are down. For example:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode javascript"><code class="sourceCode javascript"><span id="cb1-1"><a href="#cb1-1"></a><span class="op">{</span></span>
<span id="cb1-2"><a href="#cb1-2"></a>  <span class="st">&quot;status&quot;</span><span class="op">:</span><span class="st">&quot;fail&quot;</span><span class="op">,</span></span>
<span id="cb1-3"><a href="#cb1-3"></a>  <span class="st">&quot;upstreams&quot;</span><span class="op">:</span></span>
<span id="cb1-4"><a href="#cb1-4"></a>    <span class="op">{</span></span>
<span id="cb1-5"><a href="#cb1-5"></a>      <span class="st">&quot;service_name&quot;</span><span class="op">:</span> <span class="st">&quot;fail&quot;</span><span class="op">,</span></span>
<span id="cb1-6"><a href="#cb1-6"></a>      <span class="st">&quot;service_name&quot;</span><span class="op">:</span> <span class="st">&quot;pong&quot;</span><span class="op">,</span></span>
<span id="cb1-7"><a href="#cb1-7"></a>      ...</span>
<span id="cb1-8"><a href="#cb1-8"></a>    <span class="op">}</span></span>
<span id="cb1-9"><a href="#cb1-9"></a><span class="op">}</span></span></code></pre></div></td>
</tr>
</tbody>
</table>

### /users

A user is an individual account that is created to allow access to the
Chef Infra Server. For example:

-   A hosted Chef Infra Server account
-   The user that operates the workstation from which a Chef Infra
    Server will be managed

The `/users` endpoint has the following methods: `GET` and `POST`.

{{< warning >}}

This endpoint may only be accessed by the `pivotal` user, which is
created as part of the installation process for the Chef Infra Server.
(See the "Query for Users and Orgs" example below for an example of how
to access this endpoint with the `pivotal` user.)

{{< /warning >}}

{{< note >}}

This documentation for the `/users` endpoint is for version 1 of the Chef Infra Server API.  Version 0 of the API has some differences in the request body and in the results.

{{< /note >}}

#### GET

The `GET` method is used to get a list of users on the Chef Infra
Server.

This method has the following parameters:

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
<td><code>email=jane@chef.com</code></td>
<td>Filter the users returned based on their email id.</td>
</tr>
<tr class="even">
<td><code>external_authentication_uid=jane@chef.com</code></td>
<td>Filter the users returned based on their external login id.</td>
</tr>
<tr class="odd">
<td><code>verbose=true</code></td>
<td>Returns a user list with "email", "first_name", "last_name" fields. If this flag is set the email and external_authentication_uid parameters are ignored.</td>
</tr>
</tbody>
</table>

**Request**

``` none
GET /users
```

**Response**

The response is similar to:

``` none
{
  "user1": "https://chef.example/users/user1",
  "user2": "https://chef.example/users/user2"
}
```

The verbose response is similar to:

``` none
{
  "janechef": { "email": "jane.chef@user.com", "first_name": "jane", "last_name": "chef_user" },
  "yaelsmith": { "email": "yeal.chef@user.com", "first_name": "yeal", "last_name": "smith" }
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

**Optional Filtering**

Filtering on `/users` can be done with the
`external_authentication_uid`. This is to support SAML authentication.

As an example, to retrieve users whos `external_authentication_uid` is
`jane@doe.com`, you would do the following:

``` none
GET /users?external_authentication_uid=jane%40doe.com
```

*New in Chef Server 12.7.*

#### POST

The `POST` method is used to create a user on the Chef Infra Server.

This method has no parameters.

**Request**

``` none
POST /users
```

with a request body similar to:

``` javascript
{
  "username": "robert-forster",
  "display_name": "robert",
  "email": "robert@noreply.com",
  "first_name": "robert",
  "last_name": "forster",
  "middle_name": "",
  "password": "yeahpass",
  "create_key": true,
  "public_key": "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAoYyN0AIhUh7Fw1+gQtR+ \n0/HY3625IUlVheoUeUz3WnsTrUGSSS4fHvxUiCJlNni1sQvcJ0xC9Bw3iMz7YVFO\nWz5SeKmajqKEnNywN8/NByZhhlLdBxBX/UN04/7aHZMoZxrrjXGLcyjvXN3uxyCO\nyPY989pa68LJ9jXWyyfKjCYdztSFcRuwF7tWgqnlsc8pve/UaWamNOTXQnyrQ6Dp\ndn+1jiNbEJIdxiza7DJMH/9/i/mLIDEFCLRPQ3RqW4T8QrSbkyzPO/iwaHl9U196\n06Ajv1RNnfyHnBXIM+I5mxJRyJCyDFo/MACc5AgO6M0a7sJ/sdX+WccgcHEVbPAl\n1wIDAQAB \n-----END PUBLIC KEY-----\n\n"
}
```

where:

-   `username` must begin with a lower-case letter or digit, may only
    contain lower-case letters, digits, hyphens, and underscores. For
    example: `chef`.
    `username` is required to be present and have a valid value. A valid
    username is a dot separated list of elements matching
    `` a-z0-9!#$%&'*+/=?^_`{|}~- ``.
-   `display_name` is required to be present.
-   `email` is required to be present and have a valid value. The email
    validation doesn't allow for all unicode characters.
-   Either `external_authentication_uid` or `password` are required to
    be present and have a value.
-   During the POST, the `public_key` value will be broken out and
    resubmitted to the keys portion of the API in the latest Chef Infra
    Server versions.
-   Only one of the keys, `create_key` or `public_key`, may be specified.  If `create_key` is specified, a default private key is generated and returned.

**Response**

The response is similar to:

``` javascript
{
  "uri": "https://chef.example/users/robert-forster",
  "chef_key": {
    "name": "default",
    "public_key": "-----BEGIN RSA PUBLIC KEY...",
    "expiration_date": "infinity",
    "uri": "https://chef.example/users/robert-forster/keys/default",
    "private_key": "-----BEGIN RSA PRIVATE KEY..."
  }
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>201</code></td>
<td>OK. The user was created.</td>
</tr>
<tr class="even">
<td><code>400</code></td>
<td>Bad request. The contents of the request are not formatted correctly.</td>
</tr>
<tr class="odd">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="even">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="odd">
<td><code>409</code></td>
<td>Conflict. The object already exists.</td>
</tr>
<tr class="even">
<td><code>413</code></td>
<td>Request entity too large. A request may not be larger than 1000000 bytes.</td>
</tr>
</tbody>
</table>

### /users/NAME

The `/users/USER_NAME` endpoint has the following methods: `DELETE`,
`GET`, and `PUT`.

{{< note >}}

This documentation for the `/users/NAME` endpoint is for version 1 of the Chef Infra Server API.  Version 0 of the API has some differences in the request body and in the results.

{{< /note >}}

#### DELETE

The `DELETE` method is used to delete a user.

This method has no parameters.

**Request**

``` none
DELETE /users/USER_NAME
```

**Response**

The response is similar to:

``` javascript
{
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### GET

The `GET` method is used to return the details for a user.

This method has no parameters.

**Request**

``` none
GET /users/USER_NAME
```

**Response**

The response is similar to:

``` javascript
{
  "username": "robert-forster",
  "display_name": "robert",
  "email": "robert@noreply.com",
  "first_name": "robert",
  "last_name": "forster"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### PUT

The `PUT` method is used to update a specific user. If values are not
specified for the `PUT` method, the Chef Infra Server will use the
existing values rather than assign default values.

{{< note >}}

`PUT` supports renames. If `PUT /users/foo` is requested with
`{ "username: "bar""}`, then it will rename `foo` to `bar` and all of the
content previously associated with `foo` will be associated with `bar`.

{{< /note >}}

{{< note >}}

As of 12.1.0, the `"public_key"`, `"private_key"`, and `"create_key"`
parameters in PUT requests to clients/users will cause a 400 response.

{{< /note >}}

This method has no parameters.

**Request**

``` none
PUT /users/NAME
```

with a request body similar to:

``` none
{
  "username":     "grant.mclennan",
  "display_name": "Grant McLennan",
  "email":        "grant@newlocation.com",
  "first_name":   "Grant",
  "last_name":    "McLennan",
  "middle_name":  "james",
  "public_key" : "-------- BEGIN PUBLIC KEY ----and a valid key here"
}
```

**Response**

The response is similar to:

``` javascript
{
  "uri": "https://chef.example/users/grant.mclennan",
  "chef_key": {
    "name": "default",
    "public_key": "-----BEGIN RSA PUBLIC KEY...",
    "expiration_date": "infinity",
    "uri": "https://chef.example/users/rober-forster/keys/default",
    "private_key": ""
  }
}
```

If a new private key was generated, both the private and public keys are
returned.

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>201</code></td>
<td>Created. The object was created. (This response code is only returned when the user is renamed.)</td>
</tr>
<tr class="odd">
<td><code>400</code></td>
<td>Invalid. Invalid or missing values. Otherwise malformed request.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
<tr class="odd">
<td><code>409</code></td>
<td>Conflict. This response code is only returned when a user is renamed, but a user already exists with that name.</td>
</tr>
<tr class="even">
<td><code>413</code></td>
<td>Request entity too large. A request may not be larger than 1000000 bytes.</td>
</tr>
</tbody>
</table>

### /users/USER/keys/

The `/users/USER/keys` endpoint has the following methods: `GET` and
`POST`. User keys are public RSA keys in the SSL `.pem` file
format and are used for authentication.  The Chef Infra Server
does not save private keys for users.

#### GET

The `GET` method is used to retrieve all of the named user's key
identifiers, associated URIs, and expiry states.

This method has no parameters.

**Request**

``` none
GET /users/USER/keys/
```

**Response**

The response is similar to:

``` javascript
[
  {
    "name" : "default",
    "uri" : "https://chef.example/users/USER/keys/default",
    "expired" : false
  },
  {
    "name" : "key1",
    "uri" : "https://chef.example/users/USER/keys/key1",
    "expired" : false
  }
]
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### POST

The `POST` method is used to add a key for the specified user.

This method has no parameters.

**Request**

``` none
POST /users/USER/keys/
```

with a request body similar to:

``` javascript
{
  "name" : "key1",
  "public_key" : "-------- BEGIN PUBLIC KEY ----and a valid key here",
  "expiration_date" : "infinity"
}
```

**Response**

The response is similar to:

``` javascript
{
  "name" : "key1",
  "uri" : "https://chapi_chef_server.mdef.example/users/user1/keys/key1",
  "expired": false
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>201</code></td>
<td>Created. The object was created.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

### /users/USER/keys/KEY

The `/users/USER/keys/KEY` endpoint has the following methods: `DELETE`,
`GET`, and `PUT`.

#### DELETE

The `DELETE` method is used to delete the specified key for the
specified user.

This method has no parameters.

**Request**

``` none
DELETE /users/USER/keys/KEY
```

**Response**

The response returns the information about the deleted key and is
similar to:

``` javascript
{
  "name" : "default",
  "public_key" : "-------- BEGIN PUBLIC KEY --------- ...",
  "expiration_date" : "2020-12-31T00:00:00Z"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### GET

The `GET` method is used to return details for a specific key for a
specific user.

This method has no parameters.

**Request**

``` none
GET /users/USER/keys/KEY
```

**Response**

The response is similar to:

``` javascript
{
  "name" : "default",
  "public_key" : "-------- BEGIN PUBLIC KEY --------- ...",
  "expiration_date" : "2020-12-31T00:00:00Z"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### PUT

The `PUT` method is used to update one or more properties for a specific
key for a specific user.

This method has no parameters.

**Request**

``` none
PUT /users/USER/keys/KEY
```

with a request body similar to:

``` javascript
{
  "name" : "new_key_name",
  "public_key" : "-------- BEGIN PUBLIC KEY ----and a valid key here",
  "expiration_date" : "2020-12-31T00:00:00Z"
}
```

**Response**

The response contains the updated inforamtion for the key, and is
similar to:

``` javascript
{
  "name" : "new_key_name",
  "public_key" : "-------- BEGIN PUBLIC KEY --------- ...",
  "expiration_date" : "2020-12-31T00:00:00Z"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>201</code></td>
<td>Created. The object was created.</td>
</tr>
<tr class="odd">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="even">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="odd">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

## Organization Endpoints

Each organization-specific authentication request must include
`/organizations/NAME` as part of the name for the endpoint. For example,
the full endpoint for getting a list of roles:

``` none
GET /organizations/NAME/roles
```

where `ORG_NAME` is the name of the organization.

### /association_requests

Users may be invited to join organizations via the web user interface in
the Chef management console or via the `POST` endpoint in the Chef Infra
Server API.

The `/association_requests` endpoint has the following methods:
`DELETE`, `GET`, and `POST`.

#### DELETE

The `DELETE` method is used to delete a pending invitation.

This method has no parameters.

**Request**

``` none
DELETE /organizations/NAME/association_requests/ID
```

This method has no request body.

**Response**

The response is similar to:

``` javascript
{
  "id":      "79b9382ab70e962907cee1747f9969a4",
  "orgname": "testorg",
  "username" "janedoe"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### GET

The `GET` method is used to get a list of pending invitations.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/association_requests
```

This method has no request body.

**Response**

The response returns a dictionary similar to:

``` javascript
[
  {
    "id": "79b9382ab70e962907cee1747f9969a4",
    "username": "marygupta"
  },
  {
    "id": "24t1432uf33x799382abb7096g8190b5",
    "username": "johnirving"
  }
]
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
</tbody>
</table>

#### POST

The `POST` method is used to create an invitation.

This method has no parameters.

**Request**

``` javascript
{
 "user": "billysmith"
}

POST /organizations/NAME/association_requests
```

**Response**

The response is similar to:

``` javascript
{
  "uri": "https://chef.example/organizations/test/association_requests/79b9382ab70e962907cee1747f9969a4",
  "organization_user": {
    "username": "authorizeduser"
  },
  "organization": {
    "name": "test"
  },
  "user": {
    "email": "sallyjane@domain.org",
    "first_name": "sally"
  }
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>201</code></td>
<td>OK. An invitation was created.</td>
</tr>
<tr class="even">
<td><code>400</code></td>
<td>Bad request. The contents of the request are not formatted correctly.</td>
</tr>
<tr class="odd">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="even">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="odd">
<td><code>404</code></td>
<td>Not found. The invited user does not exist.</td>
</tr>
<tr class="even">
<td><code>409</code></td>
<td>Conflict. The object already exists.</td>
</tr>
</tbody>
</table>

### /clients

Use the `/clients` endpoint to manage clients and their associated RSA
key-pairs. The `/clients` endpoint has the following methods: `GET` and `POST`.

{{< note >}}

The clients should be managed using knife as opposed to the Chef Infra Server API.
The interactions between clients, nodes and acls are tricky.

{{< /note >}}

#### GET

The `GET` method is used to return a client list on the Chef Infra
Server, including clients for nodes that have been registered with the Chef Infra
Server, the chef-validator clients, and the chef-server-webui clients
for the entire organization.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/clients
```

This method has no request body.

**Response**

The response is similar to:

``` javascript
{
  "org1-validator" : "https://chef.example/orgaizations/org1/clients/org1-validator",
  "client1" : "https://chef.example/orgaizations/org1/clients/client1"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
</tbody>
</table>

#### POST

The `POST` method is used to create a new API client.

{{< note >}}

As of 12.1.0, the `"admin"` parameter is no longer supported in
client/user creation and support. If used in the `POST` or `PUT` of a
client or user, the `"admin"` parameter is ignored.

{{< /note >}}

This method has no parameters.

**Request**

``` none
POST /organizations/NAME/clients
```

with a request body similar to:

``` javascript
{
  "name": "name_of_API_client",
  "clientname": "name_of_API_client",
  "validator": true,
  "create_key": true
}
```

where `name_of_API_client` is the name of the API client to be created
and `admin` indicates whether the API client will be run as an admin API
client. Either name or clientname needs to be specified.

**Response**

The response is similar to:

``` javascript
{
  "uri": "https://chef.example/orgaizations/org1/clients/client1",
  "chef_key": {
    "name": "default",
    "expiration_date": "infinity",
    "private_key": "-----BEGIN RSA PRIVATE KEY----- ...",
    "public_key": "-----BEGIN PUBLIC KEY----- ... ",
    "uri": "https://chef.example/orgaizations/org1/clients/client1/keys/default"
}
```

Store the private key in a safe place. It will be required later (along
with the client name) to access the Chef Infra Server when using the
Chef Infra Server API.

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>201</code></td>
<td>Created. The client was created.</td>
</tr>
<tr class="even">
<td><code>400</code></td>
<td>Bad request. The contents of the request are not formatted correctly.</td>
</tr>
<tr class="odd">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="even">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="odd">
<td><code>409</code></td>
<td>Conflict. The object already exists.</td>
</tr>
<tr class="even">
<td><code>413</code></td>
<td>Request entity too large. A request may not be larger than 1000000 bytes.</td>
</tr>
</tbody>
</table>

### /clients/NAME

The `/clients/NAME` endpoint is used to manage a specific client.
This endpoint has the following methods: `DELETE`, `GET`, and `PUT`.

#### DELETE

The `DELETE` method is used to remove a specific client.

This method has no parameters.

**Request**

``` none
DELETE /organizations/NAME/clients/NAME
```

This method has no request body.

**Response**

The response has no body.

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### GET

The `GET` method is used to return a specific API client.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/clients/NAME
```

This method has no request body.

**Response**

The response is similar to:

``` javascript
{
  "name": "user1",
  "clientname": "user1",
  "orgname": "test",
  "json_class": "Chef::ApiClient",
  "chef_type": "client",
  "validator": "false"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### PUT

The `PUT` method is used to update a specific client. If values are
not specified for the `PUT` method, the Chef Infra Server will use the
existing values rather than assign default values.

{{< note >}}

`PUT` supports renames. If `PUT /client/foo` is requested with
`{ "name: "bar""}`, then it will rename `foo` to `bar` and all of the
content previously associated with `foo` will be associated with `bar`.

{{< /note >}}

{{< note >}}

As of 12.1.0, the `"admin"` parameter is no longer supported in
client/user creation and support. If used in the `POST` or `PUT` of a
client or user, then it is ignored.

{{< /note >}}

{{< note >}}

As of 12.1.0, including `"public_key"`, `"private_key"`, or
`"create_key"` in PUT requests to clients/users will cause a 400
response.

{{< /note >}}

{{< note >}}

`"name"` and `"clientname"` are not independent values. Making a PUT
request with different values will return a 400 error. Either name
may be specified to set both values.

{{< /note >}}


**Request**

``` none
PUT /organizations/NAME/clients/NAME
```

with a request body similar to:

``` javascript
{
  "name": "monkeypants",
  "validator": false
}
```

**Response**

The response is similar to:

``` javascript
{
  "name": "monkeypants",
  "clientname": "monkeypants",
  "validator": true,
  "json_class":"Chef::ApiClient",
  "chef_type":"client"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>201</code></td>
<td>Created. The client was updated. (This response code is only returned when the client is renamed.)</td>
</tr>
<tr class="odd">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="even">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="odd">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
<tr class="even">
<td><code>409</code></td>
<td>Conflict. This response code is only returned when a client is renamed, but a client already exists with the new name.</td>
</tr>
<tr class="odd">
<td><code>413</code></td>
<td>Request entity too large. A request may not be larger than 1000000 bytes.</td>
</tr>
</tbody>
</table>

### /clients/CLIENT/keys/

The `/clients/CLIENT/keys` endpoint has the following methods: `GET` and
`POST`.

#### GET

The `GET` method is used to retrieve all of the named client's key
identifiers, associated URIs, and expiry states.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/clients/CLIENT/keys
```

This method has no request body.

**Response**

The response is similar to:

``` javascript
[
  {
     "name": "default",
     "uri": "https://chef.example/organizations/example/clients/client1/keys/default",
     "expired": false
  },
  {
     "name": "key1",
     "uri": "https://chef.example/organizations/example/clients/client1/keys/key1",
     "expired": true
  }
]
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### POST

The `POST` method is used to add a key for the specified client.

This method has no parameters.

**Request**

``` none
POST /organizations/NAME/clients/CLIENT/keys
```

with a request body similar to:

``` javascript
{
  "name": "key1",
  "public_key": "-------- BEGIN PUBLIC KEY ----and a valid key here",
  "expiration_date": "infinity"
}
```

**Response**

The response is similar to:

``` javascript
{
  "uri": "https://chef.example/organizations/example/clients/client1/keys/key1"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>201</code></td>
<td>Created. The object was created.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

### /clients/CLIENT/keys/KEY

The `/clients/CLIENT/keys/KEY` endpoint has the following methods:
`DELETE`, `GET`, and `PUT`.

#### DELETE

The `DELETE` method is used to delete the specified key for the
specified client.

This method has no parameters.

**Request**

``` none
DELETE /organizations/NAME/clients/CLIENT/keys/KEY
```

This method has no request body.

**Response**

The response returns the information about the deleted key and is
similar to:

``` javascript
{
  "name": "default",
  "public_key": "-------- BEGIN PUBLIC KEY --------- ...",
  "expiration_date": "2020-12-31T00:00:00Z"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### GET

The `GET` method is used to return details for a specific key for a
specific client.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/clients/CLIENT/keys/KEY
```

This method has no request body.

**Response**

The response is similar to:

``` javascript
{
  "name" : "default",
  "public_key" : "-------- BEGIN PUBLIC KEY --------- ...",
  "expiration_date" : "2020-12-31T00:00:00Z"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### PUT

The `PUT` method is used to update one or more properties for a specific
key for a specific client.

This method has no parameters.

**Request**

``` none
PUT /organizations/NAME/clients/CLIENT/keys/KEY
```

with a request body similar to:

``` javascript
{
  "name": "new_key_name",
  "public_key": "-------- BEGIN PUBLIC KEY ----and a valid key here",
  "expiration_date": "2020-12-31T00:00:00Z"
}
```

**Response**

The response contains the updated information for the key and is
similar to:

``` javascript
{
  "name": "new_key_name",
  "public_key": "-------- BEGIN PUBLIC KEY --------- ...",
  "expiration_date": "2020-12-31T00:00:00Z"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>201</code></td>
<td>Created. The object was created.</td>
</tr>
<tr class="odd">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="even">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="odd">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

### /containers

The `/containers` endpoint has the following methods: `GET`, `POST`.

#### GET

The `GET` method is used to get a list of containers.

Note: The `/containers` endpoint is not useful outside of the Chef Infra Server code.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/containers
```

This method does not use a request body.

**Response**

The response is similar to:

``` javascript
{
  "clients": "https://chef.example/organizations/example/containers/clients",
  "containers": "https://chef.example/organizations/example/containers/containers",
  "cookbooks": "https://chef.example/organizations/example/containers/cookbooks",
  "data": "https://chef.example/organizations/example/containers/data",
  "environments": "https://chef.example/organizations/example/containers/environments",
  "groups": "https://chef.example/organizations/example/containers/groups",
  "nodes": "https://chef.example/organizations/example/containers/nodes",
  "roles": "https://chef.example/organizations/example/containers/roles",
  "sandboxes": "https://chef.example/organizations/example/containers/sandboxes"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### POST

The `POST` method is used to create a container.

Note: Using the `POST` method of the `/containers` endpoint may have unexpected effects and is likely to break your system. Use of this method is not supported.

This method has no parameters.

**Request**

``` none
POST /organizations/NAME
```

This method has a request body similar to:

``` javascript
{
  "containername": "mycontainer",
  "containerpath": "mycontainer"
}
```

**Response**

The response is similar to:

``` javascript
{
  "uri"": "https://chef.example/organizations/test/containers/mycontainer"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>201</code></td>
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

### /containers/NAME

#### DELETE

The `DELETE` method is used to remove a container.

The `/containers/Name` endpoint has the following methods: `DELETE`, `GET`.

Note: Using the `DELETE` method of the `/containers/NAME` endpoint may have unexpected effects and is likely to break your system. Use of this method is not supported. 

This method has no parameters.

**Request**

``` none
DELETE /organizations/NAME/containers/NAME
```

This method does not use a request body.

**Response**

The response does not return response body.

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### GET

The `GET` method is used to get a container.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/containers/NAME
```

This method does not use a request body.

**Response**

The response is similar to:

``` javascript
{
  "containername"": "mycontainer",
  "containerpath"": "mycontainer"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

### /cookbook_artifacts

Cookbook artifacts are specific versions of cookbooks that were specified by a Policyfile applied to a node.

The `/organization/NAME/cookbook_artifacts` endpoint has the following methods: `GET`.

#### GET

The `GET` method is used to return a hash of all cookbook artifacts and their versions.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/cookbook_artifacts
```

This method has no request body.

**Response**

The response is similar to:

``` javascript
{
  "oc-influxdb": {
    "url": "https://chef.example/organizations/example-org/cookbook_artifacts/oc-influxdb",
    "versions": [
      {
        "url": "https://chef.example/organizations/example-org/cookbook_artifacts/oc-influxdb/9634a5d998b02ff069761f6e1309a41572d0f858",
        "identifier": "9634a5d998b02ff069761f6e1309a41572d0f858"
      },
      {
        "url": "https://chef.example/organizations/example-org/cookbook_artifacts/oc-influxdb/d774c9bb079f21b64c34275ecd4b371e0cae71a1",
        "identifier": "d774c9bb079f21b64c34275ecd4b371e0cae71a1"
      }
    ]
  },
  "rabbitmq": {
    "url": "https://chef.example/organizations/example-org/cookbook_artifacts/rabbitmq",
    "versions": [
      {
        "url": "https://chef.example/organizations/example-org/cookbook_artifacts/rabbitmq/58035a5b41c005f3b5b98f22ccaed1a0d6161e22",
        "identifier": "58035a5b41c005f3b5b98f22ccaed1a0d6161e22"
      },
      {
        "url": "https://chef.example/organizations/example-org/cookbook_artifacts/rabbitmq/5c08f92cc01f94ee37d382c32023b137ee343a1e",
        "identifier": "5c08f92cc01f94ee37d382c32023b137ee343a1e"
      }
    ]
  }
}
```

 **Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
</tbody>
</table>

### /cookbook_artifacts/NAME

This endpoint lists versions of a named cookbook artifact.

The `/organization/NAME/cookbook_artifacts/NAME` endpoint has the following methods: `GET`.

#### GET

The `GET` method is used to return a hash of a single cookbook artifact and its versions.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/cookbook_artifacts/NAME
```

This method has no request body.

**Response**

The response is similar to:

``` javascript
{
  "rabbitmq": {
    "url": "https://chef.example/organizations/example-org/cookbook_artifacts/rabbitmq",
    "versions": [
      {
        "url": "https://chef.example/organizations/example-org/cookbook_artifacts/rabbitmq/0bd7539be0434e3355aff8ecccf4543ecf5c4be2",
        "identifier": "0bd7539be0434e3355aff8ecccf4543ecf5c4be2"
      },
      {
        "url": "https://chef.example/organizations/example-org/cookbook_artifacts/rabbitmq/0e1016d364685b87456c648136da04a2559821ec",
        "identifier": "0e1016d364685b87456c648136da04a2559821ec"
      }
    ]
  }
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

### /cookbook_artifacts/NAME/ID

The `/organization/NAME/cookbook_artifacts/NAME/ID` endpoint has the following methods: `DELETE`, `GET`, and `PUT`.

#### DELETE

The `DELETE` method is used to delete a single cookbook artifact version.

This method has no parameters.

**Request**

``` none
DELETE /organizations/NAME/cookbook_artifacts/NAME/ID
```

This method has no request body.

**Response**

The response contains the record of the deleted resource and is similar to:

``` javascript
{
  "version": "5.7.7",
  "name": "rabbitmq",
  "identifier": "f3cf8ea7d8bfc59e35ec541946e3e82cd4b73e74",
  "frozen?": false,
  "chef_type": "cookbook_version",
  "attributes": [
    {
      "name": "default.rb",
      "path": "attributes/default.rb",
      "checksum": "e5a530cca3898d8bd07604435dc5156e",
      "specificity": "default",
      "url": "https://chef.example/bookshelf/organization-9f69768696feedcd165633b8b475cc0b/checksum-e5a530cca3898d8bd07604435dc5156e"
    }
  ],
  "definitions": [
  ],
  "files": [
  ],
  "libraries": [
    {
      "name": "matchers.rb",
      "path": "libraries/matchers.rb",
      "checksum": "24c3f44c4d1d62300a56051f0069f639",
      "specificity": "default",
      "url": "https://chef.example/bookshelf/organization-9f69768696feedcd165633b8b475cc0b/checksum-24c3f44c4d1d62300a56051f0069f639"
    },
    {
      "name": "helpers.rb",
      "path": "libraries/helpers.rb",
      "checksum": "df65c4a7259fcb30c6f3f1305ebf7502",
      "specificity": "default",
      "url": "https://chef.example/bookshelf/organization-9f69768696feedcd165633b8b475cc0b/checksum-df65c4a7259fcb30c6f3f1305ebf7502"
    },
    {
      "name": "default.rb",
      "path": "libraries/default.rb",
      "checksum": "94292faac84ba797e720501700b30f74",
      "specificity": "default",
      "url": "https://chef.example/bookshelf/organization-9f69768696feedcd165633b8b475cc0b/checksum-94292faac84ba797e720501700b30f74"
    }
  ],
  "providers": [
    {
      "name": "user.rb",
      "path": "providers/user.rb",
      "checksum": "c31c9cc749f21962c825f983a6679d94",
      "specificity": "default",
      "url": "https://chef.example/bookshelf/organization-9f69768696feedcd165633b8b475cc0b/checksum-c31c9cc749f21962c825f983a6679d94"
    },
    {
      "name": "policy.rb",
      "path": "providers/policy.rb",
      "checksum": "746c8a3f248f5bbfa51f5d2ba60b6315",
      "specificity": "default",
      "url": "https://chef.example/bookshelf/organization-9f69768696feedcd165633b8b475cc0b/checksum-746c8a3f248f5bbfa51f5d2ba60b6315"
    }
  ],
  "recipes": [
    {
      "name": "default.rb",
      "path": "recipes/default.rb",
      "checksum": "99a9b404ff6038d6ac55a90ca68c347a",
      "specificity": "default",
      "url": "https://chef.example/bookshelf/organization-9f69768696feedcd165633b8b475cc0b/checksum-99a9b404ff6038d6ac55a90ca68c347a"
    },
    {
      "name": "cluster.rb",
      "path": "recipes/cluster.rb",
      "checksum": "fc0a86c1f858c9d37e11282efc9fe329",
      "specificity": "default",
      "url": "https://chef.example/bookshelf/organization-9f69768696feedcd165633b8b475cc0b/checksum-fc0a86c1f858c9d37e11282efc9fe329"
    }
  ],
  "resources": [
    {
      "name": "cluster.rb",
      "path": "resources/cluster.rb",
      "checksum": "85e74276e19bfdad581dce4f5c59f94a",
      "specificity": "default",
      "url": "https://chef.example/bookshelf/organization-9f69768696feedcd165633b8b475cc0b/checksum-85e74276e19bfdad581dce4f5c59f94a"
    }
  ],
  "root_files": [
    {
      "name": "metadata.rb",
      "path": "metadata.rb",
      "checksum": "36b395e758138a4295d1e3f9b3df5da9",
      "specificity": "default",
      "url": "https://chef.example/bookshelf/organization-9f69768696feedcd165633b8b475cc0b/checksum-36b395e758138a4295d1e3f9b3df5da9"
    },
    {
      "name": "README.md",
      "path": "README.md",
      "checksum": "99873670f0994642f5e6baade52c8020",
      "specificity": "default",
      "url": "https://chef.example/bookshelf/organization-9f69768696feedcd165633b8b475cc0b/checksum-99873670f0994642f5e6baade52c8020"
    }
  ],
  "templates": [
    {
      "name": "default.rabbitmq-server.erb",
      "path": "templates/default/default.rabbitmq-server.erb",
      "checksum": "077855f4dc37f7fb708976134d8b2551",
      "specificity": "default",
      "url": "https://chef.example/bookshelf/organization-9f69768696feedcd165633b8b475cc0b/checksum-077855f4dc37f7fb708976134d8b2551"
    },
    {
      "name": "90forceyes.erb",
      "path": "templates/default/90forceyes.erb",
      "checksum": "73cc571097cf77c74b4e7b5b680020c9",
      "specificity": "default",
      "url": "https://chef.example/bookshelf/organization-9f69768696feedcd165633b8b475cc0b/checksum-73cc571097cf77c74b4e7b5b680020c9"
    }
  ],
  "metadata": {
    "name": "rabbitmq",
    "description": "Installs and configures RabbitMQ server",
    "long_description": "",
    "maintainer": "Chef, Inc. and contributors",
    "maintainer_email": "mklishin@pivotal.io",
    "license": "Apache-2.0",
    "platforms": {
      "amazon": ">= 2.0",
      "centos": ">= 7.0",
      "debian": ">= 8.0",
      "opensuse": ">= 0.0.0",
      "opensuseleap": ">= 0.0.0",
      "oracle": ">= 0.0.0",
      "redhat": ">= 0.0.0",
      "scientific": ">= 0.0.0",
      "smartos": ">= 0.0.0",
      "suse": ">= 0.0.0",
      "ubuntu": ">= 14.04"
    },
    "dependencies": {
      "erlang": ">= 0.0.0",
      "yum-epel": ">= 0.0.0",
      "yum-erlang_solutions": ">= 0.0.0",
      "dpkg_autostart": ">= 0.0.0",
      "logrotate": ">= 0.0.0"
    },
    "providing": {
      "rabbitmq::cluster": ">= 0.0.0",
      "rabbitmq::community_plugins": ">= 0.0.0",
      "rabbitmq": ">= 0.0.0",
      "rabbitmq::erlang_package": ">= 0.0.0",
      "rabbitmq::esl_erlang_package": ">= 0.0.0",
      "rabbitmq::management_ui": ">= 0.0.0",
      "rabbitmq::mgmt_console": ">= 0.0.0",
      "rabbitmq::plugin_management": ">= 0.0.0",
      "rabbitmq::plugins": ">= 0.0.0",
      "rabbitmq::policies": ">= 0.0.0",
      "rabbitmq::policy_management": ">= 0.0.0",
      "rabbitmq::systemd_limits": ">= 0.0.0",
      "rabbitmq::user_management": ">= 0.0.0",
      "rabbitmq::users": ">= 0.0.0",
      "rabbitmq::vhosts": ">= 0.0.0",
      "rabbitmq::virtualhost_management": ">= 0.0.0"
    },
    "recipes": {
      "rabbitmq": "Install and configure RabbitMQ",
      "rabbitmq::systemd_limits": "Sets up kernel limits (e.g. nofile) for RabbitMQ via systemd",
      "rabbitmq::cluster": "Set up RabbitMQ clustering.",
      "rabbitmq::management_ui": "Sets up RabbitMQ management plugin/UI",
      "rabbitmq::mgmt_console": "Deprecated, alias for rabbitmq::management_ui",
      "rabbitmq::plugins": "Manage plugins with node attributes",
      "rabbitmq::plugin_management": "Deprecated, alias for rabbitmq::plugins",
      "rabbitmq::vhosts": "Manage virtual hosts with node attributes",
      "rabbitmq::virtualhost_management": "Deprecated, alias for rabbitmq::vhosts",
      "rabbitmq::users": "Manage users with node attributes",
      "rabbitmq::user_management": "Deprecated, alias for rabbitmq::users",
      "rabbitmq::policies": "Manage policies with node attributes",
      "rabbitmq::policy_management": "Deprecated, alias for rabbitmq::policies",
      "rabbitmq::erlang_package": "Provisions Erlang via Team RabbitMQ packages",
      "rabbitmq::esl_erlang_package": "Alias for erlang::esl",
      "rabbitmq::community_plugins": ""
    },
    "version": "5.7.7",
    "source_url": "https://github.com/rabbitmq/chef-cookbook",
    "issues_url": "https://github.com/rabbitmq/chef-cookbook/issues",
    "privacy": false,
    "chef_versions": [
    ],
    "ohai_versions": [
    ],
    "gems": [
    ]
  }
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### GET

The `GET` method is used to return a single cookbook artifact version.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/cookbook_artifacts/NAME/ID
```

This method has no request body.

**Response**

The response is similar to:

``` javascript
{
  "version": "5.7.7",
  "name": "rabbitmq",
  "identifier": "f3cf8ea7d8bfc59e35ec541946e3e82cd4b73e74",
  "frozen?": false,
  "chef_type": "cookbook_version",
  "attributes": [
    {
      "name": "default.rb",
      "path": "attributes/default.rb",
      "checksum": "e5a530cca3898d8bd07604435dc5156e",
      "specificity": "default",
      "url": "https://chef.example/bookshelf/organization-9f69768696feedcd165633b8b475cc0b/checksum-e5a530cca3898d8bd07604435dc5156e"
    }
  ],
  "definitions": [
  ],
  "files": [
  ],
  "libraries": [
    {
      "name": "matchers.rb",
      "path": "libraries/matchers.rb",
      "checksum": "24c3f44c4d1d62300a56051f0069f639",
      "specificity": "default",
      "url": "https://chef.example/bookshelf/organization-9f69768696feedcd165633b8b475cc0b/checksum-24c3f44c4d1d62300a56051f0069f639"
    },
    {
      "name": "helpers.rb",
      "path": "libraries/helpers.rb",
      "checksum": "df65c4a7259fcb30c6f3f1305ebf7502",
      "specificity": "default",
      "url": "https://chef.example/bookshelf/organization-9f69768696feedcd165633b8b475cc0b/checksum-df65c4a7259fcb30c6f3f1305ebf7502"
    },
    {
      "name": "default.rb",
      "path": "libraries/default.rb",
      "checksum": "94292faac84ba797e720501700b30f74",
      "specificity": "default",
      "url": "https://chef.example/bookshelf/organization-9f69768696feedcd165633b8b475cc0b/checksum-94292faac84ba797e720501700b30f74"
    }
  ],
  "providers": [
    {
      "name": "user.rb",
      "path": "providers/user.rb",
      "checksum": "c31c9cc749f21962c825f983a6679d94",
      "specificity": "default",
      "url": "https://chef.example/bookshelf/organization-9f69768696feedcd165633b8b475cc0b/checksum-c31c9cc749f21962c825f983a6679d94"
    },
    {
      "name": "policy.rb",
      "path": "providers/policy.rb",
      "checksum": "746c8a3f248f5bbfa51f5d2ba60b6315",
      "specificity": "default",
      "url": "https://chef.example/bookshelf/organization-9f69768696feedcd165633b8b475cc0b/checksum-746c8a3f248f5bbfa51f5d2ba60b6315"
    }
  ],
  "recipes": [
    {
      "name": "default.rb",
      "path": "recipes/default.rb",
      "checksum": "99a9b404ff6038d6ac55a90ca68c347a",
      "specificity": "default",
      "url": "https://chef.example/bookshelf/organization-9f69768696feedcd165633b8b475cc0b/checksum-99a9b404ff6038d6ac55a90ca68c347a"
    },
    {
      "name": "cluster.rb",
      "path": "recipes/cluster.rb",
      "checksum": "fc0a86c1f858c9d37e11282efc9fe329",
      "specificity": "default",
      "url": "https://chef.example/bookshelf/organization-9f69768696feedcd165633b8b475cc0b/checksum-fc0a86c1f858c9d37e11282efc9fe329"
    }
  ],
  "resources": [
    {
      "name": "cluster.rb",
      "path": "resources/cluster.rb",
      "checksum": "85e74276e19bfdad581dce4f5c59f94a",
      "specificity": "default",
      "url": "https://chef.example/bookshelf/organization-9f69768696feedcd165633b8b475cc0b/checksum-85e74276e19bfdad581dce4f5c59f94a"
    }
  ],
  "root_files": [
    {
      "name": "metadata.rb",
      "path": "metadata.rb",
      "checksum": "36b395e758138a4295d1e3f9b3df5da9",
      "specificity": "default",
      "url": "https://chef.example/bookshelf/organization-9f69768696feedcd165633b8b475cc0b/checksum-36b395e758138a4295d1e3f9b3df5da9"
    },
    {
      "name": "README.md",
      "path": "README.md",
      "checksum": "99873670f0994642f5e6baade52c8020",
      "specificity": "default",
      "url": "https://chef.example/bookshelf/organization-9f69768696feedcd165633b8b475cc0b/checksum-99873670f0994642f5e6baade52c8020"
    }
  ],
  "templates": [
    {
      "name": "default.rabbitmq-server.erb",
      "path": "templates/default/default.rabbitmq-server.erb",
      "checksum": "077855f4dc37f7fb708976134d8b2551",
      "specificity": "default",
      "url": "https://chef.example/bookshelf/organization-9f69768696feedcd165633b8b475cc0b/checksum-077855f4dc37f7fb708976134d8b2551"
    },
    {
      "name": "90forceyes.erb",
      "path": "templates/default/90forceyes.erb",
      "checksum": "73cc571097cf77c74b4e7b5b680020c9",
      "specificity": "default",
      "url": "https://chef.example/bookshelf/organization-9f69768696feedcd165633b8b475cc0b/checksum-73cc571097cf77c74b4e7b5b680020c9"
    }
  ],
  "metadata": {
    "name": "rabbitmq",
    "description": "Installs and configures RabbitMQ server",
    "long_description": "",
    "maintainer": "Chef, Inc. and contributors",
    "maintainer_email": "mklishin@pivotal.io",
    "license": "Apache-2.0",
    "platforms": {
      "amazon": ">= 2.0",
      "centos": ">= 7.0",
      "debian": ">= 8.0",
      "opensuse": ">= 0.0.0",
      "opensuseleap": ">= 0.0.0",
      "oracle": ">= 0.0.0",
      "redhat": ">= 0.0.0",
      "scientific": ">= 0.0.0",
      "smartos": ">= 0.0.0",
      "suse": ">= 0.0.0",
      "ubuntu": ">= 14.04"
    },
    "dependencies": {
      "erlang": ">= 0.0.0",
      "yum-epel": ">= 0.0.0",
      "yum-erlang_solutions": ">= 0.0.0",
      "dpkg_autostart": ">= 0.0.0",
      "logrotate": ">= 0.0.0"
    },
    "providing": {
      "rabbitmq::cluster": ">= 0.0.0",
      "rabbitmq::community_plugins": ">= 0.0.0",
      "rabbitmq": ">= 0.0.0",
      "rabbitmq::erlang_package": ">= 0.0.0",
      "rabbitmq::esl_erlang_package": ">= 0.0.0",
      "rabbitmq::management_ui": ">= 0.0.0",
      "rabbitmq::mgmt_console": ">= 0.0.0",
      "rabbitmq::plugin_management": ">= 0.0.0",
      "rabbitmq::plugins": ">= 0.0.0",
      "rabbitmq::policies": ">= 0.0.0",
      "rabbitmq::policy_management": ">= 0.0.0",
      "rabbitmq::systemd_limits": ">= 0.0.0",
      "rabbitmq::user_management": ">= 0.0.0",
      "rabbitmq::users": ">= 0.0.0",
      "rabbitmq::vhosts": ">= 0.0.0",
      "rabbitmq::virtualhost_management": ">= 0.0.0"
    },
    "recipes": {
      "rabbitmq": "Install and configure RabbitMQ",
      "rabbitmq::systemd_limits": "Sets up kernel limits (e.g. nofile) for RabbitMQ via systemd",
      "rabbitmq::cluster": "Set up RabbitMQ clustering.",
      "rabbitmq::management_ui": "Sets up RabbitMQ management plugin/UI",
      "rabbitmq::mgmt_console": "Deprecated, alias for rabbitmq::management_ui",
      "rabbitmq::plugins": "Manage plugins with node attributes",
      "rabbitmq::plugin_management": "Deprecated, alias for rabbitmq::plugins",
      "rabbitmq::vhosts": "Manage virtual hosts with node attributes",
      "rabbitmq::virtualhost_management": "Deprecated, alias for rabbitmq::vhosts",
      "rabbitmq::users": "Manage users with node attributes",
      "rabbitmq::user_management": "Deprecated, alias for rabbitmq::users",
      "rabbitmq::policies": "Manage policies with node attributes",
      "rabbitmq::policy_management": "Deprecated, alias for rabbitmq::policies",
      "rabbitmq::erlang_package": "Provisions Erlang via Team RabbitMQ packages",
      "rabbitmq::esl_erlang_package": "Alias for erlang::esl",
      "rabbitmq::community_plugins": ""
    },
    "version": "5.7.7",
    "source_url": "https://github.com/rabbitmq/chef-cookbook",
    "issues_url": "https://github.com/rabbitmq/chef-cookbook/issues",
    "privacy": false,
    "chef_versions": [
    ],
    "ohai_versions": [
    ],
    "gems": [
    ]
  }
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>


#### PUT

The `PUT` method is used to create or update a single cookbook artifact version.

This method has no parameters.

**Request**

``` none
PUT /organizations/NAME/cookbook_artifacts/NAME/ID
```

The request body is similar to:

``` javascript
{
  "definitions": [
    {
      "name": "unicorn_config.rb",
      "checksum": "c92b659171552e896074caa58dada0c2",
      "path": "definitions/unicorn_config.rb",
      "specificity": "default"
    }
  ],
  "attributes": [],
  "files": [],
  "providers": [],
  "metadata": {
    "dependencies": {"ruby": [], "rubygems": []},
    "name": "unicorn",
    "maintainer_email": "ops@chef.io",
    "attributes": {},
    "license": "Apache 2.0",
    "suggestions": {},
    "platforms": {},
    "maintainer": "Opscode, Inc",
    "long_description": "= LICENSE AND AUTHOR:\\n\\nAuthor:: Adam Jacob...",
    "recommendations": {},
    "version": "0.1.2",
    "conflicting": {},
    "recipes": {"unicorn": "Installs unicorn rubygem"},
    "groupings": {},
    "replacing": {},
    "description": "Installs/Configures unicorn",
    "providing": {}
  },
  "libraries": [],
  "templates": [
    {
      "name": "unicorn.rb.erb",
      "checksum": "36a1cc1b225708db96d48026c3f624b2",
      "path": "templates/default/unicorn.rb.erb",
      "specificity": "default"
    }
  ],
  "resources": [],
  "name": "unicorn",
  "identifier": "ba0dadcbca26710a521e0e3160cc5e20",
  "recipes": [
    {
      "name": "default.rb",
      "checksum": "ba0dadcbca26710a521e0e3160cc5e20",
      "path": "recipes/default.rb",
      "specificity": "default"
    }
  ],
  "root_files": [
    {
      "name": "README.rdoc",
      "checksum": "d18c630c8a68ffa4852d13214d0525a6",
      "path": "README.rdoc",
      "specificity": "default"
    },
    {
      "name": "metadata.rb",
      "checksum": "967087a09f48f234028d3aa27a094882",
      "path": "metadata.rb",
      "specificity": "default"
    },
    {
      "name": "metadata.json",
      "checksum": "45b27c78955f6a738d2d42d88056c57c",
      "path": "metadata.json",
      "specificity": "default"
    }
  ],
  "chef_type": "cookbook_artifact_version"
}
```

where the `checksum` values must have already been uploaded to the Chef
Infra Server using the sandbox endpoint. Once a file with a particular
checksum has been uploaded by the user, redundant uploads are not
necessary. Unused `checksum` values will be garbage collected.

**Response**

This method has no response body.

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>413</code></td>
<td>Request entity too large. A request may not be larger than 1000000 bytes.</td>
</tr>
</tbody>
</table>

### /cookbooks

{{% cookbooks_summary %}}

When a cookbook is uploaded, only files that are new or updated will be
included. This approach minimizes the amount of storage and time that is
required during the modify-upload-test cycle. To keep track of which
files have already been uploaded, Chef Infra Client uses a checksum and
assigns a checksum to each file. These checksums are used in the
cookbook version manifest, alongside the same records that store the
file description (name, specificity, and so on), as well as the checksum
and the URL from which the file's contents can be retrieved.

The `/cookbooks` endpoint has the following methods: `GET`.

#### GET

The `GET` method is used to return a hash of all cookbooks and cookbook
versions.

This method has the following parameters:

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
</colgroup>
<thead>
<tr class="header">
<th>Parameter</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>num_versions=n</code></td>
<td>The number of cookbook versions to include in the response, where <code>n</code> is the number of cookbook versions. For example: <code>num_versions=3</code> returns the three latest versions, in descending order (newest to oldest). Use <code>num_versions=all</code> to return all cookbook versions. If <code>num_versions</code> is not specified, a single cookbook version is returned. <code>0</code> is an invalid input (an empty array for the versions of each cookbook is returned).</td>
</tr>
</tbody>
</table>

**Request**

``` none
GET /organizations/NAME/cookbooks
```

**Response**

The response is similar to:

``` javascript
{
  "apache2": {
    "url": "https://localhost/cookbooks/apache2",
    "versions": [
      {"url": "https://localhost/cookbooks/apache2/5.1.0",
       "version": "5.1.0"},
      {"url": "https://localhost/cookbooks/apache2/4.2.0",
       "version": "4.2.0"}
    ]
  },
  "nginx": {
    "url": "https://localhost/cookbooks/nginx",
    "versions": [
      {"url": "https://localhost/cookbooks/nginx/1.0.0",
       "version": "1.0.0"},
      {"url": "https://localhost/cookbooks/nginx/0.3.0",
       "version": "0.3.0"}
    ]
  }
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
</tbody>
</table>

### /cookbooks/_latest

The `/cookbooks/_latest` endpoint has the following methods: `GET`.

#### GET

The `GET` method is used to return a list of the most recent cookbook
versions.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/cookbooks/_latest
```

**Response**

For example, if cookbooks `foo` and `bar` both exist on the Chef Infra
Server and both with versions `0.1.0` and `0.2.0`, the response is
similar to:

``` javascript
{
  "foo": "https://localhost/cookbooks/foo/0.2.0",
  "bar": "https://localhost/cookbooks/bar/0.2.0"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

### /cookbooks/_recipes

The `/cookbooks/_recipes` endpoint has the following methods: `GET`.

#### GET

The `GET` method is used to return the names of all recipes in the most
recent cookbook versions.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/cookbooks/_recipes
```

**Response**

The response is similar to:

``` javascript
{

}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

### /cookbooks/NAME

The `/cookbooks/NAME` endpoint has the following methods: `GET`.

#### GET

The `GET` method is used to return a hash that contains a key-value pair
that corresponds to the specified cookbook, with a URL for the cookbook
and for each version of the cookbook.

**Request**

``` none
GET /organizations/NAME/cookbooks/NAME
```

**Response**

The response is similar to:

``` javascript
{
  "apache2": {
    "url": "https://localhost/cookbooks/apache2",
    "versions": [
      {"url": "https://localhost/cookbooks/apache2/5.1.0",
       "version": "5.1.0"},
      {"url": "https://localhost/cookbooks/apache2/4.2.0",
       "version": "4.2.0"}
    ]
  }
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

### /cookbooks/NAME/version

{{% cookbooks_version %}}

The `/cookbooks/NAME/VERSION` endpoint has the following methods:
`DELETE`, `GET`, and `PUT`.

#### DELETE

The `DELETE` method is used to delete a cookbook version.

This method has no parameters.

**Request**

``` none
DELETE /organizations/NAME/cookbooks/NAME/VERSION
```

**Response**

This method has no response body. Unused `checksum` values will be
garbage collected.

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### GET

The `GET` method is used to return a description of a cookbook,
including its metadata and links to component files.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/cookbooks/NAME/VERSION
```

where `VERSION` can be `_latest` in order to float to head.

**Response**

The response is similar to:

``` javascript
{
  "cookbook_name": "getting-started",
  "files": [

  ],
  "chef_type": "cookbook_version",
  "definitions": [

  ],
  "libraries": [

  ],
  "attributes": [
    {
      "url": "https://domain.com/org_name/(...rest of URL)",
      "path": "attributes/default.rb",
      "specificity": "default",
      "name": "default.rb",
      "checksum": "fa0fc4abf3f6787fdsaasadfrc5c35de667c"
    }
  ],
  "recipes": [
    {
      "url": "https://domain.com/org_name/(...rest of URL)",
      "path": "recipes/default.rb",
      "specificity": "default",
      "name": "default.rb",
      "checksum": "7e79b1ace7728fdsadfsdaf857e60fc69"
    }
  ],
  "providers": [

  ],
  "resources": [

  ],
  "templates": [
    {
      "url": "https://domain.com/org_name/(...rest of URL)",
      "path": "templates/default/chef-getting-started.txt.erb",
      "specificity": "default",
      "name": "chef-getting-started.txt.erb",
      "checksum": "a29d6f2545sdffds1f140c3a78b1fe"
    }
  ],
  "root_files": [
    {
      "url": "https://domain.com/org_name/(...rest of URL)",
      "path": ".DS_Store",
      "specificity": "default",
      "name": ".DS_Store",
      "checksum": "c107b500aafd12asdffdsdf5c2a7d6"
    },
    {
      "url": "https://domain.com/org_name/(...rest of URL)",
      "path": "metadata.json",
      "specificity": "default",
      "name": "metadata.json",
      "checksum": "20f09570e54dasdf0f3ae01e6401c90f"
    },
    {
      "url": "https://domain.com/org_name/(...rest of URL)",
      "path": "metadata.rb",
      "specificity": "default",
      "name": "metadata.rb",
      "checksum": "71027aefasd487fdsa4cb6994b66ed"
    },
    {
      "url": "https://domain.com/org_name/(...rest of URL)",
      "path": "README.rdoc",
      "specificity": "default",
      "name": "README.rdoc",
      "checksum": "8b9275e56fee974easdfasdfbb729"
    }
  ],
  "name": "getting-started-0.4.0",
  "frozen?": false,
  "version": "0.4.0",
  "json_class": "Chef::CookbookVersion",
  "metadata": {
    "maintainer": "Maintainer",
    "attributes": { },
    "suggestions": { },
    "recipes": { "getting-started": "" },
    "dependencies": { },
    "platforms": { },
    "groupings": { },
    "recommendations": { },
    "name": "getting-started",
    "description": "description",
    "version": "0.4.0",
    "maintainer_email": "sysadmin@opscode.com",
    "long_description": "= DESCRIPTION:\n\nThis cookbook is used to do some things.\n\n",
    "providing": { "getting-started": ">= 0.0.0" },
    "replacing": { },
    "conflicting": { },
    "license": "Apache 2.0"
  }
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### PUT

The `PUT` method is used to create or update a cookbook version.

This method has no parameters.

**Request**

``` none
PUT /organizations/NAME/cookbooks/NAME/VERSION
```

with a request body similar to:

``` javascript
{
  "definitions": [
    {
      "name": "unicorn_config.rb",
      "checksum": "c92b659171552e896074caa58dada0c2",
      "path": "definitions/unicorn_config.rb",
      "specificity": "default"
    }
  ],
  "name": "unicorn-0.1.2",
  "attributes": [],
  "files": [],
  "json_class": "Chef::CookbookVersion",
  "providers": [],
  "metadata": {
    "dependencies": {"ruby": [], "rubygems": []},
    "name": "unicorn",
    "maintainer_email": "ops@opscode.com",
    "attributes": {},
    "license": "Apache 2.0",
    "suggestions": {},
    "platforms": {},
    "maintainer": "Opscode, Inc",
    "long_description": "= LICENSE AND AUTHOR:\n\nAuthor:: Adam Jacob...",
    "recommendations": {},
    "version": "0.1.2",
    "conflicting": {},
    "recipes": {"unicorn": "Installs unicorn rubygem"},
    "groupings": {},
    "replacing": {},
    "description": "Installs/Configures unicorn",
    "providing": {}
  },
  "libraries": [],
  "templates": [
    {
      "name": "unicorn.rb.erb",
      "checksum": "36a1cc1b225708db96d48026c3f624b2",
      "path": "templates/default/unicorn.rb.erb",
      "specificity": "default"
    }
  ],
  "resources": [],
  "cookbook_name": "unicorn",
  "version": "0.1.2",
  "recipes": [
    {
      "name": "default.rb",
      "checksum": "ba0dadcbca26710a521e0e3160cc5e20",
      "path": "recipes/default.rb",
      "specificity": "default"
    }
  ],
  "root_files": [
    {
      "name": "README.rdoc",
      "checksum": "d18c630c8a68ffa4852d13214d0525a6",
      "path": "README.rdoc",
      "specificity": "default"
    },
    {
      "name": "metadata.rb",
      "checksum": "967087a09f48f234028d3aa27a094882",
      "path": "metadata.rb",
      "specificity": "default"
    },
    {
      "name": "metadata.json",
      "checksum": "45b27c78955f6a738d2d42d88056c57c",
      "path": "metadata.json",
      "specificity": "default"
    }
  ],
  "chef_type": "cookbook_version"
}
```

where the `checksum` values must have already been uploaded to the Chef
Infra Server using the sandbox endpoint. Once a file with a particular
checksum has been uploaded by the user, redundant uploads are not
necessary. Unused `checksum` values will be garbage collected.

**Response**

This method has no response body.

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>413</code></td>
<td>Request entity too large. A request may not be larger than 1000000 bytes.</td>
</tr>
</tbody>
</table>

### /data

{{% data_bag %}}

The `/data` endpoint has the following methods: `GET` and `POST`.

#### GET

The `GET` method is used to return a list of all data bags on the Chef
Infra Server.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/data
```

**Response**

The response is similar to:

``` javascript
{
  "users": "https://chef.example/organizations/NAME/data/users",
  "applications": "https://chef.example/organizations/NAME/data/applications"
}
```

shown as a list of key-value pairs, where (in the example above) `users`
and `applications` are the names of data bags and
`https://chef.example/organizations/NAME/data/foo` is the path to the data bag.

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
</tbody>
</table>

#### POST

The `POST` method is used to create a new data bag on the Chef Infra
Server.

This method has no parameters.

**Request**

``` none
POST /organizations/NAME/data
```

with a request body that contains the key-value pair for the data bag
and is similar to:

``` javascript
{
  "name": "users"
}
```

where (in the example above) `name` is the key and "users" is the value.

**Response**

The response is similar to:

``` javascript
{
   "uri": "https://organizations/NAME/data/users",
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>201</code></td>
<td>Created. The object was created.</td>
</tr>
<tr class="even">
<td><code>400</code></td>
<td>Bad request. The contents of the request are not formatted correctly.</td>
</tr>
<tr class="odd">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="even">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>409</code></td>
<td>Conflict. A databag with that name already exists.</td>
</tr>
<tr class="odd">
<td><code>413</code></td>
<td>Request entity too large. A request may not be larger than 1000000 bytes.</td>
</tr>
</tbody>
</table>

### /data/NAME

The `/data/NAME` endpoint is used to view and update data for a specific
data bag. This endpoint has the following methods: `DELETE`, `GET`, and `POST`.

#### DELETE

The `DELETE` method is used to delete a data bag.

This method has no parameters.

**Request**

``` none
DELETE /organizations/NAME/data/NAME
```

This method has no request body.

**Response**

The response is similar to:

``` javascript
{
  "name": "users",
  "json_class": "Chef::DataBag",
  "chef_type": "data_bag"
}
```

where the key-value pairs represent the last state of the data bag item.

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### GET

The `GET` method is used to return a hash of all entries in the
specified data bag.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/data/NAME
```

**Response**

The response is similar to:

``` javascript
{
   "adam": "https://chef.example/organizations/NAME/data/users/adam"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### POST

The `POST` method is used to create a new data bag item.

This method has no parameters.

**Request**

``` none
POST /organizations/NAME/data/NAME
```

with a request body similar to:

``` javascript
{
  "id": "adam",
  "real_name": "Adam Jacob"
}
```

where `id` is required.

**Response**

This method has no response body.

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>201</code></td>
<td>OK. The item was created.</td>
</tr>
<tr class="even">
<td><code>400</code></td>
<td>Bad request. The contents of the request are not formatted correctly.</td>
</tr>
<tr class="odd">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="even">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="odd">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
<tr class="even">
<td><code>409</code></td>
<td>Conflict. The object already exists.</td>
</tr>
<tr class="odd">
<td><code>413</code></td>
<td>Request entity too large. A request may not be larger than 1000000 bytes.</td>
</tr>
</tbody>
</table>

### /data/NAME/ITEM

{{% data_bag_item %}}

The `/data/NAME/ITEM` endpoint allows the key-value pairs within a data
bag item to be viewed and managed. This endpoint has the following
methods: `DELETE`, `GET`, and `PUT`.

#### DELETE

The `DELETE` method is used to delete a key-value pair in a data bag
item.

This method has no parameters.

**Request**

``` none
DELETE /organizations/NAME/data/NAME/ITEM
```

**Response**

The response is similar to:

``` javascript
{
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### GET

The `GET` method is used to view all of the key-value pairs in a data
bag item.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/data/NAME/ITEM
```

**Response**

The response is similar to:

``` javascript
{
  "real_name": "Adam Jacob",
  "id": "adam"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### PUT

The `PUT` method is used to replace the contents of a data bag item with
the contents of this request.

This method has no parameters.

**Request**

``` none
PUT /organizations/NAME/data/NAME/ITEM
```

with a request body similar to:

``` javascript
{
  "real_name": "Adam Brent Jacob",
  "id": "adam"
}
```

where `id` is required.

**Response**

The response is similar to:

``` javascript
{
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
<tr class="odd">
<td><code>413</code></td>
<td>Request entity too large. A request may not be larger than 1000000 bytes.</td>
</tr>
</tbody>
</table>

### /environments

{{% environment %}}

The `/environments` endpoint has the following methods: `GET` and
`POST`.

#### GET

The `GET` method is used to return a data structure that contains a link
to each available environment.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/environments
```

**Response**

The response is similar to:

``` javascript
{
  "_default": "https://api.opscode.com/organizations/org_name/environments/_default",
  "webserver": "https://api.opscode.com/organizations/org_name/environments/webserver"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
</tbody>
</table>

#### POST

The `POST` method is used to create a new environment.

This method has no parameters.

**Request**

``` none
POST /organizations/NAME/environments
```

with a request body similar to:

``` javascript
{
  "name": "dev",
  "default_attributes": {},
  "json_class": "Chef::Environment",
  "description": "",
  "cookbook_versions": {},
  "chef_type": "environment"
}
```

**Response**

The response is similar to:

``` javascript
{ "uri": "https://localhost/environments/dev" }
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>201</code></td>
<td>Created. The object was created.</td>
</tr>
<tr class="even">
<td><code>400</code></td>
<td>Bad request. The contents of the request are not formatted correctly.</td>
</tr>
<tr class="odd">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="even">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="odd">
<td><code>409</code></td>
<td>Conflict. The object already exists.</td>
</tr>
<tr class="even">
<td><code>413</code></td>
<td>Request entity too large. A request may not be larger than 1000000 bytes.</td>
</tr>
</tbody>
</table>

### /environments/_default

The `/environments/_default` endpoint has the following methods: `GET`.

#### GET

The `GET` method is used to get information about the `_default`
environment on the Chef Infra Server.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/environments/_default
```

**Response**

The response is similar to:

``` javascript
{
  "name": "_default",
  "description": "The default Chef environment",
  "json_class": "Chef::Environment",
  "chef_type": "environment",
  "default_attributes": {

  },
  "override_attributes": {

  },
  "cookbook_versions": {

  }
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

### /environments/NAME

The `/environments/NAME` endpoint has the following methods: `DELETE`,
`GET`, and `PUT`.

#### DELETE

The `DELETE` method is used to delete an environment.

This method has no parameters.

**Request**

``` none
DELETE /organizations/NAME/environments/NAME
```

**Response**

The response will return the JSON for the environment that was deleted,
similar to:

``` javascript
{
  "name":"backend",
  "description":"",
  "cookbook_versions":{},
  "json_class":"Chef::Environment",
  "chef_type":"environment",
  "default_attributes":{},
  "override_attributes":{}
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### GET

The `GET` method is used to return the details for an environment as
JSON.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/environments/NAME
```

**Response**

The response is similar to:

``` javascript
{
  "name": "_default",
  "description": "The default Chef environment",
  "json_class": "Chef::Environment",
  "chef_type": "environment",
  "default_attributes": { }
  "override_attributes": { },
  "cookbook_versions": { },
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### PUT

The `PUT` method is used to update the details of an environment on the
Chef Infra Server.

This method has no parameters.

**Request**

``` none
PUT /organizations/NAME/environments/NAME
```

with a request body that contains the updated JSON for the environment
and is similar to:

``` javascript
{
  "name": "dev",
  "attributes": {},
  "json_class": "Chef::Environment",
  "description": "The Dev Environment",
  "cookbook_versions": {},
  "chef_type": "environment"
}
```

**Response**

The response will return the updated environment.

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
<tr class="odd">
<td><code>413</code></td>
<td>Request entity too large. A request may not be larger than 1000000 bytes.</td>
</tr>
</tbody>
</table>

### /environments/NAME/cookbooks/NAME

The `/environments/NAME/cookbooks/NAME` endpoint has the following
methods: `GET`.

#### GET

The `GET` method is used to return a hash of key-value pairs for the
requested cookbook.

This method has the following parameters:

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
</colgroup>
<thead>
<tr class="header">
<th>Parameter</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>num_versions=n</code></td>
<td>The number of cookbook versions to include in the response, where <code>n</code> is the number of cookbook versions. For example: <code>num_versions=3</code> returns the three latest versions, in descending order (newest to oldest). Use <code>num_versions=all</code> to return all cookbook versions. If <code>num_versions</code> is not specified, a single cookbook version is returned. <code>0</code> is an invalid input (an empty array for the versions of each cookbook is returned).</td>
</tr>
</tbody>
</table>

**Request**

``` none
GET /organizations/NAME/environments/NAME/cookbooks/NAME
```

where the first instance of `NAME` is the name of the environment, and
the second instance is the name of the cookbook.

**Response**

The response is similar to:

``` none
{
  "apache2": {
    "url": "https://localhost/cookbooks/apache2",
    "versions": [
      {"url": "https://localhost/cookbooks/apache2/5.1.0",
       "version": "5.1.0"},
      {"url": "https://localhost/cookbooks/apache2/4.2.0",
       "version": "4.2.0"}
    ]
  }
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

### /environments/NAME/cookbook_versions

The `/environments/NAME/cookbook_versions` endpoint has the following
methods: `POST`.

#### POST

The `POST` method is used to return a hash of the cookbooks and cookbook
versions (including all dependencies) that are required by the
`run_list` array. Version constraints may be specified using the `@`
symbol after the cookbook name as a delimiter. Version constraints may
also be present when the `cookbook_versions` attributes is specified for
an environment or when dependencies are specified by a cookbook.

This method has no parameters.

**Request**

``` none
POST /organizations/NAME/environments/NAME/cookbook_versions
```

with a request body similar to:

``` javascript
{
  "run_list": [
    "zed@0.0.1",
    "bar",
    "mysql",
    "gem",
    "nginx@0.99.2",
    "cron",
    "foo"
  ]
}
```

where `@x.x.x` represents a constraint for a cookbook version.

**Response**

The response will return a list of cookbooks that are required by the
`run_list` array contained in the request. The cookbooks that are
returned are often the latest versions of each cookbook. Depending on
any constraints present in the request or on dependencies a cookbook may
have for specific cookbook versions, a request may not always return the
latest cookbook version for each cookbook.

The response is similar to:

``` javascript
{
  "cookbook_name": {
    "recipes": [
      {
        "name": "default.rb",
        "path": "recipes/default.rb",
        "checksum": "12345efg78912346abcddefg789",
        "specificity": "default",
        "url": "https://URL"
      },
      {
        "name": "recipe_name.rb",
        "path": "recipes/recipe_name.rb",
        "checksum": "12345efg78912346abcddefg789",
        "specificity": "default",
        "url": "https://URL"
      },
      {
        ...
      }
    ],
    "definitions": [

    ],
    "libraries": [

    ],
    "attributes": [

    ],
    "files": [

    ],
    "templates": [
      {
        "name": "template_name.erb",
        "path": "templates/default/template_name.erb",
        "checksum": "12345efg78912346abcddefg789",
        "specificity": "default",
        "url": "https://URL"
      },
      {
        ...
      }
    ],
    "resources": [

    ],
    "providers": [

    ],
    "root_files": [
      {
        "name": "metadata.rb",
        "path": "metadata.rb",
        "checksum": "12345efg78912346abcddefg789",
        "specificity": "default",
        "url": "https://URL"
      }
    ],
    "cookbook_name": "cookbook_name-1.0.2",
    "metadata": {
      "name": "cookbook_name",
      "description": "description",
      "long_description": "",
      "maintainer": "maintainer",
      "maintainer_email": "maintainer@email.com",
      "license": "license",
      "platforms": {
      },
      "dependencies": {
        "cookbook_name": ">= 0.0.0",
        "cookbook_name": ">= 1.2.3",
        ...
        "cookbook_name": ">= 0.1.0"
      },
      "recommendations": {
      },
      "suggestions": {
      },
      "conflicting": {
      },
      "providing": {
        "cookbook_name": ">= 0.0.0",
        "cookbook_name::recipe_name": ">= 0.0.0",
        "cookbook_name::recipe_name": ">= 1.2.3",
        "cookbook_name::recipe_name": ">= 0.1.0"
      },
      "replacing": {
      },
      "attributes": {
      },
      "groupings": {
      },
      "recipes": {
        "cookbook_name": "description",
        "cookbook_name::recipe_name": "",
        ...
        "cookbook_name::recipe_name": ""
      },
      "version": "0.0.0"
    },
    "version": "0.0.0",
    "name": "cookbook_name-1.0.2",
    "frozen?": false,
    "chef_type": "cookbook_version",
    "json_class": "Chef::CookbookVersion"
  },
  "cookbook_name": {
     "recipes": [
      {
        "name": "default.rb",
        "path": "recipes/default.rb",
        "checksum": "12345efg78912346abcddefg789",
        "specificity": "default",
        "url": "https://URL"
      },
    ],
    "definitions": [

    ],
    "libraries": [
      {
        "name": "library_file.rb",
        "path": "libraries/library_file.rb",
        "checksum": "12345efg78912346abcddefg789",
        "specificity": "default",
        "url": "https://URL"
      }
    ],
    "attributes": [
      {
        "name": "default.rb",
        "path": "attributes/default.rb",
        "checksum": "12345efg78912346abcddefg789",
        "specificity": "default",
        "url": "https://URL"
      }
    ],
    "files": [

    ],
    "templates": [

    ],
    "resources": [

    ],
    "providers": [

    ],
    "root_files": [
      {
        "name": ".gitignore",
        "path": ".gitignore",
        "checksum": "12345efg78912346abcddefg789",
        "specificity": "default",
        "url": "https://URL"
      },
      {
        "name": ".kitchen.yml",
        "path": ".kitchen.yml",
        "checksum": "12345efg78912346abcddefg789",
        "specificity": "default",
        "url": "https://URL"
      },
      {
        "name": "CHANGELOG.md",
        "path": "CHANGELOG.md",
        "checksum": "12345efg78912346abcddefg789",
        "specificity": "default",
        "url": "https://URL"
      },
      {
        "name": "CONTRIBUTING",
        "path": "CONTRIBUTING",
        "checksum": "12345efg78912346abcddefg789",
        "specificity": "default",
        "url": "https://URL"
      },
      {
        "name": "LICENSE",
        "path": "LICENSE",
        "checksum": "12345efg78912346abcddefg789",
        "specificity": "default",
        "url": "https://URL"
      },
      {
        "name": "metadata.json",
        "path": "metadata.json",
        "checksum": "12345efg78912346abcddefg789",
        "specificity": "default",
        "url": "https://URL"
      },
      {
        "name": "metadata.rb",
        "path": "metadata.rb",
        "checksum": "12345efg78912346abcddefg789",
        "specificity": "default",
        "url": "https://URL"
      },
      {
        "name": "README.md",
        "path": "README.md",
        "checksum": "12345efg78912346abcddefg789",
        "specificity": "default",
        "url": "https://URL"
      },
    ],
    "chef_type": "cookbook_version",
    "name": "cookbook_name-1.0.2",
    "cookbook_name": "cookbook_name",
    "version": "1.0.2",
    "metadata": {
      "name": "cookbook_name",
      "description": "description",
      "long_description": "",
      "maintainer": "maintainer",
      "maintainer_email": "maintainer@email.com",
      "license": "license",
      "platforms": {
      },
      "dependencies": {
      },
      "recommendations": {
      },
      "suggestions": {
      },
      "conflicting": {
      },
      "providing": {
      },
      "replacing": {
      },
      "attributes": {
      },
      "groupings": {
      },
      "recipes": {
      },
      "version": "1.0.2"
    },
    "frozen?": true,
    "json_class": "Chef::CookbookVersion"
  },
  "cookbook_name": {
   ...
  }
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>400</code></td>
<td>Bad request. The contents of the request are not formatted correctly.</td>
</tr>
<tr class="odd">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="even">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="odd">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
<tr class="even">
<td><code>412</code></td>
<td>Not allowed. A set of cookbooks and/or cookbook versions could not be found that met all of the requirements of the run-list. A cookbook in the run-list may not exist. A dependency may be present for a cookbook that does not exist. A constraint on a cookbook made by a run-list, environment, or cookbook version, may not match an available cookbook version.</td>
</tr>
<tr class="odd">
<td><code>413</code></td>
<td>Request entity too large. A request may not be larger than 1000000 bytes.</td>
</tr>
</tbody>
</table>

### /environments/NAME/cookbooks

The `/environments/NAME/cookbooks` endpoint has the following methods:
`GET`.

#### GET

The `GET` method is used to get a list of cookbooks and cookbook
versions that are available to the specified environment.

This method has the following parameters:

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
</colgroup>
<thead>
<tr class="header">
<th>Parameter</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>num_versions=n</code></td>
<td>The number of cookbook versions to include in the response, where <code>n</code> is the number of cookbook versions. For example: <code>num_versions=3</code> returns the three latest versions, in descending order (newest to oldest). Use <code>num_versions=all</code> to return all cookbook versions. If <code>num_versions</code> is not specified, a single cookbook version is returned. <code>0</code> is an invalid input (an empty array for the versions of each cookbook is returned).</td>
</tr>
</tbody>
</table>

**Request**

``` none
GET /organizations/NAME/environments/NAME/cookbooks
```

**Response**

The response is similar to:

``` javascript
{
  "apache2": {
    "url": "https://localhost/cookbooks/apache2",
    "versions": [
      {"url": "https://localhost/cookbooks/apache2/5.1.0",
       "version": "5.1.0"},
      {"url": "https://localhost/cookbooks/apache2/4.2.0",
       "version": "4.2.0"}
    ]
  },
  "nginx": {
    "url": "https://localhost/cookbooks/nginx",
    "versions": [
      {"url": "https://localhost/cookbooks/nginx/1.0.0",
       "version": "1.0.0"},
      {"url": "https://localhost/cookbooks/nginx/0.3.0",
       "version": "0.3.0"}
    ]
  }
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

### /environments/NAME/nodes

The `/environments/NAME/nodes` endpoint has the following methods:
`GET`.

#### GET

The `GET` method is used to return a list of nodes in a given
environment.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/environments/NAME/nodes
```

**Response**

The response is similar to:

``` javascript
{
  "blah": "https://api.opscode.com/org/org_name/nodes/_default",
  "boxer": "https://api.opscode.com/org/org_name/nodes/frontend",
  "blarrrrgh": "https://api.opscode.com/org/org_name/nodes/backend"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

### /environments/NAME/recipes

The `/environments/NAME/recipes` endpoint has the following methods:
`GET`.

#### GET

The `GET` method is used to return a list of recipes available to a
given environment.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/environments/NAME/recipes
```

where the first instance of `NAME` is the name of the environment, and
the second instance is the name of the recipe.

**Response**

The response is similar to:

``` none
[
  "ant",
  "apache2",
  "apache2::mod_auth_openid",
  "apache2::mod_authnz_ldap",
  "apt",
  "aws",
  "capistrano",
  "chef",
  "chef::bootstrap_client",
  "chef-client::config",
  "chef-client",
  ...
]
```

The list of recipes will be the default recipes for a given cookbook. If
an environment has multiple versions of a cookbook that matches its
constraints, only the recipes from the latest version of that cookbook
will be reported.

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

### /environments/NAME/roles/NAME

The `/environments/NAME/roles/NAME` endpoint has the following methods:
`GET`.

#### GET

The `GET` method is used to return the `run_list` attribute of the role
(when the name of the environment is `_default`) or to return
`env_run_lists[environment_name]` (for non-default environments).

{{< note >}}

The behavior of this endpoint is identical to
`GET /roles/NAME/environments/NAME`; it is recommended (but not
required) that `GET /roles/NAME/environments/NAME` be used instead of
this endpoint.

{{< /note >}}

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/environments/NAME/roles/NAME
```

where the first instance of `NAME` is the name of the environment, and
the second instance is the name of the role.

**Response**

The response is similar to:

``` javascript
{
  "run_list": [
    "recipe[recipe_name]",
    "role[role_name]",
    "recipe[recipe_name]",
    "role[role_name]",
    "recipe[recipe_name]",
    "role[role_name]"
  ]
}
```

Chef Infra Client will pick up the `_default` run-list if
`env_run_list[environment_name]` is null or nonexistent.

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

### /groups

The `/groups` endpoint has the following methods: `GET` and `POST`.

#### GET

The `GET` method is used to get a list of groups on the Chef Infra
Server for a single organization.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/groups
```

**Response**

The response is similar to:

``` javascript
{
  "33a5c28a8efe11e195005fsaes25400298d3f": "https://url/for/group1",
  "admins": "https://url/for/groups/admins",
  "billing-admins": "https://url/for/billing-admins",
  "clients": "https://url/for/clients",
  "developers": "https://url/for/developers",
  "users": "https://url/for/groups/users"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### POST

The `POST` method is used to create a group on the Chef Infra
Server for a single organization.

**Request**

``` none
POST /organizations/NAME/groups
```

with a request body similar to:

``` javascript
{
  "name": "group1",
  "groupname": "group1",
  "orgname": "test",
  "actors": []
  "clients": ["mynode"],
  "groups": ["admins"],
  "users": ["betina"]
}
```

**Response**

The response is similar to:

``` javascript
{
  "uri": "https://chef.example/organizations/test/groups/group1",
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>201</code></td>
<td>OK. The group was created.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
<tr class="odd">
<td><code>409</code></td>
<td>Conflict. The requested group already exists.</td>
</tr>
</tbody>
</table>

### /groups/GROUP_NAME

The `/groups/GROUP_NAME` endpoint has the following methods: `DELETE`, `GET` and
`PUT`.

#### DELETE

The `DELETE` method is used to remove a group from a single organization.

This method has no parameters.

**Request**

``` none
DELETE /organizations/NAME/groups/GROUP_NAME
```

without a request body.

**Response**

The response is similar to:

``` javascript
{
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The group was deleted.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### GET

The `GET` method is used to get lists of users and other groups that belong to a group.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/groups/GROUP_NAME
```

**Response**

The response is similar to:

``` javascript
{
  "actors": [
    "pivotal",
    "grantmc"
  ],
  "users": [
    "pivotal",
    "grantmc"
  ],
  "clients": [

  ],
  "groups": [
    "000000000000ad94b5ddde157c070f0c"
  ],
  "orgname": "inbetweens",
  "name": "admins",
  "groupname": "admins"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### PUT

The `PUT` method is used to update a group on a single organization.
Updating the clients, groups and users memberships replaces the definitons for
the group. `GET` the group and merge changes to create the desired member lists.

This method has no parameters.

**Request**

``` none
PUT /organizations/NAME/groups/GROUP_NAME
```

with a request body similar to:

``` javascript
{
  "name": "group1",
  "groupname": "groupnew",
  "actors": {
    "clients": ["mynode","addme"],
    "groups": ["admins"],
    "users": ["betina"]
  }
}
```

**Response**

The response is similar to:

``` javascript
{
  "name": "group1",
  "groupname": "groupnew",
  "orgname": "test",
  "actors": {
    "clients": ["mynode","addme"],
    "groups": ["admins"],
    "users": ["betina"]
  }
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>201</code></td>
<td>OK. The group was updated.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

### /nodes

{{% node %}}

The `/nodes` endpoint has the following methods: `GET` and `POST`.

#### GET

The `GET` method is used to return a hash of URIs for nodes on the Chef
Infra Server.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/nodes
```

**Response**

The response is similar to:

``` javascript
{
  "latte": "https://localhost/nodes/latte"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
</tbody>
</table>

#### POST

The `POST` method is used to create a new node.

This method has no parameters.

**Request**

``` none
POST /organizations/NAME/nodes
```

with a request body similar to:

``` javascript
{
  "name": "latte",
  "chef_type": "node",
  "json_class": "Chef::Node",
  "attributes": {
    "hardware_type": "laptop"
  },
  "overrides": {},
  "defaults": {},
  "run_list": [ "recipe[unicorn]" ]
}
```

where `name` is the name of the node. Other attributes are optional. The
order of the `run_list` attribute matters.

**Response**

The response is similar to:

``` javascript
{ "uri": "https://localhost/nodes/latte" }
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>201</code></td>
<td>Created. The object was created.</td>
</tr>
<tr class="even">
<td><code>400</code></td>
<td>Bad request. The contents of the request are not formatted correctly.</td>
</tr>
<tr class="odd">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="even">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="odd">
<td><code>409</code></td>
<td>Conflict. The object already exists.</td>
</tr>
<tr class="even">
<td><code>413</code></td>
<td>Request entity too large. A request may not be larger than 1000000 bytes.</td>
</tr>
</tbody>
</table>

### /nodes/NAME

The `/nodes/NAME` endpoint has the following methods: `DELETE`, `GET`,
`HEAD` and `PUT`.

#### DELETE

The `DELETE` method is used to delete a node.

This method has no parameters.

**Request**

``` none
DELETE /organizations/NAME/nodes/NAME
```

**Response**

The response will return the last known state of the node, similar to:

``` javascript
{
  "overrides": {},
  "name": "latte",
  "chef_type": "node",
  "json_class": "Chef::Node",
  "attributes": {
    "hardware_type": "laptop"
  },
  "run_list": [
    "recipe[apache2]"
  ],
  "defaults": {}
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### GET

The `GET` method is used to return the details of a node as JSON.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/nodes/NAME
```

**Response**

The response is similar to:

``` javascript
{
  "name": "node_name",
  "chef_environment": "_default",
  "run_list": [
    "recipe[recipe_name]"
  ]
  "json_class": "Chef::Node",
  "chef_type": "node",
  "automatic": { ... },
  "normal": { "tags": [ ] },
  "default": { },
  "override": { }
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### HEAD

The `HEAD` method is used to check the existence of a node.

This method has no parameters.

**Request**

``` none
HEAD /organizations/NAME/nodes/NAME
```

**Response**

The method does not return a body.

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### PUT

The `PUT` method is used to update a node.

This method has no parameters.

**Request**

``` none
PUT /organizations/NAME/nodes/NAME
```

with a request body similar to:

``` javascript
{
  "overrides": {},
  "name": "latte",
  "chef_type": "node",
  "json_class": "Chef::Node",
  "attributes": {
    "hardware_type": "laptop"
  },
  "run_list": [
    'recipe[cookbook_name::recipe_name],
    role[role_name]'
  ],
  "defaults": {}
}
```

**Response**

The response will return the updated node.

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
<tr class="odd">
<td><code>413</code></td>
<td>Request entity too large. A request may not be larger than 1000000 bytes.</td>
</tr>
</tbody>
</table>

### /policies

The `/policies` endpoint has the following methods: `GET`.

#### GET

The `GET` method is used to get a list of policies (including policy
revisions) from the Chef Infra Server.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/policies
```

**Response**

The response groups policies by name and revision and is similar to:

``` javascript
{
  "aar": {
    "uri": "https://chef.example/organizations/org1/policies/aar",
    "revisions": {
      "37f9b658cdd1d9319bac8920581723efcc2014304b5f3827ee0779e10ffbdcc9": {
      },
      "95040c199302c85c9ccf1bcc6746968b820b1fa25d92477ea2ec5386cd58b9c5": {
      },
      "d81e80ae9bb9778e8c4b7652d29b11d2111e763a840d0cadb34b46a8b2ca4347": {
      }
    }
  },
  "jenkins": {
    "uri": "https://chef.example/organizations/org1/policies/jenkins",
    "revisions": {
      "613f803bdd035d574df7fa6da525b38df45a74ca82b38b79655efed8a189e073": {
      },
      "6fe753184c8946052d3231bb4212116df28d89a3a5f7ae52832ad408419dd5eb": {
      },
      "cc1a0801e75df1d1ea5b0d2c71ba7d31c539423b81478f65e6388b9ee415ad87": {
      }
    }
  }
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
</tbody>
</table>

### /policy_groups

The `/policy_groups` endpoint has the following methods: `GET`.

Each node has a 1:many relationship with policy settings stored on the
Chef Infra Server. This relationship is based on the policy group to
which the node is associated, and then the policy settings assigned to
that group:

-   A policy is typically named after the functional role ahost
    performs, such as "application server", "chat server", "load
    balancer", and so on
-   A policy group defines a set of hosts in a deployed units, typically
    mapped to organizational requirements such as "dev", "test",
    "staging", and "production", but can also be mapped to more detailed
    requirements as needed

### /principals/NAME

The `/principals/NAME` endpoint has the following methods: `GET`.

#### GET

The `GET` method is used to get a list of public keys for clients and
users in order to ensure that enough information is present for
authorized requests.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/principals/NAME
```

**Response**

For a user or client, the type value will vary. The response body
returns an array of principals which allows for a client with the
same name as a user. The response for a user or client is similar to:

``` javascript
{
  "Principals: [
    {
      "name": "normal_user",
      "type": "user",
      "public_key": "-----BEGIN PUBLIC KEY-----...",
      "authz_id": "eca5fdd45a8b4bacc04bbc6e37a340bes",
      "org_member":false
    }
  ]
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

### /required_recipe

The `/required_recipe` endpoint has the following method: `GET`.

#### GET

Use the `GET` method to view a recipe specified by a
Chef Infra Server administrator as part of the Chef Infra Server configuration.
This recipe will be run by all Chef Infra Clients that connect to the Chef Infra Server.
The `required_recipe` feature is aimed at expert level practioners delivering
isolated configuration changes to target systems. The returned text is
the content of a single recipe file.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/required_recipe
```

This method has no request body.

**Response**

The response is returned in plain text, not in JSON format. The response is similar to:

``` javascript
  "# My required recipe
   file '/tmp/build'
   package 'secret_sauce'
  "
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful. Required recipe is enabled, a path to a recipe is defined, and a recipe exists at the path location.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not Found. The required recipe function is not enabled.</td>
</tr>
</tbody>
</table>

### /roles

{{% role %}}

The `/roles` endpoint has the following methods: `GET` and `POST`.

#### GET

The `GET` method is used to get a list of roles along with their
associated URIs.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/roles
```

**Response**

The response is similar to:

``` javascript
{
  "webserver": "https://chef.example/organizations/org1/roles/webserver"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
</tbody>
</table>

#### POST

The `POST` method is used to create a new role on the Chef Infra Server.

This method has no parameters.

**Request**

``` none
POST /organizations/NAME/roles
```

with a request body similar to:

``` javascript
{
  "name": "webserver",
  "default_attributes": {},
  "description": "A webserver",
  "env_run_lists": {
    "testenv": {
      "recipe[pegasus]"
    }
  },
  "run_list": [
    "recipe[unicorn]",
    "recipe[apache2]"
  ],
  "override_attributes": {}
}
```

**Response**

The response is similar to:

``` javascript
{ "uri": "https://chef.example/organizations/org1/roles/webserver" }
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>201</code></td>
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>400</code></td>
<td>Bad request. The contents of the request are not formatted correctly.</td>
</tr>
<tr class="odd">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="even">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="odd">
<td><code>409</code></td>
<td>Conflict. The object already exists.</td>
</tr>
<tr class="even">
<td><code>413</code></td>
<td>Request entity too large. A request may not be larger than 1000000 bytes.</td>
</tr>
</tbody>
</table>

### /roles/NAME

The `/roles/NAME` endpoint has the following methods: `GET`, `DELETE`,
and `PUT`.

#### DELETE

The `DELETE` method is used to delete a role on the Chef Infra Server.

This method has no parameters.

**Request**

``` none
DELETE /organizations/NAME/roles/NAME
```

**Response**

The response is similar to:

``` javascript
{
  "name": "webserver",
  "chef_type": "role",
  "json_class": "Chef::Role",
  "default_attributes": {},
  "description": "A webserver",
  "env_run_lists": {
    "env1": {
      "recipe[foo1]"
    }
  },
  "run_list": [
    "recipe[apache2]"
  ],
  "override_attributes": {}
}
```

**Response Codes**


<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### GET

The `GET` method is used to return the details for a role as JSON.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/roles/NAME
```

**Response**

The response is similar to:

``` javascript
{
  "name": "webserver",
  "chef_type": "role",
  "json_class": "Chef::Role",
  "default_attributes": {},
  "description": "A webserver",
  "env_run_lists": {},
  "run_list": [
    "recipe[unicorn]",
    "recipe[apache2]"
  ],
  "override_attributes": {}
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### PUT

The `PUT` method is used to update a role on the Chef Infra Server.

This method has no parameters.

**Request**

``` none
PUT /organizations/NAME/roles/NAME
```

with a request body similar to:

``` javascript
{
  "name": "webserver",
  "default_attributes": {},
  "description": "A webserver",
  "env_run_lists": {},
  "default_attributes": {},
  "run_list": [
    "recipe[apache2]"
  ],
  "override_attributes": {}
}
```

**Response**

The response will return the JSON for the updated role.

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
<tr class="odd">
<td><code>413</code></td>
<td>Request entity too large. A request may not be larger than 1000000 bytes.</td>
</tr>
</tbody>
</table>

### /roles/NAME/environments

The `/roles/NAME/environments` endpoint has the following method: `GET`.

#### GET

The `GET` method returns a list of the environments that have
environment-specific run-lists in the given role as JSON data.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/roles/NAME/environments
```

**Response**

The response is similar to:

``` javascript
["_default","production","qa"]
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

### /roles/NAME/environments/NAME

The `/roles/NAME/environments/NAME` endpoint has the following method:
`GET`.

#### GET

The `GET` method returns the environment-specific run-list
(`env_run_lists[environment_name]`) for a role.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/roles/NAME/environments/NAME
```

where the first `NAME` is the name of the role and the second is the
name of the environment.

**Response**

The response is similar to:

``` javascript
{"run_list":["recipe[foo]"]}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

### /sandboxes

A sandbox is used to commit files so they only need to be updated one
time, as opposed to every time a cookbook is uploaded. The `/sandboxes`
endpoint has the following methods: `POST`.

#### POST

The `POST` method is used to create a new sandbox. This method accepts a
list of checksums as input and returns the URLs against which to `PUT`
files that need to be uploaded.

This method has no parameters.

**Request**

``` none
POST /organizations/NAME/sandboxes
```

with a request body similar to:

``` javascript
{"checksums": {
  "385ea5490c86570c7de71070bce9384a":null,
  "f6f73175e979bd90af6184ec277f760c":null,
  "2e03dd7e5b2e6c8eab1cf41ac61396d5":null
  }
}
```

**Response**

The response is similar to:

``` javascript
{"uri":
 "https://api.opscode.com/organizations/testorg/sandboxes/eff7b6f8b3ef44c6867216662d5eeb5f",
 "checksums":
   {"385ea5490c86570c7de71070bce9384a":
     {"url":
      "https://s3.amazonaws.com/opscode-platform-production-data/organization-(...)",
       "needs_upload":true},
       "f6f73175e979bd90af6184ec277f760c"=>
     {"url":
       "https://s3.amazonaws.com/opscode-platform-production-data/organization-(...)",
       "needs_upload":true},
       "2e03dd7e5b2e6c8eab1cf41ac61396d5":
     {"url":
       "https://s3.amazonaws.com/opscode-platform-production-data/organization-(...)",
       "needs_upload":true}
   },
 "sandbox_id"=>"eff7b6f8b3ef44c6867216662d5eeb5f"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful. A hash that maps each checksum to a hash that contains a boolean <code>needs_upload</code> field and a URL if <code>needs_upload</code> is set to <code>true</code>.</td>
</tr>
<tr class="even">
<td><code>400</code></td>
<td>Bad request. The object has already been committed or one (or more) of the objects were not properly uploaded. The payload does not contain a well-formed <code>checksums</code> parameter that is a hash containing a key for each checksum.</td>
</tr>
<tr class="odd">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="even">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="odd">
<td><code>413</code></td>
<td>Request entity too large. A request may not be larger than 1000000 bytes.</td>
</tr>
</tbody>
</table>

### /sandboxes/ID

Each sandbox has a unique identifier. The `/sandboxes/ID` endpoint has
the following methods: `PUT`.

#### PUT

The `PUT` method is used to commit files that are in a sandbox to their
final location so that changes to cookbooks will not require
re-uploading the same data.

This method has no parameters.

**Request**

``` none
PUT /organizations/NAME/sandboxes/ID
```

with a request body similar to:

``` javascript
{"is_completed":true}
```

**Response**

The response is similar to:

``` javascript
{
  "guid": guid,
  "name": guid,
  "checksums":
    {"385ea5490c86570c7de71070bce9384a":
    {"url":
      "https://s3.amazonaws.com/opscode-platform-production-data/organization-(...)",
      "needs_upload":true}
  },
  "create_time": <get an example of time format>,
  "is_completed": true
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>400</code></td>
<td>Bad request. The contents of the request are not formatted correctly.</td>
</tr>
<tr class="odd">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="even">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="odd">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
<tr class="even">
<td><code>413</code></td>
<td>Request entity too large. A request may not be larger than 1000000 bytes.</td>
</tr>
</tbody>
</table>

### /search

{{% search %}}

The `/search` endpoint allows nodes, roles, data bags, environments to
be searched. This endpoint has the following methods: `GET`.

{{< note >}}

At the end of every Chef Infra Client run, the node object is saved to
the Chef Infra Server. From the Chef Infra Server, each node object is
then added to the Apache Solr search index. This process is
asynchronous. By default, node objects are committed to the search index
every 60 seconds or per 1000 node objects, whichever occurs first.

{{< /note >}}

{{< note >}}

This endpoint does not have any ACL restrictions, which means it may be
used by any user or client that is able to make the request to the Chef
Infra Server.

{{< /note >}}

#### GET

The `GET` method is used to return a data structure that contains links
to each available search index. By default, the `role`, `node`,
`client`, and `data bag` indexes will always be available (where the
`data bag` index is the name of the data bag on the Chef Infra Server).
Search indexes may lag behind the most current data at any given time.
If a situation occurs where data needs to be written and then
immediately searched, an artificial delay (of at least 10 seconds) is
recommended.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/search
```

This method has no request body.

**Response**

The response is similar to:

``` javascript
{
  "node": "https://chef.example/organizations/org1/search/node",
  "role": "https://chef.example/organizations/org1/search/role",
  "client": "https://chef.example/organizations/org1/search/client",
  "users": "https://chef.example/organizations/org1/search/users"
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
</tbody>
</table>

### /search/INDEX

Use the `/search/INDEX` endpoint to access the search indexes on the
Chef Infra Server. The `/search/INDEX` endpoint has the following
methods: `GET` and `POST`.

{{% search_query_syntax %}}

#### GET

The `GET` method is used to return all of the data that matches the
query in the `GET` request.

This method has the following parameters:

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>The search query used to identify a list of items on a Chef Infra Server. This option uses the same syntax as the <code>knife search</code> subcommand.</td>
</tr>
<tr class="even">
<td><code>rows</code></td>
<td>The number of rows to be returned.</td>
</tr>
<tr class="odd">
<td><code>start</code></td>
<td>The row at which return results begin.</td>
</tr>
</tbody>
</table>

**Request**

``` none
GET /organizations/NAME/search/INDEX
```

**Response**

The response contains the total number of rows that match the request
and for a node index search is similar to:

``` javascript
{
 "total": 1,
 "start": 0,
 "rows": [
    {
     "automatic": {"hardware_type": "laptop"},
     "chef_environment": "_default",
     "chef_type": "node",
     "default": {}
     "json_class": "Chef::Node",
     "name": "latte",
     "normal": {},
     "override": {"hardware_type": "laptop"},
     "run_list": ["recipe[unicorn]"]
    }
  ]
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### POST

A partial search query allows a search query to be made against specific
attribute keys that are stored on the Chef Infra Server. A partial
search query can search the same set of objects on the Chef Infra Server
as a full search query, including specifying an object index and
providing a query that can be matched to the relevant index. While a
full search query will return an array of objects that match (each
object containing a full set of attributes for the node), a partial
search query will return only the values for the attributes that match.
One primary benefit of using a partial search query is that it requires
less memory and network bandwidth while Chef Infra Client processes the
search results. The attributes to be returned by the partial search
are specified in the request JSON body.

This method has the following parameters:

<table>
<colgroup>
<col style="width: 20%" />
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
<td><code>rows</code></td>
<td>The number of rows to be returned.</td>
</tr>
<tr class="odd">
<td><code>start</code></td>
<td>The row at which return results begin.</td>
</tr>
</tbody>
</table>

**Request**

``` none
POST /organizations/NAME/search
```

with a request body similar to:

``` none
{
  "name": [ "name" ],
  "ip": [ "ipaddress" ],
  "kernel_version": [ "kernel", "version" ]
}
```

**Response**

The response is similar to:

``` javascript
{
  "total":1,
  "start":0,
  "rows": [
    {
      "url": "https://chef.example/organization/org1/nodes/latte",
      "data": {
        "name": "latte",
        "ip": "123.4.5.6789",
        "kernel_version": {"linux": "1.2.3"}
      }
    }
  ]
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>413</code></td>
<td>Request entity too large. A request may not be larger than 1000000 bytes.</td>
</tr>
</tbody>
</table>

### /universe

Use the `/universe` endpoint to retrieve the known collection of
cookbooks, and then use it with Berkshelf and Chef Supermarket.

The `/universe` endpoint has the following methods: `GET`.

#### GET

The `GET` method is used to retrieve the universe data.

This method has no parameters. This method has no request body.

**Request**

``` none
GET /universe
```

**Response**

The response will return a json hash, with the name of each cookbook as
a top-level key. Each cookbook will list each version, along with its
location information and dependencies:

``` javascript
{
  "ffmpeg": {
    "0.1.0": {
      "location_path": "http://supermarket.chef.io/api/v1/cookbooks/ffmpeg/0.1.0/download",
      "location_type": "supermarket",
      "dependencies": {
        "git": ">= 0.0.0",
        "build-essential": ">= 0.0.0",
        "libvpx": "~> 0.1.1",
        "x264": "~> 0.1.1"
      }
    },
    "0.1.1": {
      "location_path": "http://supermarket.chef.io/api/v1/cookbooks/ffmpeg/0.1.1/download",
      "location_type": "supermarket",
      "dependencies": {
        "git": ">= 0.0.0",
        "build-essential": ">= 0.0.0",
        "libvpx": "~> 0.1.1",
        "x264": "~> 0.1.1"
      }
    }
  },
  "pssh": {
    "0.1.0": {
      "location_path": "http://supermarket.chef.io/api/v1/cookbooks/pssh.1.0/download",
      "location_type": "supermarket",
      "dependencies": {}
    }
  }
}
```

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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

### /updated_since

The `/updated_since` endpoint ensures that replica instances of the Chef
Infra Server are able to synchronize with the primary Chef Infra Server.
`/updated_since` was part of the Chef Replication product. Chef Replication
is no longer available as a product and the `/updated_since` endpoint
is also deprecated. The expectation for almost all chef users is that
use of the endpoint will return an http status of 404.
The `/organizations/NAME/updated_since` endpoint has the following
methods: `GET`.

{{< warning >}}

This update is available after Chef replication is installed on the Chef
Infra Server.

{{< /warning >}}

#### GET

The `GET` method is used to return the details of an organization as
JSON.

**Request**

``` none
GET /organizations/NAME/updated_since?seq=NUM
```

where `NUM` is the largest integer previously returned as an identifier.

**Response**

The response will return an array of paths for objects that have been
created, updated, or deleted since `NUM`, similar to:

``` javascript
[
  {
    "action": "create",
    "id": 1,
    "path": "/roles/foo"
  },
  {
    "action": "create",
    "id": 2,
    "path": "/roles/foo2"
  },
  {
    "action": "create",
    "id": 3,
    "path": "/roles/foo3"
  },
  {
    "action": "update",
    "id": 4,
    "path": "/roles/foo3"
  }
]
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name, and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist or the function is not implemented.</td>
</tr>
</tbody>
</table>

### /users

A user may be associated with an organization.

The `/users` endpoint has the following methods: `GET` and `POST`.

#### GET

The `GET` method is used to return an array of usernames for users
associated with an organization.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/users
```

This method has no request body.

**Response**

The response is similar to:

``` javascript
[
  { "user": { "username": "paperlatte" } }
]
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
</tbody>
</table>

#### POST

The `POST` method is used to associate a user with an organization
immediately. Superuser only.

This method has no parameters.

**Request**

``` none
POST /organizations/NAME/users
```

with a request body similar to:

``` javascript
{
  "username": "paperlatte",
}
```

where `username` is the name of the user to be associated.

**Response**

No response block is returned.

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
</colgroup>
<thead>
<tr class="header">
<th>Response Code</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>201</code></td>
<td>Created. The user was associated with the organization.</td>
</tr>
<tr class="even">
<td><code>400</code></td>
<td>Bad request. The contents of the request are not formatted correctly.</td>
</tr>
<tr class="odd">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name and that the correct key was used to sign the request.</td>
</tr>
<tr class="even">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="odd">
<td><code>409</code></td>
<td>Conflict. The user is already associated.</td>
</tr>
</tbody>
</table>

### /users/NAME

The `/users/NAME` endpoint has the following methods: `DELETE`, `GET`.

#### DELETE

The `DELETE` method is used to delete a user association with an
organization.

This method has no parameters.

**Request**

``` none
DELETE /organizations/NAME/users/NAME
```

**Response**

The response will return the end state of the user, similar to:

``` javascript
{
  "username": "paperlatte"
  "email": "latte",
  "display_name": "Ms. Latte",
  "first_name": "Paper",
  "last_name": "Latte",
  "public_key": "-----BEGIN PUBLIC KEY----- ... "
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful. The user association was removed.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

#### GET

The `GET` method is used to return the details of a user as JSON.

This method has no parameters.

**Request**

``` none
GET /organizations/NAME/users/NAME
```

**Response**

The response is similar to:

``` javascript
{
  "username": "paperlatte"
  "email": "latte",
  "display_name": "Ms. Latte",
  "first_name": "Paper",
  "last_name": "Latte",
  "public_key": "-----BEGIN PUBLIC KEY----- ... "
}
```

**Response Codes**

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
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
<td>OK. The request was successful.</td>
</tr>
<tr class="even">
<td><code>401</code></td>
<td>Unauthorized. The user or client who made the request could not be authenticated. Verify the user/client name and that the correct key was used to sign the request.</td>
</tr>
<tr class="odd">
<td><code>403</code></td>
<td>Forbidden. The user who made the request is not authorized to perform the action.</td>
</tr>
<tr class="even">
<td><code>404</code></td>
<td>Not found. The requested object does not exist.</td>
</tr>
</tbody>
</table>

## Examples

The following sections show examples of using the Chef Infra Server API.

### Query for Users and Orgs

The following example shows how to query the Chef Infra Server API for a
listing of organizations and users. The `/organizations` and `/users`
endpoints may only be accessed by the `pivotal` user, which is a user
account that is created by Chef during the installation of the Chef
Infra Server.

Run the following from a `.chef` directory that contains a `pivotal.rb`
file:

``` ruby
require 'chef'
require 'chef/server_api'

Chef::Config.from_file(".chef/pivotal.rb")
rest = Chef::ServerAPI.new(Chef::Config[:chef_server_url])
orgs = rest.get("/organizations")

puts "\n=== Listing of organizations"
orgs.each do |org|
  puts org
end

puts "\n=== Listing of Users"
users = rest.get("/users")
users.each do |user|
  puts user
end
```

An example of a `.chef/pivotal.rb` file is shown below:

``` ruby
current_dir = File.dirname(__FILE__)
node_name "pivotal"
chef_server_url "https://192.0.2.0:443"
chef_server_root "https://192.0.2.0:443"
client_key "#{current_dir}/pivotal.pem"
```

{{< note >}}

The `pivotal.pem` file must exist in the specified location and the IP
addresses must be correct for the Chef Infra Server.

{{< /note >}}
