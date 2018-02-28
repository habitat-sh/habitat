---
title: Habitat and Helm!
date: 2018-02-21
author: Tasha Drew
tags: helm, exporter, kubernetes
category: update
classes: body-article
---

As of Habitat 0.54.0, you can export your Habitat built applications into Helm! 

[Helm](https://helm.sh/) is an open source package manager for Kubernetes that is maintained by the [CNCF](https://www.cncf.io/). It allows you to define, install, and upgrade even very complex Kubernetes applications -- and you can run multiple versions of the same service within the cluster, which is very helpful when you’re managing different releases.

One of the main goals of Habitat is to enable you to build your application once, and run them anywhere you need to go. Adding Helm as a exporter for our Kubernetes users is another great option to enable this workflow.

Read on for step by step instructions as to how to use them together.  

And now, let's get started! 

### Install all the tools (optimized for Homebrew users): 

* Habitat (`brew install habitat`) (check with `hab --version` to make sure you're on 0.54.0) Or, if you prefer the `curl | bash`, try `curl https://raw.githubusercontent.com/habitat-sh/habitat/master/components/hab/install.sh | sudo bash`
* Minikube [https://kubernetes.io/docs/tasks/tools/install-minikube/](https://kubernetes.io/docs/tasks/tools/install-minikube/) 
* Kubectl (`brew install kubectl`) Alternate installation instructions available [here](https://kubernetes.io/docs/tasks/tools/install-kubectl/)
* Helm (`brew install kubernetes-helm`) Additional information is available on their github repo at [https://github.com/kubernetes/helm](https://github.com/kubernetes/helm), and learn how to get started at [https://docs.helm.sh/using_helm/#quickstart-guide](https://docs.helm.sh/using_helm/#quickstart-guide)

You'll need a Kubernetes cluster to try this out on. If you're doing this on your desktop, you can either install [Minikube](https://kubernetes.io/docs/tasks/tools/install-minikube/) or try the [Docker CE Edge releases](https://docs.docker.com/edge/) that include Kubernetes built into Docker for the desktop.

**Important Note** 

If you use Minikube, bear in mind that it uses its own Docker Engine, not your machine's Docker Engine. Habitat Studio uses your machine's Docker Engine. So, in order for your Minikube to be able to find the Helm package that you are about to build and export, you need to point your machine's Docker Engine to use Minikube's instead (once you've started your Minikube cluster).

So, for the purposes of this Helm demo to work locally, you now need to do: 

* Start minikube with RBAC enabled: `minikube start --extra-config=apiserver.Authorization.Mode=RBAC` 
* Tell your machine's Docker daemon to use minikube's instead: `eval $(minikube docker-env)`

If you would prefer to publish your Helm charts to your Dockerhub and have Helm find them there to use on your Kubernetes cluster, that is supported! Please run `hab pkg export helm --help` to see more available options. (Please note - this command only works on Linux, so if you are not on a Linux machine, you will need to enter the Habitat Studio to run it successfully, using the comman `hab studio enter`). 

### Package Habitat's core/nginx as a Helm Chart

Habitat's core team provides a curated set of plans to help you build applications and services. I highly recommend you kick the tires with a precompiled package with an artifact that already exists on the Habitat Depot which is why I chose `nginx` as the example here.

* Enter the Habitat Studio (clean room environment) with `hab studio enter`
* Retrieve the `core/nginx` hab artifact and export it to a Helm chart:

`hab pkg export helm core/nginx`

* Exit the Habitat Studio: `exit`

You now have a Helm chart for core-plans/nginx in a directory named `nginx-latest`.

### Start a cluster and deploy your helm chart!

Here we are going to set up Helm in your Kubernetes cluster, and then deploy our Helm chart onto that cluster.

* `kubectl -n kube-system create sa tiller` Creates a service account for Tiller in the kube-system namespace
* `kubectl create clusterrolebinding tiller --clusterrole cluster-admin --serviceaccount=kube-system:tiller` Creates a ClusterRoleBinding for Tiller
* `helm init --service-account tiller` Installs Tiller, specifying new service account 
* `helm version` Check and make sure you have the right version of helm running on your client & server side, (as of printing, v 2.8.1) 
* `kubectl -n kube-system describe deploy/tiller-deploy` Check and see your service account has been correctly set up per instructions
* `helm repo add habitat-operator https://kinvolk.github.io/habitat-operator/helm/charts/stable/` Adds the Habitat Operator Helm repository to your Helm configuration
* `helm dependency update nginx-latest` Reads your new nginx-latest/requirements.yaml file and sees the dependency on the habitat-operator. Pulls down the chart that describes the Habitat Operator and embeds it in your application’s chart
* `helm install nginx-latest` Has Helm install your Habitat Helm package on your Minikube cluster

You will now see output similar to this: 

```

tdrew@remtdrew01:habitat-operator[master]$ helm install nginx-latest
NAME:   honorary-wolf
LAST DEPLOYED: Tue Feb 20 22:01:57 2018
NAMESPACE: default
STATUS: DEPLOYED

RESOURCES:
==> v1beta1/ClusterRole
NAME                            AGE
honorary-wolf-habitat-operator  0s

==> v1beta1/ClusterRoleBinding
NAME                            AGE
honorary-wolf-habitat-operator  0s

==> v1beta1/Deployment
NAME                            DESIRED  CURRENT  UP-TO-DATE  AVAILABLE  AGE
honorary-wolf-habitat-operator  1        1        1           0          0s

==> v1beta1/Habitat
NAME                          AGE
nginx-1.11.10-20180221053714  0s

==> v1/Pod(related)
NAME                                             READY  STATUS             RESTARTS  AGE
honorary-wolf-habitat-operator-7f74859c6d-2jv9b  1/1    Running            0         0s

==> v1/ServiceAccount
NAME                            SECRETS  AGE
honorary-wolf-habitat-operator  1        0s

```

Indicating you are successfully running your application built by Habitat, on Minikube, using Helm Charts and the Habitat Operator! 

⎈ Happy Habitat Helming! ⎈

