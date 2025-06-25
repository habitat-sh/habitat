## Add a License Key

In order to download and sync official Chef-maintained packages from the Public Builder to your On-Prem Builder instance, a valid license key is required.

### Step 1: Obtain Your License Key

If you are an enterprise user:  
1. Log into your customer portal.
2. Copy the license key linked to your asset.

If you are a free or trial user:  
1. Generate your license key from [https://www.chef.io/license-generation-free-trial](https://www.chef.io/license-generation-free-trial)

### Step 2: Sign In to Public Builder

Navigate to the [Public Builder](https://bldr.habitat.sh) and sign in using your GitHub account.

![Sign In to Public Builder](/images/habitat/sign_in_to_public_builder.png)

### Step 3: Add the License Key to the Public Builder

After logging into the [Public Builder](https://bldr.habitat.sh), if a valid license key is not already present on your account, a pop-up will appear prompting you to enter your license key.

Enter your license key in the field provided and click **Proceed** to continue.

![Add License Key](/images/habitat/add_license_key_to_builder.png)

Once entered, your account will be authorized to view and download official Chef-maintained packages.

![License Key Added](/images/habitat/license_key_added.png)

In case an invalid license key is provided, or if the license key is expired, you will be prompted to re-enter a valid license key.

![Re-enter License Key](/images/habitat/re_enter_license_key.png)

### Step 4: Generate a Personal Access Token

Still in the **Profile** page, generate a Personal Access Token if you don’t already have one.  
This token is required for downloading, uploading, and building packages using the `hab` CLI.

### Step 5: Use the Auth Token

To use your Personal Access Token with the `hab` CLI, set it as an environment variable or pass it as a flag:

- Environment variable:
  ```bash
  export HAB_AUTH_TOKEN=<your_token>
  ```
- Or CLI flag:
  ```bash
  hab pkg download -z <your_token>
  ```

The following `hab` commands require an auth token when interacting with the Public Builder:

```bash
hab pkg download
hab pkg build
hab pkg upload
hab studio enter
hab studio build
hab studio new
hab studio run
```
### Note on Access Restrictions

The UI access restrictions will be applicable to all Habitat packages available in `bldr.habitat.sh`. However, we have ensured that the license restriction for downloading packages does **not** apply to any existing Chef products (e.g., Infra 18, Habitat 1.x, etc.) and related packages in the stable channel.

This change will be applicable for downloading **all new** Chef product releases (e.g., Infra 19, Habitat 2.0, InSpec 7, etc.) and their dependencies from `bldr.habitat.sh`.

Any existing pipelines or workflows that download or build packages for **existing releases** from Public Builder **without using an auth token** are not impacted.
