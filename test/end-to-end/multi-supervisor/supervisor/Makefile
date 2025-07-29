CHANNEL ?= stable
IMAGE_NAME ?= supervisor

image: Dockerfile
	DOCKER_BUILDKIT=1 docker build --progress plain --build-arg CHANNEL=${CHANNEL} --secret id=hat,env=HAB_AUTH_TOKEN -t ${IMAGE_NAME} .
