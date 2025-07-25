CHANNEL ?= stable
IMAGE_NAME ?= supervisor-testing-${CHANNEL}

habitat_integration_base: Dockerfile
	DOCKER_BUILDKIT=1 docker build --progress plain --build-arg CHANNEL=${CHANNEL} --secret id=hat,env=HAB_AUTH_TOKEN -t habitat_integration_base .

supervisor_image: supervisor/Dockerfile
	$(MAKE) -C supervisor CHANNEL=${CHANNEL} IMAGE_NAME=${IMAGE_NAME}

run: supervisor_image
	SUPERVISOR_IMAGE=${IMAGE_NAME} ./run_test_case.sh redis
