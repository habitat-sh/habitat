The `remote_file` resource on Windows supports accessing files from a
remote SMB/CIFS share. The file name should be specified in the source
property as a UNC path e.g. `\\myserver\myshare\mydirectory\myfile.txt`.
This allows access to the file at that path location even if the Chef
Infra Client process identity does not have permission to access the
file. Credentials for authenticating to the remote system can be
specified using the `remote_user`, `remote_domain`, and
`remote_password` properties when the user that Chef Infra Client is
running does not have access to the remote file. See the "Properties"
section for more details on these options.

**Note**: This is primarily for accessing remote files when the user
that Chef Infra Client is running as does not have sufficient access,
and alternative credentials need to be specified. If the user already
has access, the credentials do not need to be specified. In a case where
the local system and remote system are in the same domain, the
`remote_user` and `remote_password` properties often do not need to be
specified, as the user may already have access to the remote file share.

Examples:

**Access a file from a different domain account:**

``` ruby
remote_file "E:/domain_test.txt"  do
  source  "\\\\myserver\\myshare\\mydirectory\\myfile.txt"
  remote_domain "domain"
  remote_user "username"
  remote_password "password"
end
```

OR

``` ruby
remote_file "E:/domain_test.txt"  do
  source  "\\\\myserver\\myshare\\mydirectory\\myfile.txt"
  remote_user "domain\\username"
  remote_password "password"
end
```

**Access a file using a local account on the remote machine:**

``` ruby
remote_file "E:/domain_test.txt"  do
  source  "\\\\myserver\\myshare\\mydirectory\\myfile.txt"
  remote_domain "."
  remote_user "username"
  remote_password "password"
end
```

OR

``` ruby
remote_file "E:/domain_test.txt"  do
  source  "\\\\myserver\\myshare\\mydirectory\\myfile.txt"
  remote_user ".\\username"
  remote_password "password"
end
```