
DOCKER_IMAGE := abhiroopdatta7/guessing_game_rust

.PHONY: play
play: build
	docker run --rm -it ${DOCKER_IMAGE}

.PHONY: build
build:
	docker build -t ${DOCKER_IMAGE} .

.PHONY: publish
publish:
	docker push ${DOCKER_IMAGE}

.PHONY: clean
clean:
	- @docker rmi ${DOCKER_IMAGE}