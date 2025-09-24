[Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSUseCorrectCasing', '')]
param()

Describe "Migrate habitat using migrate.ps1" {
    BeforeAll {
        # Install the core/hab package from stable channel
        Write-Host "Installing core/hab from stable channel..."
        Invoke-Expression "& { $(Invoke-RestMethod https://raw.githubusercontent.com/habitat-sh/habitat/master/components/hab/install.ps1) }" | Out-Null
        if (-not $?) {
            throw "Failed to install core/hab"
        }

        # Install core/hab-sup
        Write-Host "Installing core/hab-sup from stable channel..."
        hab pkg install core/hab-sup --channel=stable
        if (-not $?) {
            throw "Failed to install core/hab-sup"
        }

        # Install core/windows-service
        Write-Host "Installing core/windows-service from stable channel..."
        hab pkg install core/windows-service --channel=stable
        if (-not $?) {
            throw "Failed to install core/windows-service"
        }

        # Start the service
        Write-Host "Starting Habitat Windows service..."
        Start-Service -Name "Habitat"
        Start-Sleep -Seconds 10
    }

    It "verifies core packages are installed before migration" {
        # Check for core/hab
        $habPath = (Get-Command hab).Path
        $habPath | Should -Exist

        # Verify hab-sup is installed from core origin
        $coreHabSupOutput = hab pkg list core/hab-sup
        $coreHabSupOutput | Should -Match "core/hab-sup"

        # Verify windows-service is installed from core origin
        $coreWindowsServiceOutput = hab pkg list core/windows-service
        $coreWindowsServiceOutput | Should -Match "core/windows-service"

        # Verify the Habitat service is running
        $habitatService = Get-Service -Name "Habitat" -ErrorAction SilentlyContinue
        $habitatService | Should -Not -BeNullOrEmpty
        $habitatService.Status | Should -Be "Running"

        # Verify hab-sup process is running
        $habSup = Get-Process -Name "hab-sup" -ErrorAction SilentlyContinue
        $habSup | Should -Not -BeNullOrEmpty
    }

    It "successfully migrates from core to chef packages" {
        # Store the pre-migration hab-sup version for comparison
        $preMigrationVersion = $null
        $habSup = Get-Process -Name "hab-sup" -ErrorAction SilentlyContinue
        if ($habSup) {
            $habSupPath = $habSup.Path
            if ($habSupPath -match 'core\\hab-sup\\([^/\\]+)') {
                $preMigrationVersion = $Matches[1]
                Write-Host "Pre-migration hab-sup version: $preMigrationVersion"
            }
        }

        # Run the migration script with test auth token
        components/hab/migrate.ps1
        $LASTEXITCODE | Should -Be 0

        # Check that hab is still installed
        $habPath = (Get-Command hab).Path
        $habPath | Should -Exist

        # Verify hab-sup is now installed from chef origin
        $chefHabSupOutput = hab pkg list chef/hab-sup
        $chefHabSupOutput | Should -Match "chef/hab-sup"

        # Verify windows-service is now installed from chef origin
        $chefWindowsServiceOutput = hab pkg list chef/windows-service
        $chefWindowsServiceOutput | Should -Match "chef/windows-service"

        # Verify the Habitat service is still running
        $habitatService = Get-Service -Name "Habitat" -ErrorAction SilentlyContinue
        $habitatService | Should -Not -BeNullOrEmpty
        $habitatService.Status | Should -Be "Running"

        # Verify hab-sup process is still running
        $habSup = Get-Process -Name "hab-sup" -ErrorAction SilentlyContinue
        $habSup | Should -Not -BeNullOrEmpty

        # Verify the running hab-sup is from the chef origin
        $habSupPath = $habSup.Path
        $habSupPath | Should -Match "chef\\hab-sup"

        # Verify version is same or newer after migration
        if ($preMigrationVersion) {
            $habSupPath -match 'chef\\hab-sup\\([^/\\]+)' | Should -Be $true
            $postMigrationVersion = $Matches[1]
            Write-Host "Post-migration hab-sup version: $postMigrationVersion"

            # Convert versions to [version] objects for proper comparison
            $preVersion = [version]($preMigrationVersion -replace '-.*$', '')
            $postVersion = [version]($postMigrationVersion -replace '-.*$', '')

            # Post-migration version should be same or newer
            $postVersion -ge $preVersion | Should -Be $true
        }
    }

    It "does not restart hab-sup when migration is run a second time" {
        # Store the current hab-sup process ID
        $habSupBefore = Get-Process -Name "hab-sup" -ErrorAction SilentlyContinue
        $habSupBefore | Should -Not -BeNullOrEmpty
        $pidBefore = $habSupBefore.Id
        $habSupPathBefore = $habSupBefore.Path
        Write-Host "Current hab-sup PID before second migration: $pidBefore"
        Write-Host "Current hab-sup path before second migration: $habSupPathBefore"

        # Verify that hab-sup is currently using chef/hab-sup
        $habSupPathBefore | Should -Match "chef\\hab-sup"

        # Get the current version before the second migration
        $versionBefore = $null
        if ($habSupPathBefore -match 'chef\\hab-sup\\([^/\\]+)') {
            $versionBefore = $Matches[1]
            Write-Host "Current hab-sup version before second migration: $versionBefore"
        }

        # Run the migration script with test auth token a second time
        Write-Host "Running migration script a second time..."
        components/hab/migrate.ps1
        $LASTEXITCODE | Should -Be 0

        # Check the hab-sup process after the second migration
        Start-Sleep -Seconds 5 # Give a moment for any potential service restart
        $habSupAfter = Get-Process -Name "hab-sup" -ErrorAction SilentlyContinue
        $habSupAfter | Should -Not -BeNullOrEmpty
        $pidAfter = $habSupAfter.Id
        $habSupPathAfter = $habSupAfter.Path
        Write-Host "Current hab-sup PID after second migration: $pidAfter"
        Write-Host "Current hab-sup path after second migration: $habSupPathAfter"

        # Verify that the hab-sup PID has not changed (no restart occurred)
        $pidAfter | Should -Be $pidBefore

        # Get the version after the second migration
        $versionAfter = $null
        if ($habSupPathAfter -match 'chef\\hab-sup\\([^/\\]+)') {
            $versionAfter = $Matches[1]
            Write-Host "Current hab-sup version after second migration: $versionAfter"
        }

        # Verify that the version is the same
        $versionAfter | Should -Be $versionBefore

        # Double-check that the service is still running properly
        $habitatService = Get-Service -Name "Habitat" -ErrorAction SilentlyContinue
        $habitatService | Should -Not -BeNullOrEmpty
        $habitatService.Status | Should -Be "Running"
    }
}