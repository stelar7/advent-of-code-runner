export DOCKER_TAG="stelar7/advent-of-code-runner:1"

.PHONY:
	test\
	docker.test\
	docker.build\
	docker.push

docker.push:
	docker push $(DOCKER_TAG)

docker.build:
	docker build . --tag $(DOCKER_TAG)

# make docker.test YEAR={year} DAY={day}
docker.test:
	docker run -ti \
	--network none \
	--env YEAR=$(YEAR) \
	--env DAY=$(DAY) \
	-v $(pwd):/test \
	$(DOCKER_TAG) \
	/bin/bash -c "cd /test && make test && exit"

test:
	mkdir -p leaderboards/$(YEAR)
	./$(YEAR)/$(DAY)/test.sh | awk '{print $$4, $$0}' | sort -n | cut -d' ' -f2- > ./leaderboards/$(YEAR)/$(DAY).txt