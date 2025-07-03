. $PSScriptRoot\..\shared.ps1

$env:HAB_LICENSE = "accept-no-persist"
Install-Habitat

@('core/psscriptanalyzer', 'core/powershell') | ForEach-Object {
    $pkg = $_
    if (!(hab pkg list $pkg | Where-Object { $_.StartsWith($pkg) })) {
        hab pkg install $pkg
    }
}

& "$(hab pkg path core/powershell)\bin\pwsh" -WorkingDirectory $PSScriptRoot -Command {
    Import-Module (Join-Path -Path "$(hab pkg path core/psscriptanalyzer)" -ChildPath "module\PSScriptAnalyzer.psd1")

    if (!$?) {
        exit 1
    }

    $excludeAnalyzeScripts = @(
        'plan.ps1',
        'template_plan.ps1',
        'last_build.ps1',
        'pre_build.ps1',
        'run.ps1',
        'init.ps1'
    )
    $excludeFormatScripts = @(
        'template_plan.ps1',
        'last_build.ps1',
        'pre_build.ps1',
        'run.ps1',
        'init.ps1'
    )

    Write-Host '$PSVersionTable reports...'
    $PSVersionTable

    Write-Host "Checking Powershell formatting..."
    # Excluding PSUseConsistentWhitespace because it conflicts with AlignAssignmentStatement and
    # PSUseConsistentIndentation which are higher value
    $formatErrors = Get-ChildItem ..\..\..\*.ps1 -Recurse -Exclude $excludeFormatScripts |
        Invoke-ScriptAnalyzer -Settings CodeFormattingOTBS -ExcludeRule PSUseConsistentWhitespace
    Write-Host ($formatErrors | Out-String)
    Write-Host "$($formatErrors.Count) errors found"

    Write-Host "Analyzing Powershell Scripts..."
    $analysisErrors = Get-ChildItem ..\..\..\*.ps1 -Recurse -Exclude $excludeAnalyzeScripts |
        Invoke-ScriptAnalyzer -Settings PSScriptAnalyzerSettings.psd1
    Write-Host ($analysisErrors | Out-String)
    Write-Host "$($analysisErrors.Count) errors found"

    Write-Host "Analyzing Powershell Habitat plans..."
    $planErrors = Get-ChildItem ..\..\..\plan.ps1 -Recurse | Invoke-ScriptAnalyzer -ExcludeRule PSUseDeclaredVarsMoreThanAssignments
    Write-Host ($planErrors | Out-String)
    Write-Host "$($planErrors.Count) errors found"

    Exit $formatErrors.Count + $analysisErrors.Count + $planErrors.Count
}

Exit $LASTEXITCODE
