Minimal Supervisor Image
########################

This `Dockerfile` defines a minimal Docker image containing only what
is needed to run a Supervisor. It does not contain any services, as
would happen with an image created using `hab pkg export docker`.

# Building

To build an image using the latest stable `core/hab`,
`core/hab-launcher`, and `core/hab-sup` packages, run:

```sh
make
```

To use packages from a non-`stable` channel, you must specify `CHANNEL`:

```sh
make CHANNEL=my_channel
```

or

```sh
CHANNEL=my_channel make
```

The image created is named `supervisor` by default. To change that,
specify `IMAGE_NAME`, similar to how you override `CHANNEL` above:

```sh
IMAGE_NAME=my_sup CHANNEL=unstable make
```
