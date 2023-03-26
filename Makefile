test:
		docker compose -f docker-compose.yaml run frontend bash -c "cd app/frontend && cargo test"
		docker compose -f docker-compose.yaml run backend bash -c "cd app/backend && cargo test"
dev:
		./startup.sh

up:
		docker compose -f docker-compose.yaml up
down:
		docker compose -f docker-compose.yaml down
build:
		docker compose -f docker-compose.yaml build

connect_to_db:
		docker compose -f docker-compose.yaml run database bash -c "psql -h localhost -p 5432 -d database -U postgres"
