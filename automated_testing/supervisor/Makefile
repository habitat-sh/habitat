CHANNEL ?= stable
IMAGE_NAME ?= supervisor

image: Dockerfile
	docker build --build-arg CHANNEL=${CHANNEL} -t ${IMAGE_NAME} .
