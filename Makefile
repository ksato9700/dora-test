build-base-python:
	docker build -t dorabase-python -f docker/python/Dockerfile docker/python

build-python:
	docker build -t dora-python -f python/Dockerfile python

run-base-python:
	docker run -it --rm -v $(PWD)/python:/work --name dorabase-python dorabase-python

run-python:
	docker run -it --rm --name dora-python dora-python

watch:
	docker exec -it dorabase-python /bin/bash
