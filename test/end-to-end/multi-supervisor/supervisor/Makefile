CHANNEL ?= stable
IMAGE_NAME ?= supervisor

image: Dockerfile
	@echo "Building supervisor image..."
	@echo "${HAB_AUTH_TOKEN}" > hab_auth_token.txt
	@DOCKER_BUILDKIT=1 docker build \
	  --build-arg CHANNEL=${CHANNEL} \
	  --secret id=hab_auth_token,src=hab_auth_token.txt \
	  -t ${IMAGE_NAME} .
	@rm hab_auth_token.txt
