CHANNEL ?= stable
IMAGE_NAME ?= supervisor-testing-${CHANNEL}

habitat_integration_base: Dockerfile
	docker build --build-arg CHANNEL=${CHANNEL} -t habitat_integration_base .

supervisor_image: supervisor/Dockerfile
	$(MAKE) -C supervisor CHANNEL=${CHANNEL} IMAGE_NAME=${IMAGE_NAME}

run: supervisor_image
	SUPERVISOR_IMAGE=${IMAGE_NAME} ./run_test_case.sh redis
