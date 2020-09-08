knife can encrypt and decrypt data bag items when the `knife data bag`
subcommand is run with the `create`, `edit`, `from file`, or `show`
arguments and the following options:

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Option</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>--secret SECRET</code></td>
<td>The encryption key that is used for values contained within a data bag item. If <code>secret</code> is not specified, Chef Infra Client looks for a secret at the path specified by the <code>encrypted_data_bag_secret</code> setting in the client.rb file.</td>
</tr>
<tr class="even">
<td><code>--secret-file FILE</code></td>
<td>The path to the file that contains the encryption key.</td>
</tr>
</tbody>
</table>