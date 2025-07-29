CHANNEL ?= stable
IMAGE_NAME ?= supervisor

image: Dockerfile
	docker build --secret id=hab_auth_token,src=../hab_auth_token.txt --build-arg CHANNEL=${CHANNEL} -t ${IMAGE_NAME} .
