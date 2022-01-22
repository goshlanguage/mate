SUPER_SECRET?=$(openssl rand -hex 16)
DATABASE_URL=postgresql://postgres:$(SUPER_SECRET)@localhost:5432/mate

.PHONY: dev-api-up
dev-api-up:
	docker run --rm --name mate-psql -d -p5432:5432 -e POSTGRES_PASSWORD=$(SUPER_SECRET) -e POSTGRES_DB=mate postgres:11.2
	sleep 3
	diesel migration run --database-url=$(DATABASE_URL)
	DATABASE_URL=$(DATABASE_URL) cargo run --bin mate-api -- -vv

.PHONY: dev-api-down
dev-api-down:
	docker rm -f mate-psql
