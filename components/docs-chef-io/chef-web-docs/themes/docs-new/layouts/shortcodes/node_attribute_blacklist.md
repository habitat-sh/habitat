<div class="admonition-warning"><p class="admonition-warning-title">Warning</p><div class="admonition-warning-text">

When attribute blacklist settings are used, any attribute defined in a
blacklist will not be saved and any attribute that is not defined in a
blacklist will be saved. Each attribute type is blacklisted
independently of the other attribute types. For example, if
`automatic_attribute_blacklist` defines attributes that will not be
saved, but `normal_attribute_blacklist`, `default_attribute_blacklist`,
and `override_attribute_blacklist` are not defined, then all normal
attributes, default attributes, and override attributes will be saved,
as well as the automatic attributes that were not specifically excluded
through blacklisting.

</div></div>

Attributes that should not be saved by a node may be blacklisted in the
client.rb file. The blacklist is a Hash of keys that specify each
attribute to be filtered out.

Attributes are blacklisted by attribute type, with each attribute type
being blacklisted independently. Each attribute type---`automatic`,
`default`, `normal`, and `override`---may define blacklists by using the
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
<td><code>automatic_attribute_blacklist</code></td>
<td>A hash that blacklists <code>automatic</code> attributes, preventing blacklisted attributes from being saved. For example: <code>['network/interfaces/eth0']</code>. Default value: <code>nil</code>, all attributes are saved. If the array is empty, all attributes are saved.</td>
</tr>
<tr class="even">
<td><code>default_attribute_blacklist</code></td>
<td>A hash that blacklists <code>default</code> attributes, preventing blacklisted attributes from being saved. For example: <code>['filesystem/dev/disk0s2/size']</code>. Default value: <code>nil</code>, all attributes are saved. If the array is empty, all attributes are saved.</td>
</tr>
<tr class="odd">
<td><code>normal_attribute_blacklist</code></td>
<td>A hash that blacklists <code>normal</code> attributes, preventing blacklisted attributes from being saved. For example: <code>['filesystem/dev/disk0s2/size']</code>. Default value: <code>nil</code>, all attributes are saved. If the array is empty, all attributes are saved.</td>
</tr>
<tr class="even">
<td><code>override_attribute_blacklist</code></td>
<td>A hash that blacklists <code>override</code> attributes, preventing blacklisted attributes from being saved. For example: <code>['map - autohome/size']</code>. Default value: <code>nil</code>, all attributes are saved. If the array is empty, all attributes are saved.</td>
</tr>
</tbody>
</table>

<div class="admonition-warning">

<p class="admonition-warning-title">Warning</p>

<div class="admonition-warning-text">

The recommended practice is to use only `automatic_attribute_blacklist`
for blacklisting attributes. This is primarily because automatic
attributes generate the most data, but also that normal, default, and
override attributes are typically much more important attributes and are
more likely to cause issues if they are blacklisted incorrectly.



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

To blacklist the `filesystem` attributes and allow the other attributes
to be saved, update the client.rb file:

``` ruby
automatic_attribute_blacklist ['filesystem']
```

When a blacklist is defined, any attribute of that type that is not
specified in that attribute blacklist **will** be saved. So based on the
previous blacklist for automatic attributes, the `filesystem` and
`map - autohome` attributes will not be saved, but the `network`
attributes will.

For attributes that contain slashes (`/`) within the attribute value,
such as the `filesystem` attribute `'/dev/diskos2'`, use an array. For
example:

``` ruby
automatic_attribute_blacklist [['filesystem','/dev/diskos2']]
```