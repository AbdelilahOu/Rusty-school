dev: 
	cargo run
	
containerup:
	docker run --name school-manager-api -e POSTGRES_USER=root -e POSTGRES_PASSWORD=mysecretpassword -p 5432:5432 -d postgres:15

containerdown:
	docker stop school-manager-api
	docker rm --force school-manager-api

createdb: 
	docker exec -it school-manager-api createdb --username=root --owner=root school

dropdb:
	docker exec -it school-manager-api dropdb school

mup:
	sea-orm-cli migrate up

mdownlatest:
	sea-orm-cli migrate down

mdownall: 
	sea-orm-cli migrate reset

mdownfresh:
	sea-orm-cli migrate fresh

mdownrefresh:
	sea-orm-cli migrate refresh

entity:
	sea-orm-cli generate entity -o entity/src --lib

server:
	cargo run

build:
	cargo build --verbose --workspace --release

.PHONY: build server createdb dropdb mup mdownlatest mdownall mdownfresh mdownrefresh entity
