. $PSScriptRoot\..\shared.ps1

$env:HAB_LICENSE = "accept-no-persist"
Install-Habitat

@('core/psscriptanalyzer', 'core/powershell') | ForEach-Object {
    $pkg = $_
    if(!(hab pkg list $pkg | Where-Object { $_.StartsWith($pkg)})) {
        hab pkg install $pkg
    }
}

& "$(hab pkg path core/powershell)\bin\pwsh" -WorkingDirectory $PSScriptRoot -Command {
    Import-Module "$(hab pkg path core/psscriptanalyzer)\module\PSScriptanAlyzer.psd1"

    $excludeScripts = @(
        'plan.ps1',
        'template_plan.ps1',
        'last_build.ps1',
        'pre_build.ps1'
    )
    Get-ChildItem ..\..\..\*.ps1 -Recurse -Exclude $excludeScripts |
        Invoke-ScriptAnalyzer -Settings .\PSScriptAnalyzerSettings.psd1 -EnableExit
    Get-ChildItem ..\..\..\plan.ps1 -Recurse | Invoke-ScriptAnalyzer -ExcludeRule PSUseDeclaredVarsMoreThanAssignments -EnableExit
}
