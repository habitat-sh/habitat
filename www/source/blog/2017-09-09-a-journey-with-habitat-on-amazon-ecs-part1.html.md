---
title: A journey with Habitat on Amazon ECS, part 1
date: 2017-09-09
author: Irving Popovetsky
tags: containers, Docker, AWS, ECS
category: Supervisor
classes: body-article
---

Amazon’s ECS (EC2 Container Service) provides a compelling offering to
developers and operators who are already very comfortable with AWS and its
tooling. In this series of posts I explore deployment strategies for
Containerized applications built using [Habitat](http://habitat.sh/) on ECS.

The mission: Taking an application stack from “works on my laptop” to “works in
production”.

*****

### Step 1, Picking an app

In my day job I work with a fairly complex microservices-based application,
[Chef Automate](https://docs.chef.io/chef_automate.html). Specifically the
containerized version you see published as the [Automate
Pilot](https://learn.chef.io/modules/chef-automate-pilot#/) — and meant for
non-production demo and trial purposes only. But once I took that demo apart and
understood what made it tick, I realized that it could be scaled up and out into
a robust Production-ready app.

A stretch goal for me personally is to continue the underlying technical work we
did for this whitepaper: [Scaling Chef Automate Beyond 50,000
nodes](https://pages.chef.io/rs/255-VFB-268/images/ScalingChefAutomate_2017.pdf)
but to see if we can reimagine the scaling solutions in a containerized version
of the app — while adding high availability and possibly even saving on cost.

### Step 2, Picking a Container Scheduler

ECS is a container scheduling system built on top of
[Docker](http://docker.com/) and AWS EC2. As somebody who uses AWS daily, I
found it pretty easy to understand how it worked and to integrate it into my
workflow.

* You define what you want to do with Task Definitions (which define which
containers you want to run) combined with ECS Services.
* These can be submitted directly via the AWS API, or more commonly via
CloudFormation.
* You can even take a regular docker-compose.yml and import it straight into ECS!
That lowers barrier slightly when transitioning from local workstation
development to ECS.

The reason that I chose ECS for this project is that it provides an elegant
solution to running data persistence services (Postgres, Elasticsearch, etc) in
the AWS RDS and Elasticsearch DBaaS, rather than trying to run those services in
containers. The magic that ties it all together is Cloudformation, which allows
you to provision persistent data services (RDS, etc) and your ECS cluster in a
single command, from a single infrastructure specification.

### Why Habitat on ECS?

![Habitat Architecture Overview](/images/infographics/habitat-architecture-overview.png)

Unlike some [competing](https://kubernetes.io/)
[solutions](https://mesosphere.com/product/), ECS doesn’t provide you niceties
like overlay networking and service discovery out of the box — which are really
important if you want to run your service on more than one docker host! For
service discovery there are many proposed solutions to solve for this that
involve setting up
[Lambdas](https://aws.amazon.com/blogs/compute/service-discovery-an-amazon-ecs-reference-architecture/),
[Consul](https://aws.amazon.com/blogs/compute/service-discovery-via-consul-with-amazon-ecs/),
[Netflix Eureka](https://github.com/Netflix/eureka), or [installing agents to
report to
DNS](https://aws.amazon.com/blogs/compute/service-discovery-via-consul-with-amazon-ecs/).
There are also 3rd offerings that provide both overlay networking and service
discovery from [Weave](https://www.weave.works/) and
[Linkerd](https://linkerd.io/) — of which I found the Weave offering compelling,
and could honestly eliminate the need for Habitat if it had config management
capabilities.

The compelling thing about Habitat is that it’s *the automation that travels
with the app* so I didn’t have to build external services and add dependencies
on those systems — which should simplify my deployment and reduce operational
complexity. It also helped that much of Chef’s internal development is shifting
from [Omnibus](https://blog.chef.io/2012/06/29/omnibus-chef-packaging/)-style
apps to Habitized apps, so a lot of the hard work had already been done on
Automate by awesome people like [Elliott](https://twitter.com/libsysguy) (hi!).

### What’s hard about Habitat on ECS?

ECS’s lack of service discovery makes it harder for Habitat supervisors to
initially find each other (the service discovery system needs service discovery
to bootstrap itself!) and the lack of overlay networking makes it harder for
supervisors to all have 2-way communication with each other.

That’s why the [Habitat docs section on
ECS](/docs/best-practices#ecs-and-habitat)
simply tells you to use docker `links` (a deprecated feature) like so:

```yaml
version: '2'
services:
  mongo:
    image: aws_account_id.dkr.ecr.ap-southeast-2.amazonaws.com/billmeyer/mongodb:latest
    hostname: "mongodb"
  national-parks:
    image: aws_account_id.dkr.ecr.ap-southeast-2.amazonaws.com/mattray/national-parks:latest
    ports:
      - "8080:8080"
    links:
      - mongo
    command: --peer mongodb --bind database:mongodb.default
```

What this does is leverage Docker (and ECS’s) *Bridge* mode networking, which is
the default networking mode and required in order for linking to work. In this
case the initial peer is the `mongodb`container which the `national-parks`
container can find because linking takes care of service discovery (the hostname
`mongodb` is resolvable from any other linked containers) as well as 2-way
communications (they are on the same bridged network, thus having unencumbered
communications with each other).

ECS has another networking mode (called [Host
mode](http://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_RegisterTaskDefinition.html#ECS-RegisterTaskDefinition-request-networkMode))
where each container doesn’t have its own IP address, and instead identifies as
the IP address of the host. Any port that the container tries to listen on
becomes a listening port on the host. More on that in a future post.

### A “simple” deployment

Let’s port Automate Pilot’s
[docker-compose.yml](https://chef-automate-artifacts.s3-us-west-2.amazonaws.com/stable/latest/pilot/docker-compose.yml)
file to ECS. I started with a fairly stock [ECS cloudformation
snippet](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/quickref-ecs.html)
and replaced the example service with the container definitions from the
docker-compose config. Here’s what that looks like:

What you see below is a complete and working Cloudformation template that
is made up of 3 important parts:

1.  Parameter definitions — asking you important questions like what VPC to deploy
into and server sizing (lines 2–35)
1.  The ECS Cluster — really the underlying EC2 instances that live in an
AutoScaling group, plus associated networking and security rules (lines 359–493)
1.  The ECS Service and Task definition — The part that once was our plucky
`docker-compose.yml` file, but now presented in a fashion that ECS can schedule,
run and even monitor (lines 52–284)

Let’s dive deeper into that ECS TaskDefition, because that deserves some
unpacking. There are 9 container instances, providing a range from data
persistence services (`postgresql`, `elasticsearch`), data processing services
(`logstash`, `rabbitmq`) and API services (`workflow-server`, `compliance`,
`notifications` and`automate-nginx`). There’s also a trick-container called
`postgresql-data` which lays down the initial Postgres data volume and then
exits — we have to set that to `Essential: false` so that ECS is okay with it
quitting.

Each container definition has some important components:

* The image name (which is pulled from
[dockerhub](https://hub.docker.com/u/chefdemo/))
* Tuning options (`Cpu`, `Memory` and `Ulimits`)
* Links (as mentioned above for service discovery and networking between
containers)
* Command Line arguments that are passed to Habitat: A peer name (to form a
[gossip ring](/docs/internals#supervisor-internals)) and bindings
(dependencies on a [service
group](/docs/using-habitat#service-groups))
* Environment variables passed to Habitat — TOML formatted parameters which inject
[configuration parameters
](/docs/using-habitat#config-updates)to be used at
runtime by this container (and also consumable by dependent services on other
containers)
* Logging configuration, which is handy because it makes all container output
available via CloudWatch Logs

Another important aspect of this configuration is the use of Volumes — right now
in a simple way to share the `postgresql-data` and `maintenance` volumes. More
on this in a future post, there’s a ton of interesting things we can do with it

**Know your Habiterms: The Initial Peer**

In this stack the initial peer is defined as the `postgresql` container — all of
the other containers must be linked to it in order to join the gossip ring and
enable and perform service discovery.

**Know your Habiterms: Service bindings**

Service bindings are a super important part of gluing services together in
Habitat. It says “Wait until the `postgresql.default` service group is available
before starting” — which makes a number of order-of-operations and other
orchestration problems magically disappear. It also allows you to explicitly
depend on other services, and know what ports to bind to or export to.

### Neat things about the underlying ECS cluster

One thing I really like about ECS is how it gives you full control over the
underlying ECS instances. They’re essentially EC2 instances with a simple
expectation: they run the ECS service and the cluster name is put in the
`/etc/ecs/ecs.config` file. That’s it! It means you could replace the stock
ECS-Optimized Amazon Linux images with CoreOS or Weave and it still works just
as well (but with cool features).

In an upcoming post I’ll show exactly what I do with that, but for now I’ll show
one very real problem I could solve easily: Elasticsearch 5 will refuse to start
unless you set a certain sysctl value like so: `sysctl -w
vm.max_map_count=262144`

That’s not a setting that can be set *inside* the container, it has to be set on
the host — and guaranteed consistent on every docker host in your cluster.
Cloudformation makes that easy, because I can set it in the `UserData` (boot
time commands) for each instance in the cluster.

### Up Next

In the next post, I’ll start to make this more robust by leveraging AWS RDS, ES
and EFS services so that our application can survive termination of the
containers or even the ECS instance!

```yaml
AWSTemplateFormatVersion: '2010-09-09'
Parameters:
  KeyName:
    Type: AWS::EC2::KeyPair::KeyName
    Description: Name of an existing EC2 KeyPair to enable SSH access to the ECS instances.
  VPC:
    Type: AWS::EC2::VPC::Id
    Description: Select a VPC that allows instances access to the Internet.
  SubnetIds:
    Type: List<AWS::EC2::Subnet::Id>
    Description: Select at least two subnets in your selected VPC.
  DesiredCapacity:
    Type: Number
    Default: '1'
    Description: Number of instances to launch in your ECS cluster.
  MaxSize:
    Type: Number
    Default: '2'
    Description: Maximum number of instances that can be launched in your ECS cluster.
  InstanceType:
    Description: EC2 instance type
    Type: String
    Default: m4.xlarge
    AllowedValues: [t2.micro, t2.small, t2.medium, t2.large, m3.medium, m3.large,
      m3.xlarge, m3.2xlarge, m4.large, m4.xlarge, m4.2xlarge, m4.4xlarge, m4.10xlarge,
      c4.large, c4.xlarge, c4.2xlarge, c4.4xlarge, c4.8xlarge, c3.large, c3.xlarge,
      c3.2xlarge, c3.4xlarge, c3.8xlarge, r3.large, r3.xlarge, r3.2xlarge, r3.4xlarge,
      r3.8xlarge, i2.xlarge, i2.2xlarge, i2.4xlarge, i2.8xlarge]
    ConstraintDescription: Please choose a valid instance type.
  ContactDept:
    Description: Contact department for billing purposes
    Type: String
  ContactEmail:
    Description: Contact email for Cloudwatch notifications and instance tagging
    Type: String

Mappings:
  AWSRegionToAMI:
    us-east-1:
      AMIID: ami-04351e12
    us-east-2:
      AMIID: ami-207b5a45
    us-west-1:
      AMIID: ami-7d664a1d
    us-west-2:
      AMIID: ami-57d9cd2e

Resources:
################################################################################
#  Combo Service - runs all of the containers in a single task definition (1 host), so linking can work
################################################################################
  AutomateService:
    Type: AWS::ECS::Service
    DependsOn: ALBListener
    Properties:
      Cluster: !Ref ECSCluster
      DesiredCount: 1
      LoadBalancers:
      - ContainerName: automate-nginx
        ContainerPort: 80
        TargetGroupArn: !Ref ECSTG
      Role: !Ref ECSServiceRole
      TaskDefinition: !Ref AutomateTask

  AutomateTask:
    Type: AWS::ECS::TaskDefinition
    Properties:
      Family: !Sub ${AWS::StackName}-automate
      ContainerDefinitions:
      - Name: postgresql-data
        Cpu: '10'
        Essential: 'false'
        Image: chefdemo/postgresql-data:stable
        Memory: 300
        LogConfiguration:
          LogDriver: awslogs
          Options:
            awslogs-group: !Ref 'CloudwatchLogsGroup'
            awslogs-region: !Ref 'AWS::Region'
            awslogs-stream-prefix: !Sub ${AWS::StackName}
        MountPoints:
        - ContainerPath: /hab/svc/postgresql/data
          SourceVolume: postgresql-data
      - Name: postgresql
        Hostname: postgresql
        Cpu: 10
        Essential: 'true'
        Image: chefdemo/postgresql:stable
        Memory: 300
        Environment:
        - Name: HAB_POSTGRESQL
          Value: |
            [superuser]
            name = 'hab'
            password = 'chefrocks'
        LogConfiguration:
          LogDriver: awslogs
          Options:
            awslogs-group: !Ref 'CloudwatchLogsGroup'
            awslogs-region: !Ref 'AWS::Region'
            awslogs-stream-prefix: !Sub ${AWS::StackName}
        MountPoints:
        - ContainerPath: /hab/svc/postgresql/data
          SourceVolume: postgresql-data
        VolumesFrom:
        - SourceContainer: postgresql-data
          ReadOnly: false
      - Name: rabbitmq
        Hostname: rabbitmq
        Cpu: 10
        Essential: 'true'
        Image: chefdemo/rabbitmq:stable
        Memory: 512
        Links:
        - postgresql
        Command:
        - --peer
        - postgresql
        Environment:
        - Name: HAB_RABBITMQ
          Value: |
            [rabbitmq]
            default_vhost = '/insights'
            default_user = 'insights'
            default_pass = 'chefrocks'
            [rabbitmq.management]
            enabled = true
        LogConfiguration:
          LogDriver: awslogs
          Options:
            awslogs-group: !Ref 'CloudwatchLogsGroup'
            awslogs-region: !Ref 'AWS::Region'
            awslogs-stream-prefix: !Sub ${AWS::StackName}
      - Name: elasticsearch
        Hostname: elasticsearch
        Cpu: 10
        Essential: 'true'
        Image: chefdemo/elasticsearch:stable
        Memory: 2048
        Links:
        - postgresql
        Command:
        - --peer
        - postgresql
        Ulimits:
        - Name: nofile
          HardLimit: 262144
          SoftLimit: 262144
        LogConfiguration:
          LogDriver: awslogs
          Options:
            awslogs-group: !Ref 'CloudwatchLogsGroup'
            awslogs-region: !Ref 'AWS::Region'
            awslogs-stream-prefix: !Sub ${AWS::StackName}
      - Name: logstash
        Cpu: 10
        Essential: 'true'
        Image: chefdemo/logstash:stable
        Memory: 2048
        Links:
        - rabbitmq
        - elasticsearch
        Command:
        - --peer
        - rabbitmq
        - --bind
        - elasticsearch:elasticsearch.default
        - --bind
        - rabbitmq:rabbitmq.default
        LogConfiguration:
          LogDriver: awslogs
          Options:
            awslogs-group: !Ref 'CloudwatchLogsGroup'
            awslogs-region: !Ref 'AWS::Region'
            awslogs-stream-prefix: !Sub ${AWS::StackName}
      - Name: workflow
        Hostname: workflow
        Cpu: 10
        Essential: 'true'
        Image: chefdemo/workflow-server:stable
        Memory: 768
        Links:
        - postgresql
        - rabbitmq
        - elasticsearch
        Command:
        - --peer
        - postgresql
        - --bind
        - database:postgresql.default
        - --bind
        - elasticsearch:elasticsearch.default
        - --bind
        - rabbitmq:rabbitmq.default
        MountPoints:
        - ContainerPath: /var/opt/delivery/delivery/etc
          SourceVolume: maintenance
        LogConfiguration:
          LogDriver: awslogs
          Options:
            awslogs-group: !Ref 'CloudwatchLogsGroup'
            awslogs-region: !Ref 'AWS::Region'
            awslogs-stream-prefix: !Sub ${AWS::StackName}
      - Name: notifications
        Hostname: notifications
        Cpu: 10
        Essential: 'true'
        Image: chefdemo/notifications:stable
        Memory: 768
        Links:
        - postgresql
        - rabbitmq
        - elasticsearch
        Command:
        - --peer
        - postgresql
        - --bind
        - database:postgresql.default
        - --bind
        - elasticsearch:elasticsearch.default
        - --bind
        - rabbitmq:rabbitmq.default
        LogConfiguration:
          LogDriver: awslogs
          Options:
            awslogs-group: !Ref 'CloudwatchLogsGroup'
            awslogs-region: !Ref 'AWS::Region'
            awslogs-stream-prefix: !Sub ${AWS::StackName}
      - Name: compliance
        Hostname: compliance
        Cpu: 10
        Essential: 'true'
        Image: chefdemo/compliance:stable
        Memory: 768
        Links:
        - postgresql
        Command:
        - --peer
        - postgresql
        - --bind
        - elasticsearch:elasticsearch.default
        LogConfiguration:
          LogDriver: awslogs
          Options:
            awslogs-group: !Ref 'CloudwatchLogsGroup'
            awslogs-region: !Ref 'AWS::Region'
            awslogs-stream-prefix: !Sub ${AWS::StackName}
      - Name: automate-nginx
        Hostname: automate-nginx
        Cpu: 10
        Essential: 'true'
        Image: chefdemo/automate-nginx:stable
        Memory: 300
        PortMappings:
        - ContainerPort: 80
        - ContainerPort: 443
        Links:
        - postgresql
        - rabbitmq
        - elasticsearch
        - compliance
        Command:
        - --peer
        - postgresql
        - --bind
        - elasticsearch:elasticsearch.default
        - --bind
        - workflow:workflow-server.default
        - --bind
        - compliance:compliance.default
        - --bind
        - notifications:notifications.default
        MountPoints:
        - ContainerPath: /var/opt/delivery/delivery/etc
          SourceVolume: maintenance
        LogConfiguration:
          LogDriver: awslogs
          Options:
            awslogs-group: !Ref 'CloudwatchLogsGroup'
            awslogs-region: !Ref 'AWS::Region'
            awslogs-stream-prefix: !Sub ${AWS::StackName}
      Volumes:
      - Name: postgresql-data
      - Name: maintenance

  ECSALB:
    Type: AWS::ElasticLoadBalancingV2::LoadBalancer
    Properties:
      Name: ECSALB
      Scheme: internet-facing
      LoadBalancerAttributes:
      - Key: idle_timeout.timeout_seconds
        Value: '30'
      Subnets: !Ref SubnetIds
      SecurityGroups: [!Ref 'EcsSecurityGroup']

  ALBListener:
    Type: AWS::ElasticLoadBalancingV2::Listener
    DependsOn: ECSServiceRole
    Properties:
      DefaultActions:
      - Type: forward
        TargetGroupArn: !Ref 'ECSTG'
      LoadBalancerArn: !Ref 'ECSALB'
      Port: '80'
      Protocol: HTTP

  ECSALBListenerRule:
    Type: AWS::ElasticLoadBalancingV2::ListenerRule
    DependsOn: ALBListener
    Properties:
      Actions:
      - Type: forward
        TargetGroupArn: !Ref 'ECSTG'
      Conditions:
      - Field: path-pattern
        Values: [/]
      ListenerArn: !Ref 'ALBListener'
      Priority: 1

  ECSTG:
    Type: AWS::ElasticLoadBalancingV2::TargetGroup
    DependsOn: ECSALB
    Properties:
      HealthCheckIntervalSeconds: 10
      HealthCheckPath: /viz/
      HealthCheckProtocol: HTTP
      HealthCheckTimeoutSeconds: 5
      HealthyThresholdCount: 2
      Name: ECSTG
      Port: 80
      Protocol: HTTP
      UnhealthyThresholdCount: 2
      VpcId: !Ref VPC

  ECSServiceRole:
    Type: AWS::IAM::Role
    Properties:
      AssumeRolePolicyDocument:
        Statement:
        - Effect: Allow
          Principal:
            Service: [ecs.amazonaws.com]
          Action: ['sts:AssumeRole']
      Path: /
      Policies:
      - PolicyName: ecs-service
        PolicyDocument:
          Statement:
          - Effect: Allow
            Action: ['elasticloadbalancing:DeregisterInstancesFromLoadBalancer', 'elasticloadbalancing:DeregisterTargets',
              'elasticloadbalancing:Describe*', 'elasticloadbalancing:RegisterInstancesWithLoadBalancer',
              'elasticloadbalancing:RegisterTargets', 'ec2:Describe*', 'ec2:AuthorizeSecurityGroupIngress']
            Resource: '*'

################################################################################
#  ECS Cluster - the EC2 instances (docker hosts) in an Autoscale group that make up the cluster
################################################################################
  ECSCluster:
    Type: AWS::ECS::Cluster

  ECSAutoScalingGroup:
    Type: AWS::AutoScaling::AutoScalingGroup
    Properties:
      VPCZoneIdentifier: !Ref SubnetIds
      LaunchConfigurationName: !Ref 'DockerHosts'
      MinSize: '1'
      MaxSize: !Ref 'MaxSize'
      DesiredCapacity: !Ref 'DesiredCapacity'
      Tags:
      - Key: Name
        Value: !Sub ${AWS::StackName}-dockerhost
        PropagateAtLaunch: true
      - Key: X-Dept
        Value: !Ref ContactDept
        PropagateAtLaunch: true
      - Key: X-Contact
        Value: !Ref ContactEmail
        PropagateAtLaunch: true
    CreationPolicy:
      ResourceSignal:
        Timeout: PT15M
    UpdatePolicy:
      AutoScalingReplacingUpdate:
        WillReplace: 'true'

  DockerHosts:
    Type: AWS::AutoScaling::LaunchConfiguration
    Properties:
      ImageId: !FindInMap [AWSRegionToAMI, !Ref 'AWS::Region', AMIID]
      SecurityGroups: [!Ref 'EcsSecurityGroup']
      InstanceType: !Ref 'InstanceType'
      IamInstanceProfile: !Ref 'EC2InstanceProfile'
      KeyName: !Ref 'KeyName'
      EbsOptimized: true
      BlockDeviceMappings:
      - DeviceName: /dev/xvda
        Ebs:
          VolumeSize: 200
          VolumeType: gp2
          DeleteOnTermination: true
      UserData:
        Fn::Base64: !Sub |
          #!/bin/bash -xe
          echo ECS_CLUSTER=${ECSCluster} >> /etc/ecs/ecs.config
          yum install -y aws-cfn-bootstrap
          # up the vm.max_map_count for elasticsearch
          sysctl -w vm.max_map_count=262144
          /opt/aws/bin/cfn-signal -e $? --stack ${AWS::StackName} --resource ECSAutoScalingGroup --region ${AWS::Region}

  EC2Role:
    Type: AWS::IAM::Role
    Properties:
      AssumeRolePolicyDocument:
        Statement:
        - Effect: Allow
          Principal:
            Service: [ec2.amazonaws.com]
          Action: ['sts:AssumeRole']
      Path: /
      Policies:
      - PolicyName: ecs-service
        PolicyDocument:
          Statement:
          - Effect: Allow
            Action: ['ecs:CreateCluster', 'ecs:DeregisterContainerInstance', 'ecs:DiscoverPollEndpoint',
              'ecs:Poll', 'ecs:RegisterContainerInstance', 'ecs:StartTelemetrySession',
              'ecs:Submit*', 'logs:CreateLogStream', 'logs:PutLogEvents']
            Resource: '*'

  AutoscalingRole:
    Type: AWS::IAM::Role
    Properties:
      AssumeRolePolicyDocument:
        Statement:
        - Effect: Allow
          Principal:
            Service: [application-autoscaling.amazonaws.com]
          Action: ['sts:AssumeRole']
      Path: /
      Policies:
      - PolicyName: service-autoscaling
        PolicyDocument:
          Statement:
          - Effect: Allow
            Action: ['application-autoscaling:*', 'cloudwatch:DescribeAlarms', 'cloudwatch:PutMetricAlarm',
              'ecs:DescribeServices', 'ecs:UpdateService']
            Resource: '*'

  EC2InstanceProfile:
    Type: AWS::IAM::InstanceProfile
    Properties:
      Path: /
      Roles: [!Ref 'EC2Role']

  EcsSecurityGroup:
    Type: AWS::EC2::SecurityGroup
    Properties:
      GroupDescription: ECS Security Group
      VpcId: !Ref VPC

  EcsSecurityGroupHTTPinbound:
    Type: AWS::EC2::SecurityGroupIngress
    Properties:
      GroupId: !Ref 'EcsSecurityGroup'
      IpProtocol: tcp
      FromPort: '80'
      ToPort: '80'
      CidrIp: 0.0.0.0/0

  EcsSecurityGroupSSHinbound:
    Type: AWS::EC2::SecurityGroupIngress
    Properties:
      GroupId: !Ref 'EcsSecurityGroup'
      IpProtocol: tcp
      FromPort: '22'
      ToPort: '22'
      CidrIp: 0.0.0.0/0

  EcsSecurityGroupALBports:
    Type: AWS::EC2::SecurityGroupIngress
    Properties:
      GroupId: !Ref 'EcsSecurityGroup'
      IpProtocol: tcp
      FromPort: '31000'
      ToPort: '61000'
      SourceSecurityGroupId: !Ref 'EcsSecurityGroup'

  CloudwatchLogsGroup:
    Type: AWS::Logs::LogGroup
    Properties:
      LogGroupName: !Join ['-', [ECSLogGroup, !Ref 'AWS::StackName']]
      RetentionInDays: 14
```
