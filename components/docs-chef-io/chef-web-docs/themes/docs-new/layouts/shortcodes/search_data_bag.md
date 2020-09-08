Any search for a data bag (or a data bag item) must specify the name of
the data bag and then provide the search query string that will be used
during the search. For example, to use knife to search within a data bag
named "admin_data" across all items, except for the "admin_users"
item, enter the following:

``` bash
knife search admin_data "(NOT id:admin_users)"
```

Or, to include the same search query in a recipe, use a code block
similar to:

``` ruby
search(:admin_data, "NOT id:admin_users")
```

It may not be possible to know which data bag items will be needed. It
may be necessary to load everything in a data bag (but not know what
"everything" is). Using a search query is the ideal way to deal with
that ambiguity, yet still ensure that all of the required data is
returned. The following examples show how a recipe can use a series of
search queries to search within a data bag named "admins". For example,
to find every administrator:

``` ruby
search(:admins, "*:*")
```

Or to search for an administrator named "charlie":

``` ruby
search(:admins, "id:charlie")
```

Or to search for an administrator with a group identifier of "ops":

``` ruby
search(:admins, "gid:ops")
```

Or to search for an administrator whose name begins with the letter "c":

``` ruby
search(:admins, "id:c*")
```

Data bag items that are returned by a search query can be used as if
they were a hash. For example:

``` ruby
charlie = search(:admins, "id:charlie").first
# => variable 'charlie' is set to the charlie data bag item
charlie["gid"]
# => "ops"
charlie["shell"]
# => "/bin/zsh"
```

The following recipe can be used to create a user for each administrator
by loading all of the items from the "admins" data bag, looping through
each admin in the data bag, and then creating a user resource so that
each of those admins exist:

``` ruby
admins = data_bag('admins')

admins.each do |login|
  admin = data_bag_item('admins', login)
  home = "/home/#{login}"

  user(login) do
    uid       admin['uid']
    gid       admin['gid']
    shell     admin['shell']
    comment   admin['comment']
    home      home
    manage_home true
  end

end
```

And then the same recipe, modified to load administrators using a search
query (and using an array to store the results of the search query):

``` ruby
admins = []

search(:admins, "*:*").each do |admin|
  login = admin["id"]

  admins << login

  home = "/home/#{login}"

  user(login) do
    uid       admin['uid']
    gid       admin['gid']
    shell     admin['shell']
    comment   admin['comment']

    home      home
    manage_home true
  end

end
```