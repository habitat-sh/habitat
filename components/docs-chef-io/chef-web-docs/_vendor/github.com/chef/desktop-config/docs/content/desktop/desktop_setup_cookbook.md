+++
title = "Chef Desktop Development Environment"
draft = false
publishDate = "2020-06-02"

[menu]
  [menu.desktop]
    title = "Chef Desktop Development Environment"
    identifier = "desktop/desktop_setup_cookbook.md Cookbook Development Setup"
    parent = "desktop"
    weight = 40
+++
[\[edit on GitHub\]](https://github.com/chef/desktop-config/blob/master/docs/content/desktop/desktop_setup_cookbook.md)

*Estimated time: 1 hour (with your own virtual machines)*

This step introduces you to fundamentals of cookbook development and walks you through the process of creating, editing, and testing a simple cookbook. These are the same basic steps that you will expand on in developing your Chef Desktop cookbook.

1. Learn about testing with Test Kitchen
1. Set up your Chef development environment
1. Make a demo cookbook
1. Test your cookbook locally
1. Test your cookbook with Test Kitchen, once you have your virtual machine images

## Development and Testing Cookbooks for Windows 10 and macOS Platforms

Testing is central to good software development. Testing your Chef Desktop cookbook provides you the opportunity to detect and correct problems before putting changes into production. Testing saves time and money, but it adds value by helping your organization achieve and maintain operational velocity.

### Licenses

It is important to understand your operating license and service agreements.
The best place to begin is find and understand your SLA:

* [Windows Licenses](https://www.microsoft.com/licensing/default)
* [Apple macOS SLA](https://www.apple.com/legal/sla/)

## Test Kitchen

Test Kitchen was installed with Chef Workstation. It provides Chef Infra with a testing harness for cookbooks that uses virtual machines(VMs). Consult your Apple and Microsoft licenses and service level agreements (SLA) to understand your options for acquiring or creating VMs for development.

### Test Kitchen Integrations

Test Kitchen uses a driver plugin architecture to enable Test Kitchen to test instances on cloud providers such as Amazon EC2, Google Compute Engine, and Microsoft Azure. You can also test on multiple local hypervisors, such as VMware, Hyper-V, or VirtualBox. [Test Kitchen Documentation](https://docs.chef.io/workstation/kitchen/) and the [Test Kitchen GitHub Repository](https://github.com/test-kitchen).

### Setup Your Development Environment

In this step, you will prepare your workstation for developing, testing, and deploying the Chef Desktop cookbook.

1. Create your local repo

    You will need a local repository on your workstation for storing your cookbooks and related chef work, and for sharing it with GitHub or another version control system.

    From the command line in your root folder (c:\ or ~/), run:

    ```powershell
    chef generate repo my_repo
    ```

### Make Your First Cookbook

  This is a practice cookbook to understand how to test.

1. Create your cookbook

    ```powershell
    chef generate cookbook my_repo/cookbooks/my_cookbook
    ```

1. Edit the `metadata.rb` file

    Open your repo in Visual Studio Code. The one key file you will want to manage is the `metadata.rb` file. Please take a moment now to add your contact information and enter a starting version number for your cookbook.

    ```powershell
    name 'my_cookbook'
    maintainer 'The Authors'
    maintainer_email 'you@example.com'
    license 'All Rights Reserved'
    description 'Installs/Configures my_cookbook'
    version '0.1.0'
    chef_version '>= 16.0'
    ```

1. Edit the default recipe

   Open `my_repo\cookbook\my_cookbook\recipes\default.rb` in Visual Studio and add:

    For Windows:

    ```ruby
    powershell_script 'get my path' do
      code <<-CODE
      [Environment]::GetEnvironmentVariable("Path")
      CODE
    end
    ```

    For macOS:

    ```ruby
    bash 'get my path' do
      user 'root'
      code <<-EOH
      echo $PATH
      EOH
    end
    ```

    * Spacing matters! Be mindful of spaces
    * Use LF line spacing and not CRLF. (Look to the bottom right of the status bar in Visual Studio Code)

### Test Your Cookbook

  In the command line, navigate to your cookbook directory. That path should be similar to `c:\my_repo\cookbooks\my_cookbook`. Then run the following command to test your code out:

  ```powershell
  cd c:\my_repo\cookbooks\my_cookbook
  chef-client -z -o my_cookbook
  ```

  You should see the cookbook path displayed in the 'run' command output. If it did not run or if it returned an error, go back and check your spelling and spacing.

#### Test With Test Kitchen

1. Apply the Cookbooks to the Images

    Run the following command to apply, or 'converge', the cookbooks with the base OS image:

    ```powershell
    kitchen converge
    ```

1. Verify the settings

    Confirm that the converged code is the code that you meant to apply. In VSCode, navigate to the `test\integration\default` directory and examine the generated integration tests. Carefully go through these tests and adjust them to match the setting to the changes in the `default.rb` file. Next, run:

    ```powershell
    kitchen verify
    ```

1. Cleanup

    When you finish with your testing, you can run the following command to delete the running test images:

    ```powershell
    kitchen destroy
    ```

#### Kitchen Test

Once you are familiar with Test Kitchen, you can perform all of the steps at once, including cleanup, with:

  ```powershell
  kitchen test
  ```

- Next: [Chef Desktop Cookbook Development Environment]({{< relref "desktop_pattern.md" >}})
- Last: [Install Chef Components]({{< relref "desktop_setup_chef.md" >}})
