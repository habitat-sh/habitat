<#
.SYNOPSIS
    Migrates Chef Habitat to the latest chef/hab* packages on Windows.

.DESCRIPTION
    This script migrates Chef Habitat to the latest chef/hab* packages.
    It updates hab.exe, hab-sup, and restarts the Windows Habitat service if needed.

    The script will install the latest chef/hab binary from the specified channel,
    followed by chef/hab-sup if hab-sup is already installed. If the Habitat
    Windows Service is running, it will also install chef/windows-service and
    may restart the service if the new version is greater than the current version.

.PARAMETER Channel
    Specifies the channel to install packages from.
    Default value is "acceptance".

.PARAMETER Auth
    Specifies the authentication token for license restricted packages.
    Uses the HAB_AUTH_TOKEN environment variable if not provided.

.PARAMETER Help
    Displays this help information.

.EXAMPLE
    .\migrate.ps1 -Auth "your-token"

    Migrates Habitat using the specified authentication token and the default acceptance channel.

.EXAMPLE
    .\migrate.ps1 -Channel stable -Auth "your-token"

    Migrates Habitat using the stable channel and the specified authentication token.

.NOTES
    - Authentication token is required (via -Auth or HAB_AUTH_TOKEN environment variable)
    - chef/hab-sup is only installed if hab-sup is already present
    - chef/windows-service is installed if the Habitat Windows Service is detected
    - The service restart only happens if the new version is greater than the currently running version
    - If no hab-sup process is running, the service will not be restarted
#>

param(
    [string]$Channel = "acceptance",
    [string]$Auth = $env:HAB_AUTH_TOKEN,
    [switch]$Help
)

# Display help if requested using standard PowerShell help
if ($Help) {
    Get-Help $MyInvocation.MyCommand.Path
    exit 0
}

# Check if authentication token is provided
if (-not $Auth) {
    Write-Host "Error: Authentication token is required" -ForegroundColor Red
    Write-Host "Please provide a token using -Auth parameter or set the HAB_AUTH_TOKEN environment variable"
    Write-Host "Use -Help for usage information"
    exit 1
}

# Set HAB_AUTH_TOKEN environment variable
Write-Host "Using authentication token for Habitat"
$env:HAB_AUTH_TOKEN = $Auth

# Function to compare versions
# Returns 1 if first version is greater, 0 if equal, -1 if second is greater
function Compare-HabitatVersion {
    param(
        [string]$Version1,
        [string]$Version2
    )

    # Handle the case where versions are exactly the same
    if ($Version1 -eq $Version2) {
        return 0
    }

    # Split versions by dots
    $v1Parts = $Version1.Split('.')
    $v2Parts = $Version2.Split('.')

    # Get the length of the shorter version
    $maxLen = [Math]::Min($v1Parts.Length, $v2Parts.Length)

    # Compare each component
    for ($i = 0; $i -lt $maxLen; $i++) {
        # Clean the segments to ensure numeric comparison
        $v1 = $v1Parts[$i] -replace '[^0-9]', ''
        $v2 = $v2Parts[$i] -replace '[^0-9]', ''

        # Default to 0 if empty
        if (-not $v1) { $v1 = 0 }
        if (-not $v2) { $v2 = 0 }

        # Convert to integers for comparison
        $num1 = [int]$v1
        $num2 = [int]$v2

        # If components are different, we can determine version order
        if ($num1 -gt $num2) {
            return 1  # First version is greater
        } elseif ($num1 -lt $num2) {
            return -1  # Second version is greater
        }
        # If components are equal, continue to the next component
    }

    # If we've compared all components of the shorter version and they're equal,
    # the version with more components is the newer one
    if ($v1Parts.Length -gt $v2Parts.Length) {
        return 1  # First version has more components
    } elseif ($v1Parts.Length -lt $v2Parts.Length) {
        return -1  # Second version has more components
    }

    # If we get here, the versions are identical
    return 0
}

# Script to install the latest chef/hab binary and chef/hab-sup from the specified channel

Write-Host "Installing latest chef/hab from $Channel channel..."
hab pkg install --binlink --force --channel="$Channel" --auth="$Auth" chef/hab
$chefhab = hab pkg path chef/hab
$habPath = Join-Path $env:ProgramData Habitat
if(Test-Path $habPath) { Remove-Item $habPath -Recurse -Force }
New-Item $habPath -ItemType Directory | Out-Null
Copy-Item "$chefhab\bin\*" $habPath

# Check if either core/hab-sup or chef/hab-sup is already installed
$habSupInstalled = $false
$coreHabSupOutput = hab pkg list core/hab-sup 2>$null
$chefHabSupOutput = hab pkg list chef/hab-sup 2>$null

if ($coreHabSupOutput -and $coreHabSupOutput -match "core/hab-sup") {
    $habSupInstalled = $true
    Write-Host "Found existing core/hab-sup installation. Installing chef/hab-sup..."
} elseif ($chefHabSupOutput -and $chefHabSupOutput -match "chef/hab-sup") {
    $habSupInstalled = $true
    Write-Host "Found existing chef/hab-sup installation. Updating chef/hab-sup..."
}

if ($habSupInstalled) {
    Write-Host "Installing latest chef/hab-sup from $Channel channel..."
    hab pkg install --channel="$Channel" --auth="$Auth" chef/hab-sup

    # Check if Habitat Windows service exists and install chef/windows-service if it does
    $habitatService = Get-Service -Name "Habitat" -ErrorAction SilentlyContinue
    if ($habitatService) {
        Write-Host "Found existing Habitat Windows service."

        # Update HAB_AUTH_TOKEN in HabService.dll.config before installing new windows-service
        Write-Host "Updating HAB_AUTH_TOKEN in config file..."

        # Path to the config file
        $configFilePath = Join-Path $env:SystemDrive "hab\svc\windows-service\HabService.dll.config"

        if (Test-Path $configFilePath) {
            try {
                # Load the config file as XML
                [xml]$configXml = Get-Content $configFilePath -ErrorAction Stop

                # Check if appSettings element exists
                $appSettings = $configXml.SelectSingleNode("//appSettings")
                if (-not $appSettings) {
                    # Create appSettings element if it doesn't exist
                    $appSettings = $configXml.CreateElement("appSettings")
                    $configXml.configuration.AppendChild($appSettings) | Out-Null
                }

                # Check if ENV_HAB_AUTH_TOKEN setting already exists
                $authTokenSetting = $appSettings.SelectSingleNode("//add[@key='ENV_HAB_AUTH_TOKEN']")
                if ($authTokenSetting) {
                    # Update existing setting
                    $authTokenSetting.SetAttribute("value", $Auth)
                    Write-Host "Updated existing ENV_HAB_AUTH_TOKEN setting in config file"
                } else {
                    # Create new add element for ENV_HAB_AUTH_TOKEN
                    $addElement = $configXml.CreateElement("add")
                    $addElement.SetAttribute("key", "ENV_HAB_AUTH_TOKEN")
                    $addElement.SetAttribute("value", $Auth)
                    $appSettings.AppendChild($addElement) | Out-Null
                    Write-Host "Added new ENV_HAB_AUTH_TOKEN setting to config file"
                }

                # Save changes to the config file
                $configXml.Save($configFilePath)
                Write-Host "Successfully updated HAB_AUTH_TOKEN in config file: $configFilePath" -ForegroundColor Green
            } catch {
                Write-Host "Error updating config file: $_" -ForegroundColor Yellow
                Write-Host "Service will be restarted, but HAB_AUTH_TOKEN may not be set correctly" -ForegroundColor Yellow
            }
        } else {
            Write-Host "Config file not found at: $configFilePath" -ForegroundColor Yellow
            Write-Host "Service will be restarted, but HAB_AUTH_TOKEN may not be set correctly" -ForegroundColor Yellow
        }

        # Now install the latest chef/windows-service package
        Write-Host "Installing latest chef/windows-service from $Channel channel..."
        hab pkg install --channel="$Channel" --auth="$Auth" chef/windows-service
        Write-Host "Successfully installed chef/windows-service package"
        # Wait a moment for the service to fully start if a new service was installed
        Start-Sleep -Seconds 5
    }

    # Get the running hab-sup version (after installations)
    $runningVersion = "0.0.0"
    $habProcess = Get-Process -Name "hab-sup" -ErrorAction SilentlyContinue
    $habSupRunning = $false

    if ($habProcess) {
        $habSupRunning = $true
        $habSupPath = (Get-Process -Name "hab-sup").Path
        if ($habSupPath -match '(?:core|chef)\\hab-sup\\([^/\\]+)') {
            $runningVersion = $Matches[1]
            Write-Host "Currently running hab-sup version: $runningVersion"
        } else {
            Write-Host "Warning: Could not determine running hab-sup version from path: $habSupPath"
        }
    } else {
        Write-Host "No running hab-sup process found. Service will not be restarted."
    }

    # Get the newly installed version using hab pkg path
    try {
        $pkgPath = hab pkg path chef/hab-sup
        if ($pkgPath -match 'chef\\hab-sup\\([^/\\]+)') {
            $newVersion = $Matches[1]
            Write-Host "Newly installed hab-sup version: $newVersion"

            # Check if Habitat Windows service is running and restart it if needed
            if ($habitatService -and $habSupRunning) {
                $compareResult = Compare-HabitatVersion -Version1 $newVersion -Version2 $runningVersion

                if ($compareResult -gt 0) {
                    Write-Host "New version is greater than currently running version. Preparing to restart service..."

                    Write-Host "Restarting Habitat service..."
                    Restart-Service -Name "Habitat" -Force
                    Write-Host "Habitat service has been restarted."

                    # Wait a moment for the service to fully start
                    Start-Sleep -Seconds 5

                    # Validate that the service is running
                    $serviceStatus = Get-Service -Name "Habitat" -ErrorAction SilentlyContinue
                    if ($serviceStatus.Status -eq "Running") {
                        Write-Host "Confirmed: Habitat service is running" -ForegroundColor Green

                        # Get the running hab-sup process and check its path
                        $newHabProcess = Get-Process -Name "hab-sup" -ErrorAction SilentlyContinue
                        if ($newHabProcess) {
                            $newHabSupPath = $newHabProcess.Path
                            if ($newHabSupPath -match "chef\\hab-sup") {
                                Write-Host "Confirmed: The running hab-sup binary is from chef/hab-sup package: $newHabSupPath" -ForegroundColor Green
                                Write-Host "Habitat service is running with the updated configuration" -ForegroundColor Green
                            } else {
                                Write-Host "Warning: The running hab-sup binary is not from chef/hab-sup package: $newHabSupPath" -ForegroundColor Yellow
                                Write-Host "This may indicate that the migration was not successful or another version is still being used."
                            }
                        } else {
                            Write-Host "Warning: hab-sup process is not running after service restart." -ForegroundColor Yellow
                            Write-Host "Please check the service status with: Get-Service -Name Habitat"
                        }
                    } else {
                        Write-Host "Warning: Habitat service is not running after restart attempt." -ForegroundColor Yellow
                        Write-Host "Please check the service status with: Get-Service -Name Habitat"
                    }
                } else {
                    Write-Host "Currently running version $runningVersion is the same or newer than the installed version $newVersion. No restart needed." -ForegroundColor Cyan
                }
            } elseif (!$habSupRunning) {
                Write-Host "Habitat service exists but no supervisor process is running. Skipping restart." -ForegroundColor Cyan
            } else {
                Write-Host "Habitat Windows service not found. No need to restart."
            }
        } else {
            Write-Host "Warning: Could not determine version from path: $pkgPath" -ForegroundColor Yellow
        }
    } catch {
        Write-Host "Warning: Could not find chef/hab-sup package. $($_)" -ForegroundColor Yellow
    }

    Write-Host "Migration complete. The latest Habitat components have been installed from the $Channel channel."
} else {
    Write-Host "No existing hab-sup installation found. Skipping chef/hab-sup installation."
    Write-Host "Only chef/hab has been installed from the $Channel channel."
}