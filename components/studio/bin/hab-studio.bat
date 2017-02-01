@echo off
"%~dp0powershell/powershell.exe" -NoProfile -ExecutionPolicy bypass -NoLogo -Command ". '%~dp0hab-studio.ps1'" %*
