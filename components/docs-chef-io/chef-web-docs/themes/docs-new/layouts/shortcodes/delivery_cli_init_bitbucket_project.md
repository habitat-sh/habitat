To initialize a project using a Bitbucket repository, run a command
similar to:

``` bash
delivery init --bitbucket PROJECT_KEY -r REPO_NAME
```

where `PROJECT_KEY` is the name of the project key in Bitbucket and
`REPO_NAME` is the name of the repository in Bitbucket. For example to
initialize the `anagrams` repository in Bitbucket with the `TEST`
project key:

``` bash
delivery init --bitbucket TEST -r anagrams
```

and returns output similar to:

``` none
Chef Delivery
Loading configuration from /Users/justinc/chef/delivery/organizations/sandbox/anagrams
Is /Users/justinc/chef/delivery/organizations/sandbox/anagrams a git repo?  yes
Creating bitbucket project: anagrams  created
adding remote delivery: ssh://justinc@Chef@delivery.chef.co:8989/Chef/sandbox/anagrams
Remote 'delivery' added to git config!
Checking for content on the git remote delivery: No upstream content
Pushing local content to server:
To ssh://justinc@Chef@delivery.chef.co:8989/Chef/sandbox/anagrams
*   refs/heads/master:refs/heads/master [new branch]
Branch master set up to track remote branch master from delivery.
Done

Creating and checking out add-delivery-config feature branch: done
Generating build cookbook skeleton
Using cached copy of build-cookbook generator "/Users/justinc/.delivery/cache/generator-cookbooks/pcb"
Build-cookbook generated: "chef" "generate" "cookbook" ".delivery/build-cookbook" "-g" "/Users/justinc/.delivery/cache/generator-cookbooks/pcb"
Adding and committing build-cookbook: done
Writing configuration to /Users/justinc/chef/delivery/organizations/sandbox/anagrams/.delivery/config.json
New delivery configuration
--------------------------
{
  "version": "2",
  "build_cookbook": {
    "name": "build-cookbook",
    "path": ".delivery/build-cookbook"
  },
  "skip_phases": [],
  "build_nodes": {},
  "dependencies": []
}
Git add and commit delivery config: done
Chef Delivery
Loading configuration from /Users/justinc/chef/delivery/organizations/sandbox/anagrams
Review for change add-delivery-config targeted for pipeline master
Created new patchset
https://delivery.chef.co/e/Chef/#/organizations/sandbox/projects/anagrams/changes/695f2bb9-ab21-4adf-a6e0-b9fc79854478
  anagrams git:(add-delivery-config)
```