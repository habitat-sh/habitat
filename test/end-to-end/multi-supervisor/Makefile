CHANNEL ?= stable
IMAGE_NAME ?= supervisor-testing-${CHANNEL}

habitat_integration_base: Dockerfile
	docker build \
	  --build-arg CHANNEL=${CHANNEL} \
	  --build-arg HAB_AUTH_TOKEN=${HAB_AUTH_TOKEN} \
	  -t habitat_integration_base .

supervisor_image: supervisor/Dockerfile
	$(MAKE) -C supervisor \
	  CHANNEL=${CHANNEL} \
	  IMAGE_NAME=${IMAGE_NAME} \
	  HAB_AUTH_TOKEN=${HAB_AUTH_TOKEN}

run: supervisor_image
	SUPERVISOR_IMAGE=${IMAGE_NAME} ./run_test_case.sh redis
