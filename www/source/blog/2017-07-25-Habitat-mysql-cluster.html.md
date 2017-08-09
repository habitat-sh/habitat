---
title: Making a Leader/Follower MySql Cluster with Habitat!
date: 2017-07-25
author: Nell Shamrell-Harrington
tags: supervisor, packaging
category: Supervisor
classes: body-article
---

One of the most compelling pieces of Habitat is using the supervisor to create self-organizing and self-healing topologies.  Today we will, step by step, create a MySQL Leader/Follower cluster using Habitat.

## Setting up your infrastructure

Create three virtual machines on the cloud provider of your choice (when creating this post, I used AWS EC2).

Why virtual machines and not containers?  You certainly could create a cluster in containers, but I strongly prefer to use containers only for stateless portions of an application.  A database is stateful part of an application and should be in a long lasting form of infrastructure, rather than an ephemeral one.  Containers give you isolation, immutability, and the ability to schedule workloads.  Databases are not schedulable, but by using Habitat you can still get isolation and immutability for your database software packages.  Habitat gives your stateful portions of your application two of the three super powers of containers.

When you set up your virtual machines, make sure that these ports are open.

* 22 (ssh)
* 9631 (tcp)
* 9638 (tcp)
* 9638 (udp)

When your virtual machines are up, ssh into each of them and run this command to install Habitat.

```console
  $ curl https://raw.githubusercontent.com/habitat-sh/habitat/master/components/hab/install.sh | sudo bash
```

Then create a hab group and user on each virtual machine.

```console
  $ sudo groupadd hab
  $ sudo useradd -g hab hab
```

## Installing and Running MySql

SSH into one of your virtual machines and run this command to download, install, and start MySql.

```console
  $ sudo hab start core/mysql --topology leader --group production
```

You will see a message indicating that an election is in progress, but a quorum has not yet been reached.  There need to be three supervisors in the ring for an election to take place, let's bring up the other two!

Now SSH into the other two virtual machines and run this command (substituting in the IP address for your first virtual machine).

```console
  $ sudo hab start core/mysql --topology leader --group production --peer first_vm_ip_address
```

If the election is successful, you will now see output like this:

```console
  mysql.production(SR): Executing hooks; 557399da9e9a4ac9b78b1ea33432c24a is the leader
```

This means that the supervisor with the ID of 557399da9e9a4ac9b78b1ea33432c24a is the leader, and the other two are followers.  The leader will receive write requests, and the follower will receive read requests.  But...how do we determine which virtual machine is running the supervisor with that cryptic ID?

## Checking out the Census

We can find this out by looking into the census of our ring.  You can find this information by heading to http://vm_ip_address:9631/census (it doesn't matter which VM's ip address you use, as long as it is in the ring).  Search for that address in the web page that comes up, then look for the IP address (on AWS, it is the private IP of the EC2 instance).

## We Need a New Leader!

Now, let's try stopping the leader.  SSH into the virtual machine which is running the supervisor functioning as the leader (see "Checking out the Census" above to find this information).  Now kill the running supervisor (I just use CTRL+c).

If you are watching the output of the two other virtual machines, you will see them elect a new leader, no human or orchestrator intervention required!

## Let's see it in action

Check out this video for a full screencast of setting up this cluster, electing a leader, stopping the leader, and electing a new leader.

<iframe width="560" height="315" src="https://www.youtube.com/embed/LKxElvaROFI" frameborder="0" allowfullscreen></iframe>
