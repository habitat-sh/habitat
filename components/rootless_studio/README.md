# Rootless Docker Studio

## Building

`./build-docker-image.sh`

### Variables

* HAB_BLDR_CHANNEL (default: "stable")
* IMAGE_NAME (default: "habitat:default-studio")
* STUDIO_TYPE (default: "default")

## Running

`docker run -it --rm -v $PWD:/src -v ~/.hab/cache:/hab/cache -e HAB_ORIGIN=$HAB_ORIGIN habitat:default-studio enter`

OR

`docker run -it --rm -v $PWD:/src -v ~/.hab/cache:/hab/cache -e HAB_ORIGIN=$HAB_ORIGIN habitat:default-studio build <optional_path>`

** Note: Eventually this will be abstracted away by the habitat cli

## TODO:

Bring in the other studio types
