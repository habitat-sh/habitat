---
title: "A Plan for Microsoft SQL Server"
date: 2018-01-03
author: Matt Wrock
tags: windows, packaging
category: windows
classes: body-article
---

If you develop or support applications on Windows, chances are that you have crossed paths or even worked deeply with Microsoft's SQL Server database. In the 14 years I spent as a .Net developer, its by far the database I worked with most extensively.

I've been using MySQL for all of my .Net Habitat demos so far. It is open source, fairly light weight, and its binaries are easily portable and can be launched by simply running mysqld.exe. So it makes for a straightforward demo. But what if like the majority of .Net shops, you work with SQL Server and would like to understand how it might run inside of Habitat? Well this post is for you!

## Challenges of Habitizing SQL Server

SQL Server has a few characteristics that make it more difficult to package and run in Habitat than other commercial software.

The SQL Server binaries (like sqlservr.exe and its supporting libraries) cannot simply be extracted and moved to `/hab/pkgs/`. SQL Server makes extensive use of the Windows registry which tracks the location on disk of all SQL Server "instances" along with much of their service metadata. Also, the absolute paths to the data files of the `model` and `msdb` system databases are stored in the `master` database. So in order to reliably install SQL Server in working condition, you need to do so via its installer binaries. Unfortunately this also means you may need to package the installer binaries which weigh in at about 1.5 GB in SQL Server 2017.

Another challenge is that your Habitat `run` hook cannot simply call `sqlservr.exe`. On the surface it seems like that should work since that is what the Windows service invokes and if you run that from a shell, it succesfully starts and runs the database engine. However, `sqlservr` does not handle `ctrl+c` signals gracefully. It wil prompt you for confirmation to terminate the database and to my knowledge that cannot be suppressed. We could forcibly terminate the process, but that may leave the database in an unrecoverable state. So we do need to have our hooks interact with the service.

All of these challenges can be dealt with so lets jump in and explore a working SQL Server Habitat plan. I'll be including the relevant sample Habitat plan and hook code in this post but please see [this github repository](https://github.com/mwrock/habitat-sql-server) for the complete Habitat artifacts.

## Packaging Sql Server

SQL Server is a commercial product. You can download an evaluation copy of the database [here](https://www.microsoft.com/en-us/sql-server/sql-server-2017) that will operate for a limited period of time. The complete install media is typically distributed via a `.cab` or `.iso` file and occupies about 1.5 GB of disk space. ~~So its not likely you will be using a `sqlserver` plan from the Habitat [`core-plans`](https://github.com/habitat-sh/core-plans) repo. You will need to build your own package that either includes or points to your own purchased install media.~~

**Update**: We now host a `core/sqlserver` plan in our core-plans repository. This plan will download the free SQL Server Express edition and allow you to specify your own install media in case you want to run your Standard or Enterprise edition.

Here is a plan one might use as a reference for their own SQL Server plan. The `plan.ps1` file is extremely simple because it merely copies the install files into the package. It doesn't actually install SQL Server because that will happen the very first time the `init` hook runs as we will see later.

Here is the plan:

```ps1 title:'habitat-sql-server/plan.ps1' link: https://github.com/mwrock/habitat-sql-server/blob/master/plan.ps1
$pkg_name = "sqlserver"
$pkg_origin = "mwrock"
$pkg_version = "0.1.0"
$pkg_maintainer = "The Habitat Maintainers <humans@habitat.sh>"
$pkg_exports = @{
  "port"="port"
}
$pkg_description = "Microsoft SQL Server 2017"
$pkg_upstream_url = "https://www.microsoft.com/en-us/sql-server/sql-server-2017"
$pkg_bin_dirs = @("bin")

$setupDir = "d:"

function Invoke-Install {
  Copy-Item "$setupDir/*" $pkg_prefix/bin -Recurse
}
```

This assumes that you have the install media mounted to your `d:` drive. Another approach one could take would be to not package the install files at all and instead have the `init` script point to a network share instead of a huge local payload.

## Installing SQL Server via the `init` Hook

So our plan really did not do anything other than "stage" the install files. It's our `init` script that will do the heavy lifting. Its only going to need to run the installer the first time we start our Habitat service. Be aware that it may take several minutes for that first `init` hook to complete.

Our `init` hook will do two things:

### Run the Installer

```ps1 title:'habitat-sql-server/hooks/init' linenos:true link: https://github.com/mwrock/habitat-sql-server/blob/master/hooks/init
# If the sql instance data is not present, install a new instance
if (!(Test-Path {{pkg.svc_data_path}}/mssql14.{{pkg.name}})) {
    setup.exe /configurationfile={{pkg.svc_config_path}}/config.ini /Q
}
```

If it does not find the data files in our Habitat `svc_data_path` then it will run `setup.exe` and use our templatized `config.ini` as the installer inputs:

```handlebars title:'habitat-sql-server/config/config.ini' link: https://github.com/mwrock/habitat-sql-server/blob/master/config/config.ini
[OPTIONS]
ACTION="Install"
IACCEPTSQLSERVERLICENSETERMS=""
UpdateEnabled="0"
FEATURES="SQLEngine"
INSTANCEID="{{pkg.name}}"
INSTANCENAME="{{pkg.name}}"
AGTSVCSTARTUPTYPE="Manual"
BROWSERSVCSTARTUPTYPE="Manual"
INSTALLSQLDATADIR="{{pkg.svc_data_path}}"
SQLSYSADMINACCOUNTS="Administrator"
SQLSVCACCOUNT="NT AUTHORITY\Network Service"
SQLSVCSTARTUPTYPE="Manual"
SQLBACKUPDIR="{{pkg.svc_data_path}}\backup"
SQLTEMPDBDIR="{{pkg.svc_data_path}}"
SQLTEMPDBLOGDIR="{{pkg.svc_data_path}}"
SQLUSERDBDIR="{{pkg.svc_data_path}}"
SQLUSERDBLOGDIR="{{pkg.svc_data_path}}"
TCPENABLED="1"
SQLSYSADMINACCOUNTS="Administrator"
SECURITYMODE="SQL"
SAPWD="{{cfg.sa_password}}"
```

We only install the SQLEngine feature assuming we just need basic database services and no reporting, OLAP, or other fancy stuff. We also set the `INSTANCEID` and `INSTANCENAME` to match our package name. This grants us some portability. If we were to deploy this to a machine that already had SQL Server installed, this adds another named instance unique to our package name.

### Set the Port and Open the Firewall

By default SQL Server listens on port 1433 and then forwards requests to a dynamically allocated port for the targeted named instance. This assumes the `SQLBrowser` service is running, but we don't want to depend on that if we can avoid it since Habitat is not running it. Instead we can assign our instance a static port and then our applications can send requests to `<hostname>,<port>`. One typically assigns static ports using the SQL Configuration Manager GUI, but we can acomplish the same end by setting a registry key:

```ps1 title:'habitat-sql-server/hooks/init' linenos:true start:6 link: https://github.com/mwrock/habitat-sql-server/blob/master/hooks/init
# Configure the instance for the configured port
Set-ItemProperty -Path  "HKLM:\SOFTWARE\Microsoft\Microsoft SQL Server\MSSQL14.{{pkg.name}}\MSSQLServer\SuperSocketNetLib\Tcp\IPAll" -Name TcpPort -Value {{cfg.port}}
```

Then we will use DSC (Desired State Configuration) to ensure our local firewall allows inbound traffic on that port. We define our DSC configuration in `config/firewall.ps1`:

```ps1 title: 'habitat-sql-server/config/firewall.ps1' link: https://github.com/mwrock/habitat-sql-server/blob/master/config/firewall.ps1
Configuration NewFirewallRule
{
    Import-DscResource -Module xNetworking
    Node 'localhost' {
        xFirewall "sqlserver-{{pkg.name}}"
        {
            Name   = "sqlserver-{{pkg.name}}"
            DisplayName = "sqlserver-{{pkg.name}}"
            Action = "Allow"
            Direction = "InBound"
            LocalPort = ("{{cfg.port}}")
            Protocol = "TCP"
            Ensure = "Present"
            Enabled  = "True"
        }
    }
}
```

And our `init` hook invokes this:

```ps1 title:'habitat-sql-server/hooks/init' linenos:true start:10 link: https://github.com/mwrock/habitat-sql-server/blob/master/hooks/init
Invoke-Command -ComputerName localhost -EnableNetworkAccess {    Write-Host "Checking for xNetworking PS module..."
    $ProgressPreference="SilentlyContinue"
    Write-Host "Checking for nuget package provider..."
    if(!(Get-PackageProvider -Name nuget -ErrorAction SilentlyContinue -ListAvailable)) {
        Write-Host "Installing Nuget provider..."
        Install-PackageProvider -Name NuGet -Force | Out-Null
    }
    Write-Host "Checking for xNetworking PS module..."
    if(!(Get-Module xNetworking -ListAvailable)) {
        Write-Host "Installing xNetworking PS Module..."
        Install-Module xNetworking -Force | Out-Null
    }
}


Import-Module "{{pkgPathFor "core/dsc-core"}}/Modules/DscCore"
Start-DscCore (Join-Path {{pkg.svc_config_path}} firewall.ps1) NewFirewallRule

```

This leverages our `core/dsc` package which makes it easy to apply a DSC configuration in Habitat's Powershell core environment. Note that we use `Invoke-Command` to wrap the installation of our `xNetworking` DSC module so that it is installed into the Windows Powershell context and not Powershell Core.

## Running the Sql Server Service

Our `run` hook is going to start the `MSSQL` service installed for our SQL named instance and then spin until the service is stopped:

```ps1 title:'habitat-sql-server/hooks/run' link: https://github.com/mwrock/habitat-sql-server/blob/master/hooks/run
Start-Service 'MSSQL${{pkg.name}}'
Write-Host "{{pkg.name}} is running"

try {
    while($(Get-Service 'MSSQL${{pkg.name}}').Status -eq "Running") {
        Start-Sleep -Seconds 1
    }
}
finally {
    if($(Get-Service 'MSSQL${{pkg.name}}').Status -ne "Stopped") {
        Write-Host "{{pkg.name}} stopping..."
        Stop-Service 'MSSQL${{pkg.name}}'
        Write-Host "{{pkg.name}} has stopped"
    }
}
```

The service name will always be suffixed with our instance name which we have set to be equal to our package name.

## Setting up Logins and Users

After Sql Server is installed we have an all powerful `sa` (Systam Administrator) user and the local `Administrator` Windows user is designated an admin user in our config.ini. We don't want our application to access the database as the `sa` user. We'll add a `post-run` hook that will run after Habitat starts our `sqlserver` service and it will ensure an application user and password are configured:

```ps1 title:'habitat-sql-server/hooks/post-run' linenos:true start:10 link: https://github.com/mwrock/habitat-sql-server/blob/master/hooks/post-run
# Create application Users

Write-Host "Starting application user setup..."

."$env:ProgramFiles\Microsoft SQL Server\140\Tools\Binn\OSQL.EXE" -S localhost,{{cfg.port}} -U sa -P {{cfg.sa_password}} -Q "create login {{cfg.app_user}} with password = '{{cfg.app_password}}'"
."$env:ProgramFiles\Microsoft SQL Server\140\Tools\Binn\OSQL.EXE" -S localhost,{{cfg.port}} -U sa -P {{cfg.sa_password}} -Q "create user {{cfg.app_user}} for login {{cfg.app_user}}"
."$env:ProgramFiles\Microsoft SQL Server\140\Tools\Binn\OSQL.EXE" -S localhost,{{cfg.port}} -U sa -P {{cfg.sa_password}} -Q "grant CREATE DATABASE to {{cfg.app_user}}"

Write-Host "Application user setup complete"
```

This uses the `osql` utility installed with any basic SQLEngine install to execute commands that creates an application login and sets up that login as a user of the `master` database and gives it the power to create a database.

Now that might seem like an overly powerful privilege for an application user and honestly it would be in a real production setup. However I am going to run Entity Framework migrations with this user and it needs this right to create an initial database for my application.

## Adding a Post-Stop Hook

In most cases, the `finally` block of our `run` hook will stop our SQL Server instance service when we ask the Supervisor to stop the `sqlserver` service. The Supervisor issues a `ctrl+debug` signal to our service process and Powershell will ensure that the `finally` block is called on the running pipeline. However there are somne isolated scenarios, like running in a Windows container, where `ctrl+debug` signals are not propperly generated and the `finally` block will not be called. We can fortify ourselves against this with a `post-stop` hook. This is called when we stop a service to perform any necessary cleanup. So we will just check to see if the service is still running and stop it if it is:

```ps1 title:'habitat-sql-server/hooks/post-stop' linenos:true start:10 link: https://github.com/mwrock/habitat-sql-server/blob/master/hooks/post-stop
if($(Get-Service 'MSSQL${{pkg.name}}').Status -ne "Stopped") {
    Write-Host "{{pkg.name}} stopping..."
    Stop-Service 'MSSQL${{pkg.name}}'
    Write-Host "{{pkg.name}} has stopped"
}
```

## Testing Connectivity to our "Habitized" SQL Server

Lets see if we can connect to our database "from the outside" using our application user defined in our `default.toml`:

```toml title:'habitat-sql-server/default.toml' link: https://github.com/mwrock/habitat-sql-server/blob/master/default.toml
sa_password="Pass@word1"
port=8888
app_user = "hab"
app_password="h@b1Tat"
```

To test this I have started this `sqlserver` Habitat service in a vm running on `192.168.137.88`. Now I'll `cd` to a local .Net Core application that has the following `appsettings.json`:

```json title:appsettings.json
{
  "server.urls": "http://*:5123",
  "ConnectionStrings": {
    "DefaultConnection": "server=192.168.137.88,8888;uid=hab;pwd=h@b1Tat;database=habitat_aspnet_sample;"
  }
}
```

So I am going to run the Entity Framework migrations which I expect to create my application's database and schema:

```powershell
C:\dev\habitat-aspnet-sample [master +2 ~4 -2 !]> dotnet ef database update

Build succeeded.
    0 Warning(s)
    0 Error(s)

Time Elapsed 00:00:02.39
Done.
```

The succinct "Done" indicate that all of our database creating dreams have come true. Lets just check to be sure:

```shell
1> select name from sys.databases
2> go
 name
 --------
 master
 tempdb
 model
 msdb
 habitat_aspnet_sample

(5 rows affected)
```

Ohhhh yeaaaahhhhh.
