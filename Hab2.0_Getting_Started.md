# Habitat 2.0 Testing Guide
This guide provides step-by-step testing procedures to validate Habitat 2.0 functionality and migration from Habitat 1.6. This is intended for testing purposes and validation of the upgrade process.

## Prerequisites

- Valid license key for Habitat 2.0
- Administrative privileges on your test system
- Basic familiarity with Habitat concepts

## Overview

The testing process involves:
1. Setting up Habitat 1.6 baseline environment
2. Creating test packages and services
3. Migrating to Habitat 2.0 using official upgrade procedures
4. Validating functionality with both stable and base channels

---

## Step 1: Install Habitat v1.6 Baseline

Follow the [official installation guide]( "https://docs.chef.io/habitat/install_habitat/#chef-habitat-for-linux" ) for complete platform-specific instructions. Below are quick installation steps for each platform:

### Linux Installation

```bash
# Basic installation (installs latest stable version)
curl https://raw.githubusercontent.com/habitat-sh/habitat/main/components/hab/install.sh | sudo bash
```

### Windows Installation

```bash
Set-ExecutionPolicy Bypass -Scope Process -Force
iex ((New-Object System.Net.WebClient).DownloadString('https://raw.githubusercontent.com/habitat-sh/habitat/main/components/hab/install.ps1'))
```

### Common Verification Steps (All Platforms)
- Check version: `hab --version`

### Habitat setup

To generate your HAB_AUTH_TOKEN, goto your builder UI and follow the steps [here](https://docs.chef.io/habitat/builder_profile/#create-a-personal-access-token)

```bash
#Name of origin where package is uploaded to in public builder
export HAB_ORIGIN=chef-private 

#Required for private packages
export HAB_AUTH_TOKEN='your token' 

#One time command to generate the public key at ~/.hab/cache/keys
hab origin key generate chef-private 

#Copy the key to /hab/cache/keys for commans that run in root/sudo
sudo cp ~/.hab/cache/keys/chef-private-* /hab/cache/keys/

```
---

## Step 2: Setup Habitat Supervisor Service

### Linux: Create systemd Service

Create the systemd service file:

```bash
sudo tee /etc/systemd/system/hab-sup.service > /dev/null <<EOF
[Unit]
Description=The Habitat Supervisor
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
ExecStart=$(which hab) sup run
Environment=HAB_LICENSE=accept-no-persist
Restart=on-failure
RestartSec=10
KillMode=mixed
KillSignal=SIGINT
TimeoutStartSec=120
TimeoutStopSec=60

[Install]
WantedBy=multi-user.target
EOF
```

Enable and start the service:

```bash
# Reload systemd configuration
sudo systemctl daemon-reload

# Enable the service
sudo systemctl enable hab-sup

# Start the service
sudo systemctl start hab-sup
sleep 10

# Verify service is running
sudo systemctl status hab-sup

# Check supervisor is accessible
sudo hab svc status
```

### Windows: Install Windows Service Package

```powershell
# Install the windows service package
hab pkg install core/hab-sup
hab pkg install core/windows-service

# Create and start the Windows service
hab pkg exec core/windows-service install

# Verify service is running
Get-Service habitat
```

### Verification Steps (All Platforms)

- **Linux**: `sudo systemctl status hab-sup` shows active/running
- **Windows**: `Get-Service habitat` shows running services
- **All**: Supervisor is accessible: `sudo hab svc status` (should connect successfully)
- **All**: Check logs for any startup issues

---

## Step 3: Build a Plan Against Stable Channel [TODO: Fix the key-not-found error ]

### Create Test Plan

Create a simple test plan or use existing Habitat components for testing:

```bash
# Create a workspace for testing
mkdir -p ~/hab-testing/test-plan
cd ~/hab-testing/test-plan

# Create a minimal test plan
cat > plan.sh << 'EOF'
pkg_name=test-service
pkg_origin=chef-private
pkg_version="1.0.0"
pkg_maintainer="Test User <test@company.com>"
pkg_license=('Apache-2.0')
pkg_description="Test service for Habitat 2.0 migration validation - Cross Platform"

# Cross-platform dependency
pkg_deps=(
  core/python
)

# Platform-agnostic service using Python HTTP server
pkg_svc_run="python -m http.server 8080"

do_build() {
  return 0
}

do_install() {
  return 0
}
EOF
```

### Build Against Stable Channel

```bash
hab pkg build .
```

### Alternative: Use Existing Component

For more realistic testing, use an existing component like `core/nginx`:

```bash
# Test with existing core package
sudo hab pkg install core/nginx
sudo hab svc load core/nginx
```

### Verification Steps

- Build completes successfully
- Package artifact is created in `results/` directory
- Dependencies are pulled from stable channel (verify in build output)

**Note:** Make a note of the atrifact (.hart) file that is generated at the end of the terminal logs.

---

## Step 4: Run the Built Package in Supervisor

### Load and Start the Service

```bash

# Install the package
sudo -E hab pkg install <path to the artifact(.hart) created in previous step>

# Load the service in supervisor
sudo hab svc load chef-private/test-service/1.0.0

# Alternatively - Load the service by providing the full pkg_ident
sudo hab svc load <PKG_IDENT>

# Verify service is loaded
sudo hab svc status
```

### Verification Steps

- Service loads without errors
- Service shows "UP" status in `hab svc status`

---

## Step 5: Setup HAB Auth Token with License Key

### Obtain License Key and Token
If you don't have one already, generate a free/trial license key to go ahead over [here](https://www.chef.io/license-generation-free-trial). Refer this [page](https://docs.chef.io/habitat/builder_profile/#add-a-progress-chef-license-key) for proper instructions.
Once generated, you should have the below info ready with you:
- Valid license key for Habitat 2.0
- Auth token associated with the license

### Persistent Configuration

Set your HAB_AUTH_TOKEN, if not already set.

```bash
# Linux/macOS: Add to shell profile
echo 'export HAB_AUTH_TOKEN=your_auth_token_here' >> ~/.bashrc

# Windows: Set environment variable
setx HAB_AUTH_TOKEN "your_auth_token_here"
```

---

## Step 6: Migrate Environment to Habitat 2.0

### (Optional) Stop Current Services

**Note:** If your supervisor is running services while executing the migration script, they will be restarted.

### Run Migration Script

To upgrade a supervisor from 1.6.x to 2.0.x, run the following:

- **linux**: 

```bash
curl https://raw.githubusercontent.com/habitat-sh/habitat/main/components/hab/migrate.sh | sudo bash -s -- --auth <HAB_AUTH_TOKEN>
```

- **windows**: 

```bash
iex "& { $(irm https://raw.githubusercontent.com/habitat-sh/habitat/main/components/hab/migrate.ps1) } --auth <HAB_AUTH_TOKEN>"
```

### Verification
- Check version: `hab --version` - Should show hab v2.x

---

## Step 7: Rebuild Plan Against Stable Channel (Habitat 2.0)

### Update Plan File
Update the `pkg_version="2.0.0"` in the existing plan file.

```bash
sed -i 's/pkg_version="1.0.0"/pkg_version="2.0.0"/' plan.sh
```

### Set env to Build Against Stable Channel

```bash
export HAB_REFRESH_CHANNEL=stable
export HAB_INTERNAL_BLDR_CHANNEL=acceptance 
export HAB_STUDIO_SECRET_HAB_INTERNAL_BLDR_CHANNEL=acceptance
```

### Build

```bash
hab pkg build .
```

### Verification Steps

- Build completes with Habitat 2.0 toolchain
- Dependencies pulled from stable channel using chef/* packages where applicable
- Package version shows 2.0.0

**Note:** Make a note of the atrifact (.hart) file that is generated at the end of the terminal logs.

---

## Step 8: Load and Verify Package in Supervisor

### Install and Load Service

```bash
# Install the new package
sudo -E hab pkg install <path to the artifact(.hart) created in previous step>

# Load the updated service
sudo hab svc load chef-private/test-service/2.0.0 --force

# Verify service status
sudo hab svc status
```

### Verification Steps

- Service loads new 2.0.0 version
- Service runs without errors

---

## Step 9: Rebuild Plan Against Base Channel

### Update Plan for Base Channel

```bash
# Update version for base channel build
sed -i 's/pkg_version="2.0.0"/pkg_version="2.0.1"/' plan.sh
```

### Build Against Base Channel

```bash
# Use the default channel , which is base.
unset HAB_REFRESH_CHANNEL

# Build
hab pkg build .
```

### Verification Steps

- Build completes successfully using base channel
- Dependencies are resolved from base channel
- Package version shows 2.0.1
- Build process shows base channel package downloads

**Note:** Make a note of the atrifact (.hart) file that is generated at the end of the terminal logs.

---

## Step 10: Load and Verify Base Channel Package

### Install and Load Final Service
```bash
# Install the new package
sudo -E hab pkg install <path to the artifact(.hart) created in previous step>

# Load the updated service
sudo hab svc load chef-private/test-service/2.0.1 --force

# Verify service status
sudo hab svc status
```

### Verification Steps

- Service loads new 2.0.1 version
- Service runs without errors

---

## Troubleshooting

### Common Issues

#### Authentication Failures
1. Check token validity
2. Test package access `hab pkg search chef/hab-sup`

#### Service Won't Start
```bash
# Check service logs
journalctl -u hab-sup -n 50

# Verify package installation
hab pkg list <PKG_IDENT>
```

#### Secret key mismatch error
Error you get when a supervior is already running and you try to run a new one.
To fix this, kill the exisiting supervisor process and then follow step 2.

#### Permission denied
If you get 404 or permissiond enied while building a package its usually because you have not set you auth token.
Set it and try again.