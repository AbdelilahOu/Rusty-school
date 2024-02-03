dev: 
	cargo run
	
containerup:
	docker run --name school-manager-db -e POSTGRES_USER=root -e POSTGRES_PASSWORD=mysecretpassword -p 5432:5432 -d postgres:15

containerdown:
	docker stop school-manager-db
	docker rm --force school-manager-db

createdb: 
	docker exec -it school-manager-db createdb --username=root --owner=root school

dropdb:
	docker exec -it school-manager-db dropdb school

migrationsup:
	sea-orm-cli migrate up

migrationslatest:
	sea-orm-cli migrate down

migrationsdown: 
	sea-orm-cli migrate reset

migrationsfresh:
	sea-orm-cli migrate fresh

entity:
	sea-orm-cli generate entity -o entity/src --lib

server:
	cargo run

watch:
	cargo watch -x run

build:
	cargo build --verbose --workspace --release

.PHONY: watch build server createdb dropdb mup mdownlatest mdownall mdownfresh mdownrefresh entity
