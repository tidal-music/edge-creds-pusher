SHELL := bash

.PHONY: build
build:
	docker build -t edge-creds-pusher:latest -f lambda.Dockerfile .

.PHONY: publish
publish: build
	docker run -v '$(PWD):/target' --rm edge-creds-pusher cp /usr/src/myapp/function.zip /target

.PHONY: clean
clean:
	docker rmi edge-creds-pusher:latest
	rm -f function.zip
	rm -fr target
