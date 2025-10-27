# Habitat 2.0 Testing Guide
This guide provides step-by-step testing procedures to validate Habitat 2.0 functionality and migration from Habitat 1.6. This is intended for testing purposes and validation of the upgrade process.

## Prerequisites

- Valid license key for Habitat 2.0
- Administrative privileges on your test system
- Basic familiarity with Habitat concepts
- Refer to the [official installation guide](https://docs.chef.io/habitat/install_habitat/) for installation procedures

## Overview

The testing process involves:
1. Setting up Habitat 1.6 baseline environment
2. Creating test packages and services
3. Migrating to Habitat 2.0 using official upgrade procedures
4. Validating functionality with both stable and base channels

---

## Step 1: Install Habitat v1.6 Baseline

Follow the [official installation guide]({{< relref "install_habitat" >}}) for complete platform-specific instructions. Below are quick installation steps for each platform:

### Linux Installation

```bash
# Basic installation (installs latest stable version)
curl https://raw.githubusercontent.com/habitat-sh/habitat/main/components/hab/install.sh | sudo bash
```

**Note:** Refer to the following guide:
- [Linux Installation Guide](https://docs.chef.io/habitat/install_habitat/#chef-habitat-for-linux)

### Windows Installation

Refer [Windows Installation Guide](https://docs.chef.io/habitat/install_habitat/#chef-habitat-for-windows)

### Common Verification Steps (All Platforms)
- Check version: `hab --version`

### Habitat setup

YOu can generate your HAB_AUTH_TOKEN [here](https://docs.chef.io/habitat/builder_profile/#create-a-personal-access-token)

```bash
#Name of origin where package is uploaded to in public builder
export HAB_ORIGIN=chef-private 

#Required for private packages
export HAB_AUTH_TOKEN='your token' 

#One time command to generate the public key
hab origin key generate chef-private 
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
pkg_svc_run="python3 -m http.server 8080"

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
# Enter studio 

hab studio enter

# Inside studio - build the package
build

# Exit studio
exit
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

---

## Step 4: Run the Built Package in Supervisor

### Load and Start the Service

```bash
# Find the built package
PACKAGE_PATH=$(find results/ -name "*.hart" -type f | head -1)
echo "Built package: $PACKAGE_PATH"

# Install the package
sudo hab pkg install $PACKAGE_PATH

# Load the service in supervisor
sudo hab svc load chef-private/test-service

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

### Configure Authentication

```bash
# Set the auth token
export HAB_AUTH_TOKEN=your_auth_token_here
```

### Persistent Configuration

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
iex "& { $(irm https://raw.githubusercontent.com/habitat-sh/habitat/master/components/hab/migrate.ps1) } --auth <HAB_AUTH_TOKEN>"
```

### Verification
- Check version: `hab --version` - Should show hab v2.x