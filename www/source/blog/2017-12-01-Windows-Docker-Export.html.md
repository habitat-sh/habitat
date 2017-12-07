---
title: "Exporting Windows Packages to Docker"
date: 2017-12-01
author: Matt Wrock
tags: windows, supervisor
category: windows
classes: body-article
---

Yesterday's Habitat 0.50.0 release introduces the ability to export a Windows package to a Docker Windows container. This is great for multiple reasons! For starters:

* You can easily test multi supervisor configurations without provisioning VMs or fiddling with alternate supervisor ports.
* Habitat becomes a great onboarding path to containerizing your Windows applications.
* The containerization gives you an added level of isolation and repeatability - Currently you can assume a vanilla Windows 2016 Server Core base image and don't need to guess what dependencies or other services may have snuck on to that instance.

We use `microsoft/windowsservercore` as our base image. So if your package needs features or other Windows settings not included in that default image, make sure your `init` hook installs them.

Lets walk through containerizing a "Habitized" ASP.NET Core application that accesses a MySQL database. We will create two images: one for the web application and the other for the database. Then we will run both and get them talking to each other.

## What you Need to Get Started

You will need Habitat version 0.50.0 at a minimum and be on a Windows host running [Docker Community Edition for Windows](https://store.docker.com/editions/community/docker-ce-desktop-windows). Also, you must be "switched" to Windows containers.

We'll just be pulling the ASP.NET and MySQL packages from the [Habitat Depot](https://bldr.habitat.sh/#/explore) and will not be building them here. The source for this application can be found in [this github repo](https://github.com/habitat-sh/habitat-aspnet-sample).

## Export the MySQL Database to a Docker Image

Again, make sure you run this on Windows, otherwise you will be building a Linux image.

```console
hab pkg export docker core/mysql
```

Note that if you have never done a `docker pull` on `microsoft/windowsservercore` before, your first Windows container export may take a very long time since this is what we use as the base image and it's about a 5GB download.

## Export the ASP.NET Core Application

```console
hab pkg export docker core/habitat-aspnet-sample
```

Now a `docker image ls` should include something like this:

```console
REPOSITORY                                      TAG                     IMAGE ID
CREATED             SIZE
core/mysql                                      5.7.17                  aa93cd96fd21
2 hours ago         12.5GB
core/mysql                                      5.7.17-20170315123622   aa93cd96fd21
2 hours ago         12.5GB
core/mysql                                      latest                  aa93cd96fd21
2 hours ago         12.5GB
core/habitat-aspnet-sample                      0.2.0                   1a1816ecb4f7
2 hours ago         10.7GB
core/habitat-aspnet-sample                      0.2.0-20170703165439    1a1816ecb4f7
2 hours ago         10.7GB
core/habitat-aspnet-sample                      latest                  1a1816ecb4f7
2 hours ago         10.7GB
```

## Start MySQL

```console
docker run -p 3306:3306 -it core/mysql
```

This should produce something like:

```console
hab-sup(MR): Supervisor Member-ID 69c41a05c5ba44109b7e0f9c00b93ad1
hab-sup(MR): Starting core/mysql
hab-sup(MR): Starting gossip-listener on 0.0.0.0:9638
hab-sup(MR): Starting http-gateway on 0.0.0.0:9631
mysql.default(HK): init, compiled to C:/hab/svc\mysql\hooks\init
mysql.default(HK): run, compiled to C:/hab/svc\mysql\hooks\run
mysql.default(HK): Hooks compiled
mysql.default(SR): Hooks recompiled
default(CF): Updated init.sql 8db8e9979876f6fd29559ec0e4ab35c9964dc8cb6001e450e66cd2d96c7e4a60
default(CF): Updated my.cnf 0142697a2d8284d415b05f6bca27607a0911b92c3fe3048889fdd640e6ca3578mysql.default(SR): Configuration recompiled
mysql.default(SR): Initializing
mysql.default hook[init]:(HK):
mysql.default hook[init]:(HK):
mysql.default hook[init]:(HK):     Directory: C:\hab\svc\mysql\var
mysql.default hook[init]:(HK):
mysql.default hook[init]:(HK):
mysql.default hook[init]:(HK): Mode                LastWriteTime         Length Name

mysql.default hook[init]:(HK): ----                -------------         ------ ----

mysql.default hook[init]:(HK): d-----       11/29/2017   1:46 PM                logs

mysql.default hook[init]:(HK):
mysql.default hook[init]:(HK):
mysql.default(SV): Starting service as user=containeradministrator, group=
```

## Populate the Database with the Initial Schema

We now have an empty MySQL database running. Before our application can succesfully interact with it, we need to migrate our schema to the running instance. We will use the [EF Core .NET Command-line Tools](https://docs.microsoft.com/en-us/ef/core/miscellaneous/cli/dotnet) to perform the migration. The easiest way to do this is to run the migration locally on your host and target the running MySQL container.

1. Clone the [habitat-aspnet-sample](https://github.com/habitat-sh/habitat-aspnet-sample) repository - `git clone https://github.com/habitat-sh/habitat-aspnet-sample`
1. `cd habitat-aspnet-sample`
1. Edit the `ConnectionStrings` in `appsetting.json` which sits in the root of the repository to point to the running database with the correct username and password:

```console
  "ConnectionStrings": {
    "DefaultConnection": "server=192.168.1.1;uid=hab;pwd=hab;port=3306;database=habitat_aspnet_sample;"
  },
```

Note that we changed `server` to point to our IP address and changed the `uid` and `pwd` to "hab" because that is our username and also our password. Thats right! Its all about security first with this bad boy database!

Now you may be wondering why we did not just leave the `server` set to `localhost`; after all, we published port 3306 in our `docker run` command. This is one of the differences with Windows Containers - they ["don't do loopback"](https://blog.sixeyed.com/published-ports-on-windows-containers-dont-do-loopback/). So you will need to use your machine IP address instead or the IP of the container.

Finally, get the .Net Core SDK, restore the necessary .Net bits, and run the migration:

```console
hab pkg install core/dotnet-core-sdk
hab pkg exec core/dotnet-core-sdk dotnet restore
hab pkg exec core/dotnet-core-sdk dotnet ef database update
```

If all went well then your database container should have an empty but properly formatted schema that our application understands.

## Start the Web Application container

Let's now start our ASP.NET Core application container that will interact with our database. Note that we will need to use `docker inspect --format '{{ .NetworkSettings.Networks.nat.IPAddress }}' <container id>` to get the IP address of the database container to use for our `--peer` argument that allows our application to find the MySQL supervisor.

```console
docker run -p 8090:8090 -it --rm core/habitat-aspnet-sample --bind database:mysql.default --peer 172.24.223.87
```

We should see something like:

```console
hab-sup(MR): Supervisor Member-ID 35fab1419f7946b9ac10d91c145ee140
hab-sup(MR): Starting core/habitat-aspnet-sample
hab-sup(MR): Starting gossip-listener on 0.0.0.0:9638
hab-sup(MR): Starting http-gateway on 0.0.0.0:9631
habitat-aspnet-sample.default(SR): The specified service group 'mysql.default' for binding 'database' is not (yet?) present in the census data.
habitat-aspnet-sample.default(SR): Waiting for service binds...
habitat-aspnet-sample.default(HK): init, compiled to C:/hab/svc\habitat-aspnet-sample\hooks\init
habitat-aspnet-sample.default(HK): run, compiled to C:/hab/svc\habitat-aspnet-sample\hooks\run
habitat-aspnet-sample.default(HK): Hooks compiled
habitat-aspnet-sample.default(SR): Hooks recompiled
default(CF): Updated appsettings.json 99b0ce83ce3742721936e40bf56ddd5fd8877df8aaeaf61789be3fa70fbc269a
habitat-aspnet-sample.default(SR): Configuration recompiled
habitat-aspnet-sample.default(SR): Initializing
habitat-aspnet-sample.default(SV): Starting service as user=containeradministrator, group=
habitat-aspnet-sample.default(O): Hosting environment: Production
habitat-aspnet-sample.default(O): Content root path: C:\hab\svc\habitat-aspnet-sample\var
habitat-aspnet-sample.default(O): Now listening on: http://*:8090
habitat-aspnet-sample.default(O): Application started. Press Ctrl+C to shut down.
```

Now let's see if we can get a HTML response from our containerized application:

```console
Invoke-WebRequest http://192.168.1.1:8090 -Method Head


StatusCode        : 200
StatusDescription : OK
Content           :
RawContent        : HTTP/1.1 200 OK
                    Content-Type: text/html; charset=utf-8
                    Date: Thu, 30 Nov 2017 00:21:20 GMT
                    Server: Kestrel


Forms             : {}
Headers           : {[Content-Type, text/html; charset=utf-8], [Date, Thu, 30 Nov 2017
                    00:21:20 GMT], [Server, Kestrel]}
Images            : {}
InputFields       : {}
Links             : {}
ParsedHtml        : mshtml.HTMLDocumentClass
RawContentLength  : 0
```

Of course you could use an actual browser to interact with this "killer app," but There we have it - our Windows Habitat package running in a container!
