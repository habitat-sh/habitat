#!/bin/bash
set -euo pipefail

# Default channel is acceptance
CHANNEL="acceptance"
# Use existing HAB_AUTH_TOKEN if available, otherwise empty
AUTH_TOKEN="${HAB_AUTH_TOKEN:-}"

# Always use migrate.sh as the script name in help text
SCRIPT_NAME="migrate.sh"

# Function to display help information
show_help() {
  cat << EOF
Usage: ${SCRIPT_NAME} [OPTIONS]

Description:
  This script migrates Chef Habitat to the latest chef/hab* packages.
  It will install the latest chef/hab binary from the specified channel,
  followed by chef/hab-sup and chef/hab-launcher if hab-sup is already
  installed. If a hab-sup systemd service is running and the new version is
  greater than the current version, it will restart the service. The
  provided authentication token is added to the hab-sup systemd service
  environment.

Options:
  --help             Show this help message and exit
  --channel CHANNEL  Specify the channel to install packages from (default: acceptance)
  --auth TOKEN       Specify the authentication token for license restricted packages
                     (uses HAB_AUTH_TOKEN environment variable if not provided)

Examples:
  ${SCRIPT_NAME} --auth "your-token"
  ${SCRIPT_NAME} --channel stable --auth "your-token"

Note:
  - Authentication token is required (via --auth or HAB_AUTH_TOKEN environment variable)
  - chef/hab-sup and chef/hab-launcher are only installed if hab-sup is already present
  - The service restart only happens if the new version is greater than the currently running version
EOF
  exit 0
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
  case "$1" in
    --help|-h)
      show_help
      ;;
    --channel)
      CHANNEL="$2"
      shift 2
      ;;
    --auth)
      AUTH_TOKEN="$2"
      shift 2
      ;;
    *)
      echo "Error: Unknown option $1"
      echo "Use --help for usage information"
      exit 1
      ;;
  esac
done

# Check if authentication token is provided
if [ -z "$AUTH_TOKEN" ]; then
  echo "Error: Authentication token is required"
  echo "Please provide a token using --auth option or set the HAB_AUTH_TOKEN environment variable"
  echo "Use --help for usage information"
  exit 1
fi

# Export HAB_AUTH_TOKEN
echo "Using authentication token for Habitat"
export HAB_AUTH_TOKEN="$AUTH_TOKEN"

# Function to compare versions
# Returns 1 if first version is greater, 0 if equal, -1 if second is greater
compare_versions() {
  # Handle the case where versions are exactly the same
  if [[ "$1" == "$2" ]]; then
    echo 0
    return
  fi
  
  # Split versions by dots into arrays using read and IFS
  local IFS=.
  local -a version1=()
  local -a version2=()
  
  # Read version components into arrays properly
  read -ra version1 <<< "$1"
  read -ra version2 <<< "$2"
  
  # Get the length of the shorter version
  local max_len
  if [[ ${#version1[@]} -lt ${#version2[@]} ]]; then
    max_len=${#version1[@]}
  else
    max_len=${#version2[@]}
  fi
  
  # Compare each component
  for ((i=0; i<max_len; i++)); do
    # Clean the segments to ensure numeric comparison using parameter expansion
    local v1
    v1="${version1[$i]//[^0-9]/}"
    local v2
    v2="${version2[$i]//[^0-9]/}"
    
    # Default to 0 if empty
    v1=${v1:-0}
    v2=${v2:-0}
    
    # Convert to integers for comparison (with base 10 explicitly specified)
    local num1=$((10#$v1))
    local num2=$((10#$v2))
    
    # If components are different, we can determine version order
    if ((num1 > num2)); then
      echo 1  # First version is greater
      return
    elif ((num1 < num2)); then
      echo -1  # Second version is greater
      return
    fi
    # If components are equal, continue to the next component
  done
  
  # If we've compared all components of the shorter version and they're equal,
  # the version with more components is the newer one
  if [[ ${#version1[@]} > ${#version2[@]} ]]; then
    echo 1  # First version has more components
    return
  elif [[ ${#version1[@]} < ${#version2[@]} ]]; then
    echo -1  # Second version has more components
    return
  fi
  
  # If we get here, the versions are identical
  echo 0
}

# Script to install the latest chef/hab binary, chef/hab-sup and chef/hab-launcher from the specified channel

echo "Installing latest chef/hab from $CHANNEL channel..."
hab pkg install chef/hab -bf --channel="$CHANNEL" --auth="$AUTH_TOKEN"

# Check if either core/hab-sup or chef/hab-sup is already installed
if hab pkg list core/hab-sup 2>/dev/null | grep -q "core/hab-sup" || hab pkg list chef/hab-sup 2>/dev/null | grep -q "chef/hab-sup"; then
  if hab pkg list core/hab-sup 2>/dev/null | grep -q "core/hab-sup"; then
    echo "Found existing core/hab-sup installation. Installing chef/hab-sup and chef/hab-launcher..."
  elif hab pkg list chef/hab-sup 2>/dev/null | grep -q "chef/hab-sup"; then
    echo "Found existing chef/hab-sup installation. Updating chef/hab-sup and chef/hab-launcher..."
  fi
  
  # Get the running hab-sup version first (before installation)
  if pgrep -x "hab-sup" > /dev/null; then
    # Extract the version from the path shown in pgrep -a output
    # Example: "12345 /hab/pkgs/core/hab-sup/1.6.56/20220901123456/bin/hab-sup run"
    PROC_INFO=$(pgrep -a hab-sup)
    echo "Current process info: $PROC_INFO"
    # Extract the path part that includes the version
    # Result would be like: /hab/pkgs/core/hab-sup/1.6.56/20220901123456/bin/hab-sup
    HAB_SUP_PATH=$(echo "$PROC_INFO" | sed 's/^[0-9][0-9]* //' | awk '{print $1}')
    # Extract just the version part from path
    RUNNING_VERSION=$(echo "$HAB_SUP_PATH" | sed 's|/hab/pkgs/\(core\|chef\)/hab-sup/\([^/]*\)/.*|\2|')
    echo "Currently running hab-sup version: $RUNNING_VERSION"
  else
    RUNNING_VERSION="0.0.0"
    echo "No running hab-sup process found."
  fi
  
  echo "Installing latest chef/hab-sup from $CHANNEL channel..."
  hab pkg install --channel="$CHANNEL" --auth="$AUTH_TOKEN" chef/hab-sup

  echo "Installing latest chef/hab-launcher from $CHANNEL channel..."
  hab pkg install --channel="$CHANNEL" --auth="$AUTH_TOKEN" chef/hab-launcher
  
  # Get the newly installed version using find instead of ls for better handling of special characters
  NEW_VERSION=$(find /hab/pkgs/chef/hab-sup/ -maxdepth 1 -mindepth 1 -type d | sed 's|.*/||' | sort -V | tail -1)
  
  # Check if hab-sup systemd service is running and restart it if needed
  if command -v systemctl >/dev/null 2>&1 && systemctl is-active --quiet hab-sup; then
    # Compare versions
    echo "Newly installed hab-sup version: $NEW_VERSION"
    
    COMPARE_RESULT=$(compare_versions "$NEW_VERSION" "$RUNNING_VERSION")
    
    if [ "$COMPARE_RESULT" -gt 0 ]; then
      echo "New version is greater than currently running version. Preparing to restart service..."
      
      # Check if the systemd unit file needs to be updated with HAB_AUTH_TOKEN
      echo "Ensuring systemd unit file has the HAB_AUTH_TOKEN environment variable..."
      
      # Create override directory if it doesn't exist
      mkdir -p /etc/systemd/system/hab-sup.service.d/
      
      # Create or update the environment override file
      echo "Creating systemd override for environment variables..."
      echo -e "[Service]\nEnvironment=\"HAB_AUTH_TOKEN=$AUTH_TOKEN\"" | tee /etc/systemd/system/hab-sup.service.d/env-override.conf > /dev/null
      
      # Reload systemd to pick up the changes
      echo "Reloading systemd daemon..."
      systemctl daemon-reload
      
      echo "Restarting hab-sup service..."
      systemctl restart hab-sup
      echo "hab-sup service has been restarted."
      
      # Wait a moment for the service to fully start
      sleep 5
      
      # Validate that the running binary is from chef/hab-sup
      echo "Validating the running hab-sup binary..."
      if pgrep -x "hab-sup" > /dev/null; then
        HAB_SUP_PATH=$(readlink -f /proc/"$(pgrep -x "hab-sup")"/exe)
        if echo "$HAB_SUP_PATH" | grep -q "chef/hab-sup"; then
          echo "✓ Confirmed: The running hab-sup binary is from chef/hab-sup package: $HAB_SUP_PATH"
          
          # Verify environment variable is set in the service
          echo "Checking if HAB_AUTH_TOKEN is set in the running service..."
          if systemctl show hab-sup | grep -q "Environment=.*HAB_AUTH_TOKEN"; then
            echo "✓ Confirmed: HAB_AUTH_TOKEN environment variable is properly set in the service"
          else
            echo "⚠ Warning: HAB_AUTH_TOKEN does not appear to be set in the running service"
            echo "Please check systemd configuration for the hab-sup service"
          fi
        else
          echo "⚠ Warning: The running hab-sup binary is not from chef/hab-sup package: $HAB_SUP_PATH"
          echo "This may indicate that the migration was not successful or another version is still being used."
        fi
      else
        echo "⚠ Warning: hab-sup process is not running after service restart."
        echo "Please check the service status with: sudo systemctl status hab-sup"
      fi
    else
      echo "Currently running version $RUNNING_VERSION is the same or newer than the installed version. No restart needed."
    fi
  else
    echo "systemd not detected or hab-sup service is not running. No need to restart."
  fi
  
  echo "Migration complete. The latest Habitat components have been installed from the $CHANNEL channel."
else
  echo "No existing hab-sup installation found. Skipping chef/hab-sup and chef/hab-launcher installation."
  echo "Only chef/hab has been installed from the $CHANNEL channel."
fi