CHANNEL ?= stable
IMAGE_NAME ?= supervisor-testing-${CHANNEL}

habitat_integration_base: Dockerfile
	@echo "Building habitat_integration_base image..."
	@echo "${HAB_AUTH_TOKEN}" > hab_auth_token.txt
	@DOCKER_BUILDKIT=1 docker build \
	  --no-cache \
	  --build-arg CHANNEL=${CHANNEL} \
	  --secret id=hab_auth_token,src=hab_auth_token.txt \
	  -t habitat_integration_base .
	@rm hab_auth_token.txt

supervisor_image: supervisor/Dockerfile
	$(MAKE) -C supervisor \
	  CHANNEL=${CHANNEL} \
	  IMAGE_NAME=${IMAGE_NAME} \
	  HAB_AUTH_TOKEN=${HAB_AUTH_TOKEN}

run: supervisor_image
	SUPERVISOR_IMAGE=${IMAGE_NAME} ./run_test_case.sh redis
