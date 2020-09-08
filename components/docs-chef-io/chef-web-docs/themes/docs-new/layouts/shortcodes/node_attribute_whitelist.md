<div class="admonition-warning"><p class="admonition-warning-title">Warning</p><div class="admonition-warning-text">

When attribute whitelist settings are used, only the attributes defined
in a whitelist will be saved and any attribute that is not defined in a
whitelist will not be saved. Each attribute type is whitelisted
independently of the other attribute types. For example, if
`automatic_attribute_whitelist` defines attributes to be saved, but
`normal_attribute_whitelist`, `default_attribute_whitelist`, and
`override_attribute_whitelist` are not defined, then all normal
attributes, default attributes, and override attributes are saved, as
well as the automatic attributes that were specifically included through
whitelisting.

</div></div>

Attributes that should be saved by a node may be whitelisted in the
client.rb file. The whitelist is a hash of keys that specifies each
attribute to be saved.

Attributes are whitelisted by attribute type, with each attribute type
being whitelisted independently. Each attribute type---`automatic`,
`default`, `normal`, and `override`---may define whitelists by using the
following settings in the client.rb file:

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Setting</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>automatic_attribute_whitelist</code></td>
<td>A hash that whitelists <code>automatic</code> attributes, preventing non-whitelisted attributes from being saved. For example: <code>['network/interfaces/eth0']</code>. Default value: <code>nil</code>, all attributes are saved. If the hash is empty, no attributes are saved.</td>
</tr>
<tr class="even">
<td><code>default_attribute_whitelist</code></td>
<td>A hash that whitelists <code>default</code> attributes, preventing non-whitelisted attributes from being saved. For example: <code>['filesystem/dev/disk0s2/size']</code>. Default value: <code>nil</code>, all attributes are saved. If the hash is empty, no attributes are saved.</td>
</tr>
<tr class="odd">
<td><code>normal_attribute_whitelist</code></td>
<td>A hash that whitelists <code>normal</code> attributes, preventing non-whitelisted attributes from being saved. For example: <code>['filesystem/dev/disk0s2/size']</code>. Default value: <code>nil</code>, all attributes are saved. If the hash is empty, no attributes are saved.</td>
</tr>
<tr class="even">
<td><code>override_attribute_whitelist</code></td>
<td>A hash that whitelists <code>override</code> attributes, preventing non-whitelisted attributes from being saved. For example: <code>['map - autohome/size']</code>. Default value: <code>nil</code>, all attributes are saved. If the hash is empty, no attributes are saved.</td>
</tr>
</tbody>
</table>

<div class="admonition-warning">

<p class="admonition-warning-title">Warning</p>

<div class="admonition-warning-text">

The recommended practice is to only use `automatic_attribute_whitelist`
to whitelist attributes. This is primarily because automatic attributes
generate the most data, but also that normal, default, and override
attributes are typically much more important attributes and are more
likely to cause issues if they are whitelisted incorrectly.



</div>

</div>

For example, automatic attribute data similar to:

``` javascript
{
  "filesystem" => {
    "/dev/disk0s2" => {
      "size" => "10mb"
    },
    "map - autohome" => {
      "size" => "10mb"
    }
  },
  "network" => {
    "interfaces" => {
      "eth0" => {...},
      "eth1" => {...},
    }
  }
}
```

To whitelist the `network` attributes and prevent the other attributes
from being saved, update the client.rb file:

``` ruby
automatic_attribute_whitelist ['network/interfaces/']
```

When a whitelist is defined, any attribute of that type that is not
specified in that attribute whitelist **will not** be saved. So based on
the previous whitelist for automatic attributes, the `filesystem` and
`map - autohome` attributes will not be saved, but the `network`
attributes will.

Leave the value empty to prevent all attributes of that attribute type
from being saved:

``` ruby
automatic_attribute_whitelist []
```

For attributes that contain slashes (`/`) within the attribute value,
such as the `filesystem` attribute `'/dev/diskos2'`, use an array. For
example:

``` ruby
automatic_attribute_whitelist [['filesystem','/dev/diskos2']]
```