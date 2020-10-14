+++
title = "Builder Profile"

date = 2020-10-12T16:08:26-07:00
draft = false

[menu]
  [menu.habitat]
    title = "Builder Profile"
    identifier = "habitat/builder/builder-profile.md Builder Profile"
    parent = "habitat/builder"
    weight = 30
+++

<<<<<<< HEAD:components/docs-chef-io/content/habitat/builder-profile.md
[\[edit on GitHub\]](https://github.com/habitat-sh/habitat/blob/master/components/docs-chef-io/content/habitat/builder-profile.md)
=======
Whether you are looking to leverage the SaaS or on-prem version of Chef Habitat Builder, you will need to create an account on the SaaS version of Chef Habitat Builder. After you have then downloaded the version, you will then sync the two accounts.

This documentation covers everything from creating an account to setting up automated builds and exporting packages to a variety of container registries.

## Get an Account

### Prerequisites

You need to set a few things up before you can get started with Chef Habitat Builder:

* Download and install the [Chef Habitat CLI](https://www.habitat.sh/docs/install-habitat/#install-habitat)
* A [GitHub account](https://github.com/join)

### Sign-in and Authorize Chef Habitat Builder

Chef Habitat Builder automatically creates your account the first time you sign in using the GitHub authentication process. You'll also need to authorize the Chef Habitat Builder application in Github.

Head over to the Chef Habitat Builder sign-in page at [https://bldr.habitat.sh/#/sign-in](https://bldr.habitat.sh/#/sign-in) to get started.

1. To sign in with an existing GitHub account, select **Sign in with GitHub**
1. If you need to set up a GitHub account, select the **Sign up here** link

![Chef Habitat sign in with Github](/images/habitat/builder_signin.png)

Signing in with your GitHub account and authorizing the Chef Habitat Builder application the first time you sign in grants you access to the Chef Habitat Builder platform. Once you've completed signing in and authorizing Chef Habitat Builder, you'll arrive at the 'My Origins' view.

![Authorize the Chef Habitat Application](/images/habitat/authorize.png)

## Set up your Profile
>>>>>>> b2e0b5b8... Keep only used images in /static/images/habitat:components/docs-chef-io/content/habitat/builder-get-started.md

Use the _Profile_ tab to:

* See the GitHub account used to sign in
* Add an email to your profile
* Create your personal access token

Access your profile by selecting the **round icon at the top right corner** of any page. Select the **profiles** option from the drop-down menu to  customize your profile and create your personal access token.

![Access your Chef Habitat Builder profile](/images/habitat/builder_profile.png)

### Register an Email Address

Adding an email address to your profile gives the Chef Habitat team permission to contact you directly about important information. If you use an email address associated with a GitHub account, it will also use your GitHub avatar. Save your changes by selecting **save**.

![Register your email address](/images/habitat/builder_profile_user.png)

### Create a Personal Access Token

Chef Habitat Builder uses an access token, called a _personal access token_ or a _Habitat authentication token_ (HAB_AUTH_TOKEN), to give you access to actions that you would like to take on Chef Habitat Builder. The _personal access token_ is the first level of permissions that you need to for any interactions with Chef Habitat Builder, such as uploading packages or checking the status of build jobs.

Create your personal access token at the bottom of the profile page (below the save button), by selecting **Generate Token**.

![Create your personal access token](/images/habitat/generate-token.png)

Your generated access token will appear in the field. The access token is visible in the tab once, and navigating away from or reloading the page will cause it to vanish from the display. Copy your access token by selecting the icon on the right side of the field and set it as an environment variable before continuing.

![Copy your personal access token](/images/habitat/copy-token.png)

#### Set the personal access token as a Windows Environment Variable

You can use your personal access token as a [Windows environment variable](https://docs.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_environment_variables?view=powershell-7) for a single session by passing it in the command line or save it in your user settings for use across sessions.

Save your personal authorization token as a permanent environment variable in Windows using:

```PS
SETX HAB_AUTH_TOKEN <token> /m
```

Replacing <token> with the contents of your generated personal access token.

You can also save your personal access token as a permanent environment variable using the Windows user interface. In your Windows help bar, enter `environment` and select **Edit the system environment variables** from the list of suggestions.

This opens the `System Properties` window on the `Advanced` tab. Select the `Environment Variables` button.

![Navigate to Windows Environment Variables](/images/habitat/environment_variable.png)

In the next window, select the `New` button in the top part. This opens a dialog box that lets you set individual user variables.

![Make new user variable](/images/habitat/environment_variable_new.png)

Create a permanent environment variable by entering `HAB_AUTH_TOKEN` as the variable name. Next, paste the authorization token that you copied after you generated a new token on your profile page as the variable value. After you select the `OK`, you will see the new token in the user variables field.

![Save your HAB_AUTH_TOKEN](/images/habitat/environment_variable_new_var.png)

To test that your new token works correctly, open the Command Prompt---which you can find by entering command in the Windows search box---and entering `echo %HAB_AUTH_TOKEN%`. You should see the value that you pasted into the environment variable.

#### Set the personal access token as a macOS Environment Variable

Set the `HAB_AUTH_TOKEN` in the CLI with:

```bash
export HAB_AUTH_TOKEN=<token>
```

Replacing `<token>` with the contents of your generated personal access token.

To use your personal access token across sessions, set it as an environment variable in your interactive shell configuration file, such as your `.bashrc`.

```bash
export HAB_AUTH_TOKEN=<token>
```

Then initialize the path from the command line, by running:

```bash
source ~/.bashrc
```
