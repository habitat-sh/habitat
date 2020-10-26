+++
title = "Google Container Registry (GCR)"
description = "Google Container Registry"

[menu]
  [menu.habitat]
    title = "Google Container Registry (GCR)"
    identifier = "habitat/containers/gcr-and-habitat"
    parent = "habitat/containers"

+++


[Google Container Registry](https://cloud.google.com/container-registry/) is a private Docker repository that
works with popular continuous delivery systems. It runs on GCP to provide consistent uptime on an infrastructure
protected by Google's security. The registry service hosts your private images in Cloud Storage under your GCP project.

Before you can push or pull images, you must configure Docker to use the gcloud command-line tool to authenticate
requests to Container Registry. To do so, run the following command (you are only required to do this once):

```bash
$ gcloud auth configure-docker
```

Further access control information is [available here](https://cloud.google.com/container-registry/docs/access-control).

After a successful Chef Habitat package build, images can be pushed to the Container Registry using the registry URI. The format of this
follows: `[HOSTNAME]/[PROJECT-ID]/[IMAGE]:[TAG]`, more details at [this link](https://cloud.google.com/container-registry/docs/pushing-and-pulling):

```bash
$ hab pkg export docker ./results/habskp-hab-gcr-demo-0.1.0-20180710145742-x86_64-linux.hart
$ docker tag habskp/hab-gcr-demo:latest eu.gcr.io/user-project/hab-gcr-demo:latest
$ docker push eu.gcr.io/user-project/hab-gcr-demo:latest
```

## Related Reading

* [Google Container Registry](https://cloud.google.com/container-registry/)
