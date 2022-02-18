# Bootstrap Studio

This studio is intended to replace the previous stage1 type studio based bootstrapping process.


We break all kinds of circular dependencies on previous versions of habitat and habitat 
packages. We leverage a minimal ubuntu docker image as our stage1 bootstrapping environment, instead of a chroot based environment.

## Building
```bash
# You must be inside the bootstrap-studio folder
./build-studio.sh
```
## Entering the Studio
```bash
HAB_DOCKER_STUDIO=stage1 HAB_DOCKER_STUDIO_IMAGE=bootstrap hab studio enter -D
```