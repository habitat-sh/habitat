function Install-HabService {
	if((Get-Service Habitat -ErrorAction SilentlyContinue) -ne $null) {
		Write-Error "The Habitat service is already installed. Please run 'hab exec core/windows-service uninstall' first if you wish to reinstall."
		return
	}

	if(!(Test-Path (Join-Path $env:SystemDrive "hab\pkgs\core\hab-sup"))) {
		$habProc = Get-Process hab -ErrorAction SilentlyContinue
		if(!$habProc) {
			Write-Error "Could not locate the Habitat CLI. Make sure you are running this via 'hab pkg exec core/windows-service install'."
			return
		}
		$habExe = $habProc[0].Path
		& $habExe pkg install core/hab-sup
	}

	$svcPath = Join-Path $env:SystemDrive "hab\svc\windows-service"
	if(!(Test-Path $svcPath)) {
		mkdir $svcPath
	}

	Copy-Item "$PSScriptRoot\*" $svcPath -Force

	&$env:systemroot\system32\sc.exe create Habitat binpath= "$svcPath\HabService.exe" start= auto
	if($LASTEXITCODE -ne 0) {
	    Write-Error "Failed to install the Habitat Service!"
	}
	else {
		&$env:systemroot\system32\sc.exe description Habitat "The Habitat Supervisor service"
		Write-Host "Congratulations! The Habitat Service has succesfully been installed!"
	}
}

function Uninstall-HabService {
	if((Get-Service Habitat -ErrorAction SilentlyContinue) -eq $null) {
		Write-Error "The Habitat service is not installed."
		return
	}

	$svcPath = $env:SystemDrive -join "hab\svc\windows-service"
	Stop-Service Habitat
	while((Get-Service Habitat).Status -ne "Stopped") {
		Start-Sleep -Seconds 1
	}

	&$env:systemroot\system32\sc.exe delete Habitat

	if($LASTEXITCODE -ne 0) {
	    Write-Error "Failed to uninstall the Habitat Service!"
	}
	else {
		Write-Host "The Habitat Service has succesfully been uninstalled!"
	}
}