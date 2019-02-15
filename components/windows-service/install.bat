@echo off
pwsh.exe -NoProfile -ExecutionPolicy bypass -NoLogo -Command ". '%~dp0habitat.ps1';Install-HabService" %*
