
DOCKER_IMAGE := abhiroopdatta7/guessing_game_rust
DOCKER_TAG := latest

.PHONY: play
play: build
	docker run --rm -it ${DOCKER_IMAGE}:${DOCKER_TAG}

.PHONY: build
build:
	docker build -t ${DOCKER_IMAGE}:${DOCKER_TAG} .

.PHONY: publish
publish:
	docker push ${DOCKER_IMAGE}:${DOCKER_TAG}

.PHONY: clean
clean:
	- @docker rmi ${DOCKER_IMAGE}:{DOCKER_TAG}