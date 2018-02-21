=======
---
title: Habitat and Helm!
date: 2018-02-20
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

### Install all the tools (optimized for homebrew users): 

* Habitat (`brew install habitat`) (check with `hab --version` to make sure you're on 0.54.0) Or, if you prefer the `curl | bash`, try `curl https://raw.githubusercontent.com/habitat-sh/habitat/master/components/hab/install.sh | sudo bash`
* Minikube [https://kubernetes.io/docs/tasks/tools/install-minikube/](https://kubernetes.io/docs/tasks/tools/install-minikube/) 
* Kubectl (`brew install kubectl`) Alternate installation instructions available [here](https://kubernetes.io/docs/tasks/tools/install-kubectl/)
* Go (`brew install golang`) Alternate installation instructions available [here](https://golang.org/doc/install#install) 
* Helm (`brew install kubernetes-helm`) Additional information is available on their github repo at [https://github.com/kubernetes/helm](https://github.com/kubernetes/helm), and learn how to get started at [https://docs.helm.sh/using_helm/#quickstart-guide](https://docs.helm.sh/using_helm/#quickstart-guide)
* Tiller [https://docs.helm.sh/using_helm/#initialize-helm-and-install-tiller](https://docs.helm.sh/using_helm/#initialize-helm-and-install-tiller) (i.e. just type `helm init`) 
* Habitat Operator for Kubernetes: 

```
git clone https://github.com/kinvolk/habitat-operator.git
cd habitat-operator
make build
make image
```

This will produce a `kinvolk/habitat-operator` image, which can then be deployed to your cluster.
Read all about the Operator on Github at [https://github.com/kinvolk/habitat-operator/blob/master/README.md](https://github.com/kinvolk/habitat-operator/blob/master/README.md) 

**Important Note** 

Minikube uses its own Docker Engine, not your machine's Docker Engine. Habitat Studio uses your machine's Docker Engine. So, in order for your Minikube do be able to find the Helm package that you are about to build and export, you need to point your machine's Docker Engine to use Minikube's instead (once you've started your Minikube cluster). 

So, for the purposes of this Helm demo to work locally, you now need to do: 

* Be in the same directory as your compiled habitat-operator (i.e. habitat-operator)
* Start minikube with RBAC enabled: `minikube start --extra-config=apiserver.Authorization.Mode=RBAC` 
* Tell your machine's Docker daemon to use minikube's instead: `eval $(minikube docker-env)`

If you would prefer to publish your Helm charts to your Dockerhub and have Helm find them there to use on your Kubernetes cluster, that is supported! Enter the Habitat Studio (`hab studio enter`) and use `hab pkg export helm --help` to see more available options.

### Go to core-plans/nginx 

Habitat's core team provides a curated set of plans to help you build applications and services. You can clone that to your local laptop using `https://github.com/habitat-sh/core-plans.git` and then try out exporting one of these plans to Helm. I highly recommend you kick the tires with a plan that is reasonably fast to compile, which is why I chose `nginx` as the example here.

Once you've cloned core-plans locally, go to `core-plans/nginx` and enter the Habitat Studio. The Habitat Studio is a clean room designed for building software without picking up any external dependencies from your local machine.

* Enter the Habitat Studio within the nginx core plan with `hab studio enter`
* Build the latest nginx package and stores it as a habitat artifact (“hart”) in the nginx plan’s `results` directory: `build`
* Export the hab artifact to a Helm chart: `hab pkg export helm results/<yourfilename>.hart` This command will look very similar to `hab pkg export helm results/tdrew-nginx-1.11.10-20180221053714-x86_64-linux.hart`
* Exit the Habitat Studio: `exit`

You now have a Helm chart for core-plans/nginx in your core-plans/nginx/nginx-<version>-<datestamp>/ directory
I suggest you pwd for the above ^^ so you can use it in another tab when you have started your cluster and want to know where your helm chart lives

### Start a cluster and deploy your helm chart!

Here we are going to set up your minikube, and then deploy our helm chart onto that cluster. So, go back to your `habitat-operator` directory where you left your minikube cluster running and: 

* `kubectl create -f examples/rbac` This gives the Habitat Operator the permissions it needs on your RBAC-enabled cluster
* `kubectl apply -f examples/rbac/habitat-operator.yml` Deploys the Habitat Operator in your minikube cluster 
* `kubectl -n kube-system create sa tiller` Creates a service account for Tiller in the kube-system namespace
* `kubectl create clusterrolebinding tiller --clusterrole cluster-admin --serviceaccount=kube-system:tiller` Creates a ClusterRoleBinding for Tiller
* `helm init --service-account tiller` Installs Tiller, specifying new service account 
* `helm version` Check and make sure you have the right version of helm running on your client & server side, (as of printing, v 2.8.1) 
* `kubectl -n kube-system describe deploy/tiller-deploy` Check and see your service account has been correctly set up per instructions
* `helm dependency build <what your pwd was from building your Helm + Habitat package>` Reads your new nginx-<version>-<timestamp>/requirements.yaml file and sees the dependency on the habitat-operator. Pulls down the chart that describes the Habitat Operator and embeds it in your application’s chart 
* `helm install /Users/tdrew/core-plans/nginx/results/nginx-1.11.10-20180220023948/` (or whatever your pwd was -- if it's exactly this we need to talk.) Has Helm install your Habitat Helm package on your Minikube cluster

You will now see output similar to this: 

```

tdrew@remtdrew01:habitat-operator[master]$ helm install /Users/tdrew/core-plans/nginx/nginx-1.11.10-20180221053714/
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
honorary-wolf-habitat-operator-7f74859c6d-2jv9b  0/1    ContainerCreating  0         0s

==> v1/ServiceAccount
NAME                            SECRETS  AGE
honorary-wolf-habitat-operator  1        0s

```

Indicating you are successfully running your application built by Habitat, on Minikube, using Helm Charts and the Habitat Operator! 

⎈ Happy Habitat Helming! ⎈

