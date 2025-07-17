CHANNEL ?= stable
IMAGE_NAME ?= supervisor

image: Dockerfile
	docker build --build-arg CHANNEL=${CHANNEL} --build-arg HAB_AUTH_TOKEN=${HAB_AUTH_TOKEN} -t ${IMAGE_NAME} .
