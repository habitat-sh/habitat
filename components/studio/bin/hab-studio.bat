@echo off
"%~dp0powershell/pwsh.exe" -NoProfile -ExecutionPolicy bypass -NoLogo -File "%~dp0hab-studio.ps1" %*
