A knife plugin can be used to make authenticated API requests to the
Chef Infra Server using the following methods:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 87%" />
</colgroup>
<thead>
<tr class="header">
<th>Method</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>rest.delete_rest</code></td>
<td>Use to delete an object from the Chef Infra Server.</td>
</tr>
<tr class="even">
<td><code>rest.get_rest</code></td>
<td>Use to get the details of an object on the Chef Infra Server.</td>
</tr>
<tr class="odd">
<td><code>rest.post_rest</code></td>
<td>Use to add an object to the Chef Infra Server.</td>
</tr>
<tr class="even">
<td><code>rest.put_rest</code></td>
<td>Use to update an object on the Chef Infra Server.</td>
</tr>
</tbody>
</table>

For example:

``` ruby
module MyCommands
  class MyNodeDelete < Chef::Knife
    #An implementation of knife node delete
    banner 'knife my node delete [NODE_NAME]'

    def run
      if name_args.length < 1
        show_usage
        ui.fatal("You must specify a node name.")
        exit 1
      end
      nodename = name_args[0]
      api_endpoint = "nodes/#{nodename}"
      # Again, we could just call rest.delete_rest
      nodey = rest.get_rest(api_endpoint)
      ui.confirm("Do you really want to delete #{nodey}")
      nodey.destroy
    end
  end
end
```