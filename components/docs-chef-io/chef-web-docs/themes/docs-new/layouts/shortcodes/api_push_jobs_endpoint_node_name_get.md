The `GET` method is used to get the status (`up` or `down`) for an
individual node.

This method has no parameters.

**Request**

``` xml
GET /organizations/ORG_NAME/pushy/node_states/NODE_NAME
```

**Response**

The response is similar to:

``` javascript
{
  "node_name": "FIONA",
  "status": "down",
  "updated_at": "Tue, 04 Sep 2012 23:17:56 GMT"
}
```

where `updated_at` shows the date and time at which a node's status last
changed.

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
</tbody>
</table>