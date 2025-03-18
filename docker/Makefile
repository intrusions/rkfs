DOCKER_COMPOSE_FILE := docker-compose.yml
DOCKER_COMPOSE_DIR  := .
NAME                := linux_x86_64
SHELL               := /bin/bash
ENV_FILE            := .env
FLAGS               := -d
.DEFAULT_GOAL       := up

up:
	@docker compose                                     \
		-f $(DOCKER_COMPOSE_DIR)/$(DOCKER_COMPOSE_FILE) \
		--env-file $(DOCKER_COMPOSE_DIR)/.env           \
		up                                              \
		$(FLAGS)

down:
	@docker compose                                         \
		-f $(DOCKER_COMPOSE_DIR)/$(DOCKER_COMPOSE_FILE)     \
		down                                                \

shell:
	@docker exec -it $(NAME) $(SHELL)

re: down up

.PHONY: up down re shell