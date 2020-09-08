The `rights` property can be used in a recipe to manage access control
lists (ACLs), which allow permissions to be given to multiple users and
groups. Use the `rights` property can be used as many times as
necessary; Chef Infra Client will apply them to the file or directory as
required. The syntax for the `rights` property is as follows:

``` ruby
rights permission, principal, option_type => value
```

where

`permission`

:   Use to specify which rights are granted to the `principal`. The
    possible values are: `:read`, `:write`, `read_execute`, `:modify`,
    and `:full_control`.

    These permissions are cumulative. If `:write` is specified, then it
    includes `:read`. If `:full_control` is specified, then it includes
    both `:write` and `:read`.

    (For those who know the Microsoft Windows API: `:read` corresponds
    to `GENERIC_READ`; `:write` corresponds to `GENERIC_WRITE`;
    `:read_execute` corresponds to `GENERIC_READ` and `GENERIC_EXECUTE`;
    `:modify` corresponds to `GENERIC_WRITE`, `GENERIC_READ`,
    `GENERIC_EXECUTE`, and `DELETE`; `:full_control` corresponds to
    `GENERIC_ALL`, which allows a user to change the owner and other
    metadata about a file.)

`principal`

:   Use to specify a group or user name. This is identical to what is
    entered in the login box for Microsoft Windows, such as `user_name`,
    `domain\user_name`, or `user_name@fully_qualified_domain_name`. Chef
    Infra Client does not need to know if a principal is a user or a
    group.

`option_type`

:   A hash that contains advanced rights options. For example, the
    rights to a directory that only applies to the first level of
    children might look something like:
    `rights :write, 'domain\group_name', :one_level_deep => true`.
    Possible option types:

    <table>
    <colgroup>
    <col style="width: 12%" />
    <col style="width: 87%" />
    </colgroup>
    <thead>
    <tr class="header">
    <th>Option Type</th>
    <th>Description</th>
    </tr>
    </thead>
    <tbody>
    <tr class="odd">
    <td><code>:applies_to_children</code></td>
    <td>Specify how permissions are applied to children. Possible values: <code>true</code> to inherit both child directories and files; <code>false</code> to not inherit any child directories or files; <code>:containers_only</code> to inherit only child directories (and not files); <code>:objects_only</code> to recursively inherit files (and not child directories).</td>
    </tr>
    <tr class="even">
    <td><code>:applies_to_self</code></td>
    <td>Indicates whether a permission is applied to the parent directory. Possible values: <code>true</code> to apply to the parent directory or file and its children; <code>false</code> to not apply only to child directories and files.</td>
    </tr>
    <tr class="odd">
    <td><code>:one_level_deep</code></td>
    <td>Indicates the depth to which permissions will be applied. Possible values: <code>true</code> to apply only to the first level of children; <code>false</code> to apply to all children.</td>
    </tr>
    </tbody>
    </table>

For example:

``` ruby
resource 'x.txt' do
  rights :read, 'Everyone'
  rights :write, 'domain\group'
  rights :full_control, 'group_name_or_user_name'
  rights :full_control, 'user_name', :applies_to_children => true
end
```

or:

``` ruby
rights :read, ['Administrators','Everyone']
rights :full_control, 'Users', :applies_to_children => true
rights :write, 'Sally', :applies_to_children => :containers_only, :applies_to_self => false, :one_level_deep => true
```

Some other important things to know when using the `rights` attribute:

-   Only inherited rights remain. All existing explicit rights on the
    object are removed and replaced.
-   If rights are not specified, nothing will be changed. Chef Infra
    Client does not clear out the rights on a file or directory if
    rights are not specified.
-   Changing inherited rights can be expensive. Microsoft Windows will
    propagate rights to all children recursively due to inheritance.
    This is a normal aspect of Microsoft Windows, so consider the
    frequency with which this type of action is necessary and take steps
    to control this type of action if performance is the primary
    consideration.

Use the `deny_rights` property to deny specific rights to specific
users. The ordering is independent of using the `rights` property. For
example, it doesn't matter if rights are granted to everyone is placed
before or after `deny_rights :read, ['Julian', 'Lewis']`, both Julian
and Lewis will be unable to read the document. For example:

``` ruby
resource 'x.txt' do
  rights :read, 'Everyone'
  rights :write, 'domain\group'
  rights :full_control, 'group_name_or_user_name'
  rights :full_control, 'user_name', :applies_to_children => true
  deny_rights :read, ['Julian', 'Lewis']
end
```

or:

``` ruby
deny_rights :full_control, ['Sally']
```