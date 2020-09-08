For example:

``` ruby
name "jenkins-master"
run_list "java", "jenkins::master", "recipe[policyfile_demo]"
default_source :supermarket, "https://mysupermarket.example"
cookbook "policyfile_demo", path: "cookbooks/policyfile_demo"
cookbook "jenkins", "~> 2.1"
cookbook "mysql", github: "chef-cookbooks/mysql", branch: "master"
```