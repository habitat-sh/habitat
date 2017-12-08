---
title: Habitat, Rails, and Postgres in 3 Different Ways
date: 2017-08-18
author: Nell Shamrell-Harrington
tags: rails, postgresql, databases, rds, containers, docker
category: supervisor
classes: body-article
---

One of the best parts of managing software with Habitat is its flexibility.  We can use it to run a rails application in a container, virtual machine, or on bare metal.  We can also run that application's database on a container, virtual machine, bare metal, or even a cloud database service like Amazon RDS.  In this post we will:

* Create a simple Rails application
* Run the Rails application in one Docker container with its Postgresql database in another container
* Run the Rails application in a Docker container with its Postgresql database in an Amazon RDS instance
* Run the Rails application in a Docker container with its Postgresql database running as a cluster of 3 virtual machines

## Pre-requisites
* Rails installed on your workstation
* Habitat installed and set up on your workstation
* Text editor of your choice (in this post I use vim, but feel free to substitute your preferred editor)

## Creating your Rails application

Let's create an ultra simple Rails application.

```shell
$ rails new widget_world --database=postgresql
$ cd widget_world
$ vim Gemfile
```

Navigate to this line

```ruby ~/widget_world/Gemfile
gem 'tzinfo-data', platforms: [:mingw, :mswin, :x64_mingw, :jruby]
```

And remove the platforms so it looks like this

```ruby ~/widget_world/Gemfile
gem 'tzinfo-data'
```

Now run the following commands to install your gems, create a scaffold for Widgets, create and migrate the database, and then to run the application locally.

```shell ~/widget_world
$ bundle install
$ rails generate scaffold Widget name:string
$ rails db:create
$ rails db:migrate
$ rails server
```

Open up browser and head to "http://localhost:3000/widgets" and you should see your running application!  Go ahead and try creating new widgets, then viewing the list of them!

<iframe width="560" height="315" src="https://www.youtube.com/embed/8oz0-Q4bwQA" frameborder="0" allowfullscreen></iframe>

## Habiterizing the application

Now let's package up this application with Habitat!

We're going to use the Ruby scaffolding included with Habitat! To get this started, go ahead and run:

```shell ~/widget_world
$ hab plan init -s ruby
```

This will generate a Habitat directory, including a new plan.sh file.  Let's take a look at this file.

```shell ~/widget_world
$ vim habitat/plan.sh
```

```bash ~/widget_world/habitat/plan.sh
pkg_name=widget_world
pkg_origin=your_origin
pkg_scaffolding="core/scaffolding-ruby"
```

Now add this content

```bash ~/widget_world/habitat/plan.sh
pkg_name=widget_world
pkg_origin=your_origin
pkg_scaffolding="core/scaffolding-ruby"
pkg_version="0.1.0"
pkg_binds_optional=( [database]="port" )
```

Save and close the file. Now, we need to generate a secret key for the rails application.  Luckily for us, Rails makes this super easy.  Run:

```shell
$ rails secret
```

Copy the value generated

Now, we need to provide that value to our package. Open up the habitat/default.toml file:

```shell
$ vim habitat/default.toml
```

And add in this content:

```toml ~/widget_world/habitat/default.toml
secret_key_base = "secret_key_you_just_generated"

rails_env = 'production'

[db]
user = "admin"
password = "admin"
```

Note: The "admin" username and "admin" password are the defaults for the core/postgresql package.  We will cover how to change them later in this post.

Now, let's enter the Habitat Studio.

```shell
$ hab studio enter
```

Then run the `build` command

```studio
[1][default:/src:0]# build
```


## Running the application and database in Docker containers

Since we're running our rails application in a docker container, let's export our package as a docker image.

```studio
[2][default:/src:0]# hab pkg export docker ./results/<your-hart-package>.hart
```

And we are going to be running the postgresql database in another container using the [core/postgresql](https://bldr.habitat.sh/#/pkgs/core/postgresql) package.  Let's export that package as a docker image as well (Habitat will automatically download it for us from public Builder).

```studio
[3][default:/src:0]# hab pkg export docker core/postgresql
```

Now let's exit out of the Studio:

```studio
[4][default:/src:0]# exit
```

To bring up both containers locally, let's use Docker compose.  Go ahead and create a Docker compose file.

```shell ~/widget_world
$ vim docker-compose.yml
```

And add in this content:

```yaml ~/widget_world/docker-compose.yml
version: '3'
services:
  db:
    image: core/postgresql
    volumes:
      - "./habitat/default.toml:/default.toml"
  railsapp:
    image: your_origin/widget_world
    ports:
      - 8000:8000
    links:
    - db
    command: --peer db --bind database:postgresql.default
```

Save and close that file, now let's bring up those containers!

```shell ~/widget_world
$ docker-compose up
```

Whoops!  You will see this error:

```shell
railsapp_1  | widget_world.default hook[init]:(HK): There are 3 remaining config settings which must be set correctly:
railsapp_1  | widget_world.default hook[init]:(HK):
railsapp_1  | widget_world.default hook[init]:(HK):  * db.user      - The database username (Current: admin)
railsapp_1  | widget_world.default hook[init]:(HK):  * db.password  - The database password (Current: <set>)
railsapp_1  | widget_world.default hook[init]:(HK):  * db.name      - The database name (Current: widet_world_production)
railsapp_1  | widget_world.default hook[init]:(HK):
railsapp_1  | widget_world.default hook[init]:(HK): Aborting...
railsapp_1  | widget_world.default(HK): Initialization failed! 'init' exited with status code 15
```

To fix this, we need to set up the database.  Currently, the ruby scaffolding does not automatically set up the database for you, this still needs to be done manually.

```shell ~/widget_world
$ docker-compose exec railsapp hab pkg exec your-origin/widget_world widget_world-rake db:setup
```

And now we have a running Rails app and database running in Docker containers!  Head on over to https://localhost:8000/widgets to check it out!

This is a great and quickly satisfying demo...but it's not a great idea to run a database in a container.  Running a database in a container locks you into the host for that container, and you lose a good deal of the portability benefits of containers.  Let's keep running the Rails app in a container, but look at running the database for the app somewhere else.

Go ahead and shut down and remove both containers and let's try something different.

<iframe width="560" height="315" src="https://www.youtube.com/embed/gNNiRXNrcO0" frameborder="0" allowfullscreen></iframe>

## Using an Amazon RDS Database

Cloud database services like Amazon RDS are quick and easy to use.  Let's spin up an RDS database for our Rails app.

Although you can spin up an RDS instance using the Amazon cli, I often find it easiest to use the web GUI.

Go ahead and set up a basic postgres cluster using RDS.  Default values are fine, but make sure to note of what you set these two values as

* database username
* database password

RDS instances can sometimes take awhile to spin up, wait until it is ready before moving onto the next instructions.

Once the RDS instance is up, we need to make a few changes to the habitat/default.toml file in our widget_world repo.

Open it up:

```shell
$ vim habitat/default.toml
```

And modify it so that the db.user matches the username you set for your RDS instance and the db.password matches the password you set for your RDS instance.  Than add in on more attribute - db.hostname.  Set that to the endpoint for your RDS instance (you can get this from the AWS GUI or cli).  Do not include the port number in the endpoint. (i.e. it should be "https://my_endpoint" not "https://my_endpoint:5432")

```toml ~/widget_world/habitat/default.toml
secret_key_base = "secret_key_you_generated_earlier"

rails_env = "production"

[db]
user = "rds_instance_username"
password = "rds_instance_password"
host = "rds_endpoint_without_port"
```

Now, head back into the Studio.

```shell ~/widget_world
$ hab studio enter
```

Build your package, export that newly built package as a Docker image, then exit the Studio.

```studio
[1][default:/src:0]# build
[2][default:/src:0]# hab pkg export docker ./results/<your-hart-package>.hart
[3][default:/src:0]# exit
```

Now, let's run the rails application container.  We are NOT using docker-compose this time.

```shell ~/widget_world
$ docker run -it -p 8000:8000 your_origin/widget_world
```

Now, we once again need to setup the database in that container.  Find out the id of your docker container, one of the ways to do this is to run this command:

```shell ~/widget_world
$ docker ps
```

Now, execute the database setup on that container with:

```shell ~/widget_world
$ docker exec -it container_id hab pkg exec your-origin/widget_world widget_world-rake db:setup
```

Once this runs, head back to your browser, navigate to localhost:8000/widgets and check out your app - even though the app is in a container, it's database is an RDS instance.

When you are finished, go ahead and stop and destroy the Docker container running your application and, if you like, destroy the RDS instance.

<iframe width="560" height="315" src="https://www.youtube.com/embed/v9XheJWUwrk" frameborder="0" allowfullscreen></iframe>

## Using your own postgresql cluster

Now, let's pretend you want to run your own postgresql cluster.  You can also do this using the [core/postgresql](https://bldr.habitat.sh/#/pkgs/core/postgresql) package.

For this example, I use three AWS EC2 virtual machines for my cluster.

### Creating the security group

Before spinning up those VMs, however let's create a security group.  These examples use the AWS CLI, but feel free to use the web GUI if you prefer.

```shell ~/widget_world
$ aws ec2 create-security-group --group-name habitat-postgres-cluster --description "security group for a postgres cluster created and managed with Habitat"
```

Now, create the rules for that security group.

Habitat requires these ports to be open:

* 22 (ssh)
* 9631 (tcp)
* 9638 (tcp)
* 9638 (udp)

Additionally, since our app will be connecting to this database, we need this port open:

* 5432 (tcp)

```shell ~/widget_world
$ aws ec2 authorize-security-group-ingress --group-name habitat-postgres-cluster --protocol tcp --port 22 --cidr 0.0.0.0/0
$ aws ec2 authorize-security-group-ingress --group-name habitat-postgres-cluster --protocol tcp --port 9631 --cidr 0.0.0.0/0
$ aws ec2 authorize-security-group-ingress --group-name habitat-postgres-cluster --protocol tcp --port 9638 --cidr 0.0.0.0/0
$ aws ec2 authorize-security-group-ingress --group-name habitat-postgres-cluster --protocol udp --port 9638 --cidr 0.0.0.0/0
$ aws ec2 authorize-security-group-ingress --group-name habitat-postgres-cluster --protocol tcp --port 5432 --cidr 0.0.0.0/0
```

### Creating the EC2 instances

Now, let's create three virtual machines.  In the following examples, I am creating them in the AWS N. Virgina reason using an Ubuntu AMI.  If you create your virtual machines in a different region, you will need to substitute in the appropriate AMI.

```shell ~/widget_world
$ aws ec2 run-instances --image-id ami-cd0f5cb6 --security-group-ids your_security_group_id --count 3 --instance-type t2.medium --key-name your-key-name --query 'Instances[*].{ID:InstanceId,PublicIp:PublicIpAddress,PrivateIp:PrivateIpAddress'
```

The above command will return the instance ids, public ips, and private ips of each virtual machine.  Save this information somewhere you can refer to it, or save the query to run again whenever you need to.

Now, ssh into each of the three instances using the public IP.  On each instance, run these commands to install Habitat and set it up.

```shell "Run this on each instance"
$ curl https://raw.githubusercontent.com/habitat-sh/habitat/master/components/hab/install.sh | sudo bash
$ sudo groupadd hab
$ sudo useradd -g hab hab
```

### Setting up the postgresql cluster

Now, in one of these three instance, run this command to both install core/postgres and start a Supervisor ring

```shell "Run this on one of the three instances"
$ sudo hab start core/postgresql --topology leader --group production
```

Then, in each of the other two instances, run this command

```shell "Run this on the remaining two instances"
$ sudo hab start core/postgresql --topology leader --group production --peer first_instance_public_ip_address
```

This will peer them with the first instance and create a postgresql Supervisor ring.  When all three are up, you will see them hold an election and select a leader.

We need to know which instance is the leader, and the leader ID returned by Habitat is pretty cryptic.  Fortunately, there is a way to look it up.

Pick the public IP of one of those three instances (it doesn't matter which), then head into your browser and navigate to http://your_instance_public_ip:9631/census).

Once there, search for the id of the leader.  That will bring you to information about that particular Supervisor.  Look for the ip address - this is the private IP of the instance that is the leader.

Now, match that private IP to one of your instances.

And then copy the public IP of that instance.  Now let's get this information into our application package.

Open up your habitat/default.toml file and replace the value for the "host" key with that public ip address (it does need to be the public ip of the leader in order to work).

```toml ~/widget_world/habitat/default.toml
secret_key_base = "secret_key_you_generated_earlier"

rails_env = "production"

[db]
user = "admin"
password = "admin"
host = "leader_public_ip_address"
```

Now, enter back into Studio.

```shell ~/widget-world
$ hab studio enter
```

Build your package again and then export that new package into a docker image.

```studio
[1][default:/src:0]# build
[2][default:/src:0]# hab pkg export docker ./results/<your-hart-package>.hart
[3][default:/src:0]# exit
```

Now, let's run that container again.

```shell ~/widget-world
$ docker run -it -p 8000:8000 your_origin/widget_world
```

Now, we once again need to setup the database in that container.  Find out the id of your docker container, one of the ways to do this is to run this command:

```shell ~/widget-world
$ docker ps
```

Now, execute the database setup on that container with:

```shell ~/widget-world
$ docker exec -it container_id hab pkg exec your-origin/widget_world widget_world-rake db:setup
```

(This runs the setup and migrations on your postgresql cluster)

Once this runs, head back to your browser, navigate to localhost:8000/widgets and check out your app - now using your own postgresql database cluster!

<iframe width="560" height="315" src="https://www.youtube.com/embed/2uREwe4vFWE" frameborder="0" allowfullscreen></iframe>

## Conclusion

Again, one of the best parts of Habitat is it's flexibility - how it allows you to use the same packages on multiple types of infrastructure.  This allows you to choose the best infrastructure for you at this time, and provides plenty of room for changes throughout the lifecyle of your application.
