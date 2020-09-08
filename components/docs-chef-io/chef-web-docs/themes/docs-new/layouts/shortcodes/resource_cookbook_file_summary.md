Use the **cookbook_file** resource to transfer files from a
sub-directory of `COOKBOOK_NAME/files/` to a specified path located on a
host that is running Chef Infra Client. The file is selected according
to file specificity, which allows different source files to be used
based on the hostname, host platform (operating system, distro, or as
appropriate), or platform version. Files that are located in the
`COOKBOOK_NAME/files/default` sub-directory may be used on any platform.