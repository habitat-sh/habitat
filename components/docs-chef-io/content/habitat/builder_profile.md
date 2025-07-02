+++
title = "Builder Profile"

date = 2020-10-12T16:08:26-07:00
draft = false
gh_repo = "habitat"

[menu]
  [menu.habitat]
    title = "Builder Profile"
    identifier = "habitat/builder/builder-profile Builder Profile"
    parent = "habitat/builder"
    weight = 30
+++

To use the SaaS or on-premises version of Chef Habitat Builder, you need to create an account on the SaaS version. After downloading the version, sync the two accounts.

## Get an account

### Prerequisites

Set up the following before getting started with Chef Habitat Builder:

- Download and install the [Chef Habitat CLI]({{< relref "install_habitat" >}})
- A [GitHub account](https://github.com/join)

### Sign in and authorize Chef Habitat Builder

Chef Habitat Builder automatically creates your account the first time you sign in using GitHub authentication. You'll also need to authorize the Chef Habitat Builder application in GitHub.

Go to the [Chef Habitat Builder sign-in page](https://bldr.habitat.sh/#/sign-in) to get started.

1. To sign in with an existing GitHub account, select **Sign in with GitHub**.
1. To set up a GitHub account, select the **Sign up here** link.

Signing in with your GitHub account and authorizing the Chef Habitat Builder application grants you access to the platform. After signing in and authorizing, you'll arrive at the **My Origins** page.

## Set up your Habitat Builder profile

Use the **Profile** tab to:

- View the GitHub account used to sign in.
- Add an email to your profile.
- Create your personal access token.

Access your profile by selecting the **round icon at the top right corner** of any page, then select **Profile** from the drop-down menu. From here, you can manage your profile, create a personal access token, and add a Progress Chef license key.

### Add a Progress Chef license key

To download and sync official Chef-maintained packages from Chef Habitat SaaS Builder to an on-premises Builder instance, you need a valid license key.

To add your license key, follow these steps:

1. If you're an enterprise user, log into your customer portal and copy the license key linked to your asset.

   Free or trial users can get a [free or trial license key](https://www.chef.io/license-generation-free-trial).

1. Log into [Chef Habitat SaaS Builder](https://bldr.habitat.sh). If you haven't entered your license key, a pop-up will prompt you to do so.

   Enter your license key in the field provided and select **Proceed**. Once entered, your account will be authorized to view and download official Chef-maintained packages.

### Register an email address

Adding an email address to your profile allows the Chef Habitat team to contact you about important information. If you use an email address associated with a GitHub account, it will also use your GitHub avatar. Save your changes by selecting **Save**.

### Create a personal access token

Chef Habitat Builder uses a personal access token (`HAB_AUTH_TOKEN`) to authorize actions from the `hab` CLI, such as uploading packages or checking build job statuses.

To create your personal access token:

1. Go to the bottom of the profile page and select **Generate Token**.

1. Copy the generated token by selecting the icon on the right side of the field. The token is visible only once. If you navigate away or reload the page, it will disappear. Save it as an environment variable before continuing.

#### Set the personal access token as a Windows environment variable

To use your token in a single session, pass it in the command line. To use it across sessions, save it as a permanent environment variable.

To save it permanently, use:

```PS
SETX HAB_AUTH_TOKEN <TOKEN> /m
```

Replace `<TOKEN>` with your generated token.

You can also save it through the Windows user interface:

1. Search for "environment" in the Windows help bar and select **Edit the system environment variables**.
1. In the **System Properties** window, select **Environment Variables**.
1. In the next window, select **New** under user variables.
1. Enter `HAB_AUTH_TOKEN` as the variable name and paste your token as the value. Select **OK** to save.

To test, open Command Prompt and enter:

```cmd
echo %HAB_AUTH_TOKEN%
```

You should see your token value.

#### Set the personal access token as a macOS environment variable

To set the token for the current session, use:

```bash
export HAB_AUTH_TOKEN=<TOKEN>
```

Replace `<TOKEN>` with your generated token.

To use it across sessions, add it to your shell configuration file (for example, `.bashrc`):

```bash
export HAB_AUTH_TOKEN=<TOKEN>
```

Then initialize the path with:

```bash
source ~/.bashrc
```
