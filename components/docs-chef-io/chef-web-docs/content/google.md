+++
title = "Chef and Google"
draft = false

aliases = ["/google.html"]

[menu]
  [menu.infra]
    title = "Google Cloud Platform"
    identifier = "chef_infra/setup/integrations/google.md Google Cloud Platform"
    parent = "chef_infra/setup/integrations"
    weight = 20
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/google.md)

Google Cloud Platform is a suite of cloud computing services that run on
the same infrastructure that Google uses internally for its end-user
products, such as Google Search and YouTube. Alongside a set of
management tools, it provides a series of modular cloud services
including computing, data storage, data analytics, and machine learning.
This page outlines the different tools that can be used to integrate
Chef with the Google Cloud Platform.

## knife-google

[\[GitHub\]](https://github.com/chef/knife-google)

This plugin gives knife the ability to create, bootstrap, and manage
Google Compute Engine (GCE) instances.

### Authentication and Authorization

`knife-google` relies on the Google Auth Library to handle
authentication to the Google Cloud API. The auth library expects to find
a JSON credentials file located under
`~/.config/gcloud/application_default_credentials.json`.

The easiest way to create this is to download and install the [Google
Cloud SDK](https://cloud.google.com/sdk/) and run the
`gcloud auth application-default login` command, which will create the
credentials file for you.

If you already have a file you'd like to use that is in a different
location, set the `GOOGLE_APPLICATION_CREDENTIALS` environment variable
with the full path to that file.

These are the necessary settings for your `config.rb` file:

``` ruby
knife[:gce_project] = 'my-test-project'
knife[:gce_zone]    = 'us-east1-b'
```

### Usage Examples

**Create a server:**

``` bash
knife google server create test-instance-1 --gce-image centos-7-v20160219 \
--gce-machine-type n1-standard-2 --gce-public-ip ephemeral --connection-user myuser \
--identity-file /Users/myuser/.ssh/google_compute_engine
```

**Delete multiple servers:**

``` bash
knife google server delete my-instance-1 my-instance-2 --purge
```

**List all servers:**

``` bash
knife google server list
```

## kitchen-google

[\[GitHub\]](https://github.com/test-kitchen/kitchen-google)

A test kitchen driver for Google Cloud Platform.

### Usage Examples

The following is a basic `kitchen.yml` example:

``` yaml
---
driver:
  name: gce
  project: mycompany-test
  zone: us-east1-c
  email: me@mycompany.com
  tags:
    - devteam
    - test-kitchen
  service_account_scopes:
    - devstorage.read_write
    - userinfo.email

provisioner:
  name: chef_zero

transport:
 username: chefuser

platforms:
  - name: centos-7
    driver:
      image_project: centos-cloud
      image_name: centos-7-v20170124
  - name: ubuntu-16.04
    driver:
     image_project: ubuntu-os-cloud
     image_family: ubuntu-1604-lts
  - name: windows
    driver:
     image_project: windows-cloud
     image_name: windows-server-2012-r2-dc-v20170117
     disk_size: 50
suites:
  - name: default
    run_list:
      - recipe[COOKBOOK::default]
    attributes:
```
