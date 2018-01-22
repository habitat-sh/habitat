@echo off
"powershell.exe" -NoProfile -ExecutionPolicy bypass -NoLogo -Command ". '%~dp0publish-studio.ps1'" %*
