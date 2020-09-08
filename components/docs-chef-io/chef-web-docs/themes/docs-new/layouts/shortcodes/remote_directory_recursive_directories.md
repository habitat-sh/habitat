The **remote_directory** resource can be used to recursively create the
path outside of remote directory structures, but the permissions of
those outside paths are not managed. This is because the `recursive`
attribute only applies `group`, `mode`, and `owner` attribute values to
the remote directory itself and any inner directories the resource
copies.

A directory structure:

    /foo
      /bar
        /baz

The following example shows a way create a file in the `/baz` directory:

``` ruby
remote_directory "/foo/bar/baz" do
  owner 'root'
  group 'root'
  mode '0755'
  action :create
end
```

But with this example, the `group`, `mode`, and `owner` attribute values
will only be applied to `/baz`. Which is fine, if that's what you want.
But most of the time, when the entire `/foo/bar/baz` directory structure
is not there, you must be explicit about each directory. For example:

``` ruby
%w[ /foo /foo/bar /foo/bar/baz ].each do |path|
  remote_directory path do
    owner 'root'
    group 'root'
    mode '0755'
  end
end
```

This approach will create the correct hierarchy---`/foo`, then `/bar` in
`/foo`, and then `/baz` in `/bar`---and also with the correct attribute
values for `group`, `mode`, and `owner`.