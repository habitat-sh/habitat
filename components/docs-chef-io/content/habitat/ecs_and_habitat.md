+++
title = "Amazon ECS and Chef Habitat"
description = "Amazon ECS and Chef Habitat"

[menu]
  [menu.habitat]
    title = "Amazon ECS and Chef Habitat"
    identifier = "habitat/containers/ecs-and-habitat"
    parent = "habitat/containers"

+++

Amazon Web Services provides a container management service called [EC2 Container Service (ECS)](https://aws.amazon.com/ecs/). ECS provides a Docker registry, container hosting and tooling to make deploying Docker-based containers fairly straightforward. ECS will schedule and deploy  your Docker containers within a Task while Chef Habitat manages the applications.

## EC2 Container Registry

[EC2 Container Registry (ECR)](https://aws.amazon.com/ecr/) is a fully-managed Docker registry provided by Amazon Web Services. Applications exported to Docker with ```hab pkg export docker``` put the containers into namespaced repositories, so you will need to create these within ECR. For example, if you were building ```core/mongodb``` containers you would use the following command:

```bash
$ aws ecr create-repository --repository-name core/mongodb
```

To tag and push the images to the ECR you will use your Repository URI (substituting your **aws_account_id** and availability zone).

```bash
$ docker tag core/mongodb:latest aws_account_id.dkr.ecr.ap-southeast-2.amazonaws.com/core/mongodb:latest
$ docker push aws_account_id.dkr.ecr.ap-southeast-2.amazonaws.com/core/mongodb:latest
```

## EC2 Compute Service

Once Docker images are pushed to ECR, they may be run on Amazon's ECS within a [Task Definition](http://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_defintions.html) which may be expressed as a [Docker Compose file](http://docs.aws.amazon.com/AmazonECS/latest/developerguide/cmd-ecs-cli-compose.html). Here is an example of a Tomcat application using a Mongo database demonstrating using Chef Habitat-managed containers:

```yaml docker-compose.yml
version: '2'
services:
  mongo:
    image: aws_account_id.dkr.ecr.ap-southeast-2.amazonaws.com/username/mongodb:latest
    hostname: "mongodb"
  national-parks:
    image: aws_account_id.dkr.ecr.ap-southeast-2.amazonaws.com/username/national-parks:latest
    ports:
      - "8080:8080"
    links:
      - mongo
    command: --peer mongodb --bind database:mongodb.default
```

From the example, the ```mongo``` and ```national-parks``` services use the Docker images from the ECR. The ```links``` entry manages the deployment order of the container and according to the [Docker Compose documentation](https://docs.docker.com/engine/userguide/networking/default_network/dockerlinks/#/updating-the-etchosts-file) ```links``` should create ```/etc/hosts``` entries. This does not appear to currently work with ECS so we assign the ```hostname: "mongodb"```.

The ```command``` entry for the National Parks Tomcat application allows the Chef Habitat Supervisor to ```--peer``` to the ```mongo``` gossip ring and ```--bind``` applies ```database``` entries to its Mongo configuration.

## Related Reading

* [A Journey with Chef Habitat on Amazon ECS, Part 1](https://blog.chef.io/a-journey-with-habitat-on-amazon-ecs-part-1/))
* [A Journey with Chef Habitat on Amazon ECS, Part 2](https://blog.chef.io/a-journey-with-habitat-on-amazon-ecs-part-2/)
