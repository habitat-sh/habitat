Use the `render` method in a template to reference a partial template
file:

``` ruby
<%= render 'partial_name.txt.erb', :option => {} %>
```

where `partial_name` is the name of the partial template file and
`:option` is one (or more) of the following:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 87%" />
</colgroup>
<thead>
<tr class="header">
<th>Option</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>:cookbook</code></td>
<td>By default, a partial template file is assumed to be located in the cookbook that contains the top-level template. Use this option to specify the path to a different cookbook</td>
</tr>
<tr class="even">
<td><code>:local</code></td>
<td>Indicates that the name of the partial template file should be interpreted as a path to a file in the local file system or looked up in a cookbook using the normal rules for template files. Set to <code>true</code> to interpret as a path to a file in the local file system and to <code>false</code> to use the normal rules for template files</td>
</tr>
<tr class="odd">
<td><code>:source</code></td>
<td>By default, a partial template file is identified by its file name. Use this option to specify a different name or a local path to use (instead of the name of the partial template file)</td>
</tr>
<tr class="even">
<td><code>:variables</code></td>
<td>A hash of <code>variable_name =&gt; value</code> that will be made available to the partial template file. When this option is used, any variables that are defined in the top-level template that are required by the partial template file must have them defined explicitly using this option</td>
</tr>
</tbody>
</table>

For example:

``` ruby
<%= render 'simple.txt.erb', :variables => {:user => Etc.getlogin }, :local => true %>
```